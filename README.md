# BetterBuilder

This rust crate aims to create better type-safe builders. Builders must fully provide all required and any optional fields before the end type can be instantiated.

tldr; No more `.build()?`, just `.build()` - and leverage the Rust type system to your advantage!

## Installation

**Cargo Add**
Run the command:
```shell
cargo add better_builder
```

**Cargol Toml**

```toml
better_builder = "0.1"
```

## Example Usage

```rust
use better_builder::BetterBuilder;

#[derive(Debug, BetterBuilder)]
struct Cart {
    owner: String,
    num_wheels: u8,
    // Because this is an Option<T>, we assume it's not required to construct the object.
    num_seats: Option<u8>,
    inventory: Vec<String>,
}

fn main() {
    let t = Cart::builder()
        // All required properties, in order.
        .owner("Alice".to_string())
        .num_wheels(4)
        .inventory(vec!["apple".to_string(), "banana".to_string()])
        // Any optional properties.
        .num_seats(Some(2))
        // Finish building and get the final object.
        .build();

    assert_eq!(t.owner, "Alice".to_string());
    assert_eq!(t.num_wheels, 4);
    assert_eq!(t.num_seats, Some(2));
    assert_eq!(t.inventory, vec!["apple".to_string(), "banana".to_string()]);
}
```

# Licensing and Contributions

This project is licensed under the MIT License and ApacheV2 at your option.. You can find the full
text of these licenses in the `LICENSE-MIT` and `LICENSE-APACHE` files included in the repository.

Contributions to this project are welcomed and will also be licensed under the same terms. By
submitting a pull request or contributing in any other way, you agree to license your contributions
under the MIT License and Apache.

We value and appreciate all contributions to make this project better!
