#![warn(clippy::pedantic, clippy::nursery, clippy::all)]

use std::{cell::RefCell, collections::HashMap};

use proc_macro::TokenStream;
use quote::quote;

type Result<T> = std::result::Result<T, syn::Error>;

fn convert_snake_case_to_upper_camel_case(ident: &syn::Ident) -> syn::Ident {
    let ident_str = ident.to_string();
    let mut camel_case = String::new();
    let mut capitalize_next = true;
    for c in ident_str.chars() {
        if c == '_' {
            capitalize_next = true;
        } else if capitalize_next {
            camel_case.push(c.to_ascii_uppercase());
            capitalize_next = false;
        } else {
            camel_case.push(c);
        }
    }

    syn::Ident::new(&camel_case, ident.span())
}

/// Data structure to store information about a field for later use in codegen.
struct FieldData<'a> {
    ident: &'a syn::Ident,
    ty: &'a syn::Type,
    builder_name_cache: RefCell<Option<syn::Ident>>,
}

impl<'a> FieldData<'a> {
    const fn new(ident: &'a syn::Ident, ty: &'a syn::Type) -> Self {
        Self {
            ident,
            ty,
            builder_name_cache: RefCell::new(None),
        }
    }

    /// Checks if the field is optional.
    ///
    /// Returns `true` if the first parent path segment is [`std::option::Option`].
    fn is_optional(&self) -> bool {
        match &self.ty {
            syn::Type::Path(syn::TypePath { path, .. }) => path
                .segments
                .iter()
                .next()
                .map_or(false, |segment| segment.ident == "Option"),
            _ => false,
        }
    }

    /// Generates a builder name for the field.
    ///
    /// The builder name is generated based on the parent struct name and the field name.
    /// If there are other builders with the same name, a suffix "Missing" followed by a number is
    /// added to the builder name.
    ///
    /// ## Arguments
    ///
    /// * `parent_struct_name` - The identifier of the parent struct.
    /// * `other_builders` - A mutable reference to a `HashMap` that stores other builders.
    ///
    /// ## Returns
    ///
    /// The generated builder name as a `syn::Ident`.
    fn generate_builder_name(
        &self,
        parent_struct_name: &syn::Ident,
        other_builders: &mut HashMap<String, u16>,
    ) -> syn::Ident {
        if let Some(builder_name) = &self.builder_name_cache.borrow().as_ref() {
            return (**builder_name).clone();
        }

        let orig_field_name = &self.ident;
        let field_name = convert_snake_case_to_upper_camel_case(orig_field_name);

        let mut builder_name = format!("{parent_struct_name}BuilderMissing{field_name}");
        let count = other_builders.entry(builder_name.clone()).or_insert(0);
        if *count > 0 {
            builder_name.push_str(&count.to_string());
        }
        *count = (*count)
            .checked_add(1)
            .expect("Overflow in builder name generation");

        let new_builder_name = syn::Ident::new(&builder_name, field_name.span());

        self.builder_name_cache
            .replace(Some(new_builder_name.clone()));

        new_builder_name
    }

    fn get_name_and_type(&self) -> proc_macro2::TokenStream {
        let field_name = self.ident;
        let field_type = self.ty;
        quote! {
            #field_name: #field_type,
        }
    }
}

impl<'a> TryFrom<&'a syn::Field> for FieldData<'a> {
    type Error = &'static str;

    fn try_from(field: &'a syn::Field) -> std::result::Result<Self, Self::Error> {
        let ident = field
            .ident
            .as_ref()
            .ok_or("Field must have an identifier")?;

        Ok(Self::new(ident, &field.ty))
    }
}

struct BetterBuilderGenerator<'a> {
    original_data: &'a syn::DeriveInput,
    fields: Vec<FieldData<'a>>,
}

