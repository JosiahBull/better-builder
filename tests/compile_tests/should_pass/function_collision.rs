//! This test checks that the macro works even with function collisions.

use better_builder::BetterBuilder;

#[derive(Debug, BetterBuilder)]
struct Cart {
    owner: String,
    num_wheels: u8,
    num_seats: Option<u8>,
}

impl Cart {
    // Create a function that we expect to collide!
    fn builder() -> Self {
        Self {
            owner: "".to_string(),
            num_wheels: 0,
            num_seats: None,
        }
    }
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
