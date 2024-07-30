//! This test checks that the macro works when all fields are required.

use better_builder::BetterBuilder;

#[derive(BetterBuilder)]
struct Cart {
    owner: String,
    num_wheels: u8,
    num_seats: u8,
}

fn main() {
    let t = Cart::builder()
        .owner("Alice".to_string())
        .num_wheels(4)
        .num_seats(2)
        .build();

    assert_eq!(t.owner, "Alice".to_string());
    assert_eq!(t.num_wheels, 4u8);
    assert_eq!(t.num_seats, 2u8);
}