impl<'a> BetterBuilderGenerator<'a> {
    pub fn new(original_data: &'a syn::DeriveInput) -> Result<Self> {
        let struct_data = match &original_data.data {
            syn::Data::Struct(data)
                if data
                    .fields
                    .iter()
                    // Allow any, because if one field is named all must be.
                    .any(|field| matches!(field, syn::Field { ident: Some(_), .. })) ||
                    // Allow structs with no fields.
                    data.fields.is_empty() =>
            {
                Ok(data)
            }
            syn::Data::Struct(_) => {
                // SAFETY: This is a compile time error, which is not included in coverage. We have a test
                // for this specific case in: tests/compile_tests/should_fail/error_on_tuple_struct.rs
                Err(syn::Error::new_spanned(
                    original_data,
                    "BetterBuilder can only be derived on structs with named fields.",
                ))
            }
            _ => {
                // SAFETY: This is a compile time error, which is not included in coverage. We have a test
                // for this specific case in: tests/compile_tests/should_fail/error_on_enum.rs
                Err(syn::Error::new_spanned(
                    original_data,
                    "BetterBuilder can only be derived on structs.",
                ))
            }
        }?;

        let mut fields = struct_data
            .fields
            .iter()
            .map(FieldData::try_from)
            .collect::<std::result::Result<Vec<_>, &'static str>>()
            .map_err(|_| syn::Error::new_spanned(original_data, "Field must have an identifier"))?;
        fields.sort_by_key(FieldData::is_optional);
        Ok(Self {
            original_data,
            fields,
        })
    }

    pub fn fields(&self) -> &[FieldData] {
        &self.fields
    }

    pub const fn visibility(&self) -> &syn::Visibility {
        &self.original_data.vis
    }

    pub const fn struct_name(&self) -> &syn::Ident {
        &self.original_data.ident
    }

    pub fn final_builder_name(&self) -> syn::Ident {
        let struct_name = self.struct_name();
        let builder_name = format!("{struct_name}Builder");
        syn::Ident::new(&builder_name, struct_name.span())
    }

    pub fn optional_names(&self) -> Vec<&syn::Ident> {
        self.fields
            .iter()
            .filter(|field| field.is_optional())
            .map(|field| field.ident)
            .collect()
    }

    pub fn generate_optional_setters(&self) -> Vec<proc_macro2::TokenStream> {
        let optional_fields = self.fields.iter().filter(|field| field.is_optional());
        optional_fields
            .map(|field| {
                let field_name = field.ident;
                let field_type = &field.ty;
                quote! {
                    pub fn #field_name(mut self, #field_name: #field_type) -> Self {
                        self.#field_name = #field_name;
                        self
                    }
                }
            })
            .collect()
    }

    pub fn generate_final_builder(&self) -> proc_macro2::TokenStream {
        let struct_name = self.struct_name();
        let builder_name = self.final_builder_name();
        let visibility = self.visibility();
        let struct_fields = self.fields.iter().map(FieldData::get_name_and_type);

        let setters = self.generate_optional_setters();

        let constructor_fields = self.fields.iter().map(|field| {
            let field_name = field.ident;
            quote! {
                #field_name: self.#field_name,
            }
        });

        let first_builder = {
            // If the first field is optional we need to initialise the final builder with all None
            // values.

            // Otherwise, just initialise the final builder with no fields.

            match self.fields.first() {
                Some(field) if !field.is_optional() => {
                    let first_builder_name =
                        field.generate_builder_name(struct_name, &mut HashMap::new());
                    quote! {
                        pub fn builder() -> #first_builder_name {
                            #first_builder_name {}
                        }
                    }
                }
                _ => {
                    let optional_fields = self.optional_names();
                    quote! {
                        pub fn builder() -> #builder_name {
                            #builder_name {
                                #(#optional_fields: None,)*
                            }
                        }
                    }
                }
            }
        };

        quote! {
            #visibility struct #builder_name {
                #(#struct_fields)*
            }

            impl #builder_name {
                #(#setters)*

                pub fn build(self) -> #struct_name {
                    #struct_name {
                        #(#constructor_fields)*
                    }
                }
            }

            impl #struct_name {
                #first_builder
            }
        }
    }
}

