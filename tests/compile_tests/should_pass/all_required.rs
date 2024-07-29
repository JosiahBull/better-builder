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
        .owner(Some("Alice".to_string()))
        .num_wheels(Some(4))
        .num_seats(Some(2))
        .build();
    assert_eq!(t.owner, Some("Alice".to_string()));
    assert_eq!(t.num_wheels, Some(4));
    assert_eq!(t.num_seats, Some(2));
}
