use proc_macro::TokenStream;
use quote::quote;

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

#[proc_macro_derive(BetterBuilder)]
// TODO: address lints.
#[allow(clippy::too_many_lines, clippy::missing_panics_doc)]
pub fn derive_better_builder(item: TokenStream) -> TokenStream {
    let input = syn::parse_macro_input!(item as syn::DeriveInput);

    // Return an error if this is being derived on something other than a struct.
    let stuct_data = match input.data.clone() {
        syn::Data::Struct(data)
            if data
                .fields
                .iter()
                // Allow any, because if one field is named all must be.
                .any(|field| matches!(field, syn::Field { ident: Some(_), .. })) ||
                // Allow structs with no fields.
                data.fields.is_empty() =>
        {
            data
        }
        syn::Data::Struct(_) => {
            // SAFETY: This is a compile time error, which is not included in coverage. We have a test
            // for this specific case in: tests/compile_tests/should_fail/error_on_tuple_struct.rs
            return syn::Error::new_spanned(
                input,
                "BetterBuilder can only be derived on structs with named fields.",
            )
            .to_compile_error()
            .into();
        }
        _ => {
            // SAFETY: This is a compile time error, which is not included in coverage. We have a test
            // for this specific case in: tests/compile_tests/should_fail/error_on_enum.rs
            return syn::Error::new_spanned(input, "BetterBuilder can only be derived on structs.")
                .to_compile_error()
                .into();
        }
    };

    let struct_name = &input.ident;
    let visibility = &input.vis;

    // Generate an iterator over all fields, then partition it into two iterators. One for required
    // fields and one for optional fields. An optional fields has an Option<T>.
    let (required_fields, optional_fields): (Vec<_>, Vec<_>) =
        stuct_data.fields.into_iter().partition(|field| {
            // Check the first path segment ident is not Option.
            let is_optional = match &field.ty {
                syn::Type::Path(syn::TypePath { path, .. }) => path
                    .segments
                    .iter()
                    .next()
                    .is_some_and(|segment| segment.ident != "Option"),
                _ => false,
            };

            is_optional
        });

    let mut builder_structs = Vec::new();
    let mut fields_used_so_far: Vec<&syn::Field> = Vec::new();

    for (index, field) in required_fields.iter().enumerate() {
        let field_name = field.ident.as_ref().unwrap();
        let field_type = &field.ty;
        let builder_name = format!(
            "{}BuilderMissing{}",
            struct_name,
            convert_snake_case_to_upper_camel_case(field_name)
        );
        let builder_name = syn::Ident::new(&builder_name, field_name.span());

        let builder_fields = fields_used_so_far
            .iter()
            // .chain(std::iter::once(&field))
            .map(|field| {
                let field_name = field.ident.as_ref().unwrap();
                let field_type = &field.ty;
                quote! {
                    #field_name: #field_type,
                }
            });

        let next_field_name = required_fields
            .get(index + 1)
            .map(|f| f.ident.as_ref().unwrap());
        let next_builder_name = next_field_name.map_or_else(
            || {
                let builder_name = format!("{struct_name}Builder");
                syn::Ident::new(&builder_name, struct_name.span())
            },
            |next_field_name| {
                let builder_name = format!(
                    "{}BuilderMissing{}",
                    struct_name,
                    convert_snake_case_to_upper_camel_case(next_field_name)
                );
                syn::Ident::new(&builder_name, next_field_name.span())
            },
        );
        let optional_fields = if next_field_name.is_some() {
            quote! {}
        } else {
            // We are constructing the final builder, which will have many optional fields.
            // that need to be initalised to 'None'.
            let optional_fields = optional_fields.iter().map(|field| {
                let field_name = field.ident.as_ref().unwrap();
                quote! {
                    #field_name: None,
                }
            });

            quote! {
                #(#optional_fields)*
            }
        };

        let field_names_used_so_far = fields_used_so_far
            .iter()
            .map(|field| field.ident.as_ref().unwrap());

        // Generate builder struct
        let builder_struct = quote! {
            #visibility struct #builder_name {
                #(#builder_fields)*
            }
            impl #builder_name {
                pub fn #field_name(self, #field_name: #field_type) -> #next_builder_name {
                    #next_builder_name {
                        #field_name,
                        #(#field_names_used_so_far: self.#field_names_used_so_far,)*
                        #optional_fields
                    }
                }
            }
        };

        builder_structs.push(builder_struct);
        fields_used_so_far.push(field);
    }

    // Generate the actual builder struct, which should include methods for only the optional fields.
    let builder_fields = required_fields
        .iter()
        .chain(optional_fields.iter())
        .map(|field| {
            let field_name = field.ident.as_ref().unwrap();
            let field_type = &field.ty;
            quote! {
                #field_name: #field_type,
            }
        });

    let first_struct_name = required_fields
        .first()
        .map(|field| field.ident.as_ref().unwrap());
    let first_builder_name = first_struct_name.map_or_else(
        || {
            let builder_name = format!("{struct_name}Builder");
            syn::Ident::new(&builder_name, struct_name.span())
        },
        |first_struct_name| {
            let builder_name = format!(
                "{}BuilderMissing{}",
                struct_name,
                convert_snake_case_to_upper_camel_case(first_struct_name)
            );
            syn::Ident::new(&builder_name, first_struct_name.span())
        },
    );
    // If the first builder is the final builder, then we need to initialise all optional fields to
    // None.
    let first_builder_optional_fields = if first_struct_name.is_some() {
        quote! {}
    } else {
        let optional_fields = optional_fields.iter().map(|field| {
            let field_name = field.ident.as_ref().unwrap();
            quote! {
                #field_name: None,
            }
        });

        quote! {
            #(#optional_fields)*
        }
    };

    let builder_struct_name = format!("{struct_name}Builder");
    let builder_struct_name = syn::Ident::new(&builder_struct_name, struct_name.span());

    let optional_field_setter_methods = optional_fields.iter().map(|field| {
        let field_name = field.ident.as_ref().unwrap();
        // Field Type is Option<T> so we need to strip the Option.
        let field_type = &field.ty;
        quote! {
            pub fn #field_name(mut self, #field_name: #field_type) -> Self {
                self.#field_name = #field_name;
                self
            }
        }
    });

    let struct_constructor_all_fields = required_fields
        .iter()
        .map(|field| {
            let field_name = field.ident.as_ref().unwrap();
            quote! {
                #field_name: self.#field_name,
            }
        })
        .chain(optional_fields.iter().map(|field| {
            let field_name = field.ident.as_ref().unwrap();
            quote! {
                #field_name: self.#field_name,
            }
        }));

    let builder_struct = quote! {
        #visibility struct #builder_struct_name {
            #(#builder_fields)*
        }

        impl #builder_struct_name {
            #(#optional_field_setter_methods)*

            pub fn build(self) -> #struct_name {
                #struct_name {
                    #(#struct_constructor_all_fields)*
                }
            }
        }

        impl #struct_name {
            pub fn builder() -> #first_builder_name {
                #first_builder_name {
                    #first_builder_optional_fields
                }
            }
        }
    };

    builder_structs.push(builder_struct);

    // Combine all the generated code into a single TokenStream.
    let output = quote! {
        #(#builder_structs)*
    };

    output.into()
}