fn implementation_better_builder(input: &syn::DeriveInput) -> Result<TokenStream> {
    let struct_data = BetterBuilderGenerator::new(input)?;

    let mut other_builders = HashMap::new();
    let mut fields_used_so_far: Vec<&FieldData> = Vec::new();
    let mut culm_tokens = quote! {};

    for (index, field) in struct_data.fields().iter().enumerate() {
        if field.is_optional() {
            break;
        }

        let field_name = field.ident;
        let field_type = field.ty;
        let builder_name =
            field.generate_builder_name(struct_data.struct_name(), &mut other_builders);

        let struct_def_fields = fields_used_so_far
            .iter()
            .map(|a| FieldData::get_name_and_type(a))
            .fold(quote! {}, |acc, x| quote! { #acc #x });

        let builder_fields = fields_used_so_far.iter().map(|a| a.ident);
        let visibility = struct_data.visibility();

        let builder = match struct_data.fields().get(index + 1) {
            Some(next_field) if !next_field.is_optional() => {
                let next_builder_name =
                next_field.generate_builder_name(struct_data.struct_name(), &mut other_builders);

                quote! {
                    #visibility struct #builder_name {
                        #struct_def_fields
                    }

                    impl #builder_name {
                        pub fn #field_name(self, #field_name: #field_type) -> #next_builder_name {
                            #next_builder_name {
                                #field_name,
                                #(#builder_fields: self.#builder_fields,)*
                            }
                        }
                    }
                }
            },
            _ => {
                let final_builder_name = struct_data.final_builder_name();
                let optional_fields = struct_data.optional_names();
                quote! {
                    #visibility struct #builder_name {
                        #struct_def_fields
                    }

                    impl #builder_name {
                        pub fn #field_name(self, #field_name: #field_type) -> #final_builder_name {
                            #final_builder_name {
                                #field_name,
                                #(#builder_fields: self.#builder_fields,)*
                                #(#optional_fields: None,)*
                            }
                        }
                    }
                }
            }
        };

        culm_tokens.extend(builder);
        fields_used_so_far.push(field);
    }

    let final_builder = struct_data.generate_final_builder();
    let output = quote! {
        #culm_tokens
        #final_builder
    };

    Ok(output.into())
}

#[proc_macro_derive(BetterBuilder)]
pub fn derive_better_builder(item: TokenStream) -> TokenStream {
    let input = syn::parse_macro_input!(item as syn::DeriveInput);
    match implementation_better_builder(&input) {
        Ok(output) => output,
        Err(err) => err.to_compile_error().into(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_convert_snake_case_to_upper_camel_case() {
        let ident = syn::Ident::new("my_field", proc_macro2::Span::call_site());
        let camel_case = convert_snake_case_to_upper_camel_case(&ident);
        assert_eq!(camel_case.to_string(), "MyField");
    }

    #[test]
    fn test_field_data_is_optional() {
        let ident = syn::Ident::new("my_field", proc_macro2::Span::call_site());
        let ty = syn::parse_quote!(Option<i32>);

        let field = FieldData::new(&ident, &ty);
        assert!(field.is_optional());

        let ident = syn::Ident::new("my_field", proc_macro2::Span::call_site());
        let ty = syn::parse_quote!(i32);
        let field = FieldData::new(&ident, &ty);
        assert!(!field.is_optional());
    }

    #[test]
    fn test_field_data_generate_builder_name() {
        let ident = syn::Ident::new("my_field", proc_macro2::Span::call_site());
        let ty = syn::parse_quote!(i32);
        let field = FieldData::new(&ident, &ty);

        let ident = syn::Ident::new("my_Field", proc_macro2::Span::call_site());
        let ty = syn::parse_quote!(i32);
        let field2 = FieldData::new(&ident, &ty);

        let parent_struct_name = syn::Ident::new("MyStruct", proc_macro2::Span::call_site());

        let mut other_builders = HashMap::new();

        let builder_name = field.generate_builder_name(&parent_struct_name, &mut other_builders);
        assert_eq!(builder_name.to_string(), "MyStructBuilderMissingMyField");

        let builder_name = field2.generate_builder_name(&parent_struct_name, &mut other_builders);
        assert_eq!(builder_name.to_string(), "MyStructBuilderMissingMyField1");
    }

    #[test]
    fn test_field_data_generate_builder_idempotent() {
        let ident = syn::Ident::new("my_field", proc_macro2::Span::call_site());
        let ty = syn::parse_quote!(i32);

        let field = FieldData::new(&ident, &ty);

        let parent_struct_name = syn::Ident::new("MyStruct", proc_macro2::Span::call_site());

        let mut other_builders = HashMap::new();

        let builder_name = field.generate_builder_name(&parent_struct_name, &mut other_builders);
        assert_eq!(builder_name.to_string(), "MyStructBuilderMissingMyField");

        let builder_name = field.generate_builder_name(&parent_struct_name, &mut other_builders);
        assert_eq!(builder_name.to_string(), "MyStructBuilderMissingMyField");
    }
}
