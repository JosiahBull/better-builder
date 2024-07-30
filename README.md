# BetterBuilder

[![crates.io](https://img.shields.io/crates/v/better-builder?style=flat-square&logo=rust)](https://crates.io/crates/better-builder)
[![license](https://img.shields.io/badge/license-Apache--2.0_OR_MIT-blue?style=flat-square)](#license)
[![github actions](https://img.shields.io/github/actions/workflow/status/josiahbull/better-builder/ci.yaml?branch=main&style=flat-square&logo=github)](https://github.com/josiahbull/better-builder/actions)
[![docs.rs](https://img.shields.io/docsrs/better-builder?style=flat-square&logo=rust)](https://docs.rs/better-builder)
<!-- [![codecov](https://img.shields.io/codecov/c/github/josiahbull/better-builder?style=flat-square&logo=codecov)](https://codecov.io/gh/josiahbull/better-builder) -->

This rust crate aims to create better type-safe builders. Builders must fully provide all required and any optional fields before the end type can be instantiated.

tldr; No more `.build().unwrap()`, just `.build()` - and leverage the Rust type system to your advantage!

## Installation

**Cargo Add**

Run the command:

```shell
cargo add better_builder
```

**Cargo.toml**

```toml
better_builder = "0.1.0"
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

## Semantic Versioning and MSRV

This project follows semantic versioning. The minimum supported Rust version (MSRV) is `1.70.0`.

## License

Licensed under either of [Apache License, Version 2.0](LICENSE-APACHE) or
[MIT license](LICENSE-MIT) at your option.

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall
be dual licensed as above, without any additional terms or conditions.
