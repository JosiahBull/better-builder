//! This test checks that the macro works when no fields are provided.

use better_builder::BetterBuilder;

#[derive(Debug, BetterBuilder)]
struct Cart {}

fn main() {
    let _t = Cart::builder().build();
}
