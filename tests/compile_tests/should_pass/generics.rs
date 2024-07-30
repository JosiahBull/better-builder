//! This test checks that the macro works with various levels of generics.

use better_builder::BetterBuilder;

#[derive(Debug, BetterBuilder)]
struct Cart<T: AsRef<str>> {
    owner: T,
    num_wheels: u8,
    num_seats: Option<u8>,
}

fn main() {
    let t = Cart::builder()
        .owner("Alice".to_string())
        .num_wheels(4)
        .build();
    assert_eq!(t.owner, "Alice".to_string());
    assert_eq!(t.num_wheels, 4);
    assert_eq!(t.num_seats, None);
}
