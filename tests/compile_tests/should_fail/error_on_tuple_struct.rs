// TODO: eventually this test case should pass.

use better_builder::BetterBuilder;

#[derive(BetterBuilder)]
struct Cart(
    String,
    u8,
    Option<u8>,
    Vec<String>,
);

fn main() {}
