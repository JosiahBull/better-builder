//! This test checks a typical use case with several fields.

use better_builder::BetterBuilder;

#[derive(Debug, BetterBuilder)]
struct Cart {
    owner: String,
    num_wheels: u8,
    num_seats: Option<u8>,
    inventory: Vec<String>,
}

fn main() {
    let t = Cart::builder()
        .owner("Alice".to_string())
        .num_wheels(4)
        .inventory(vec!["apple".to_string(), "banana".to_string()])
        .num_seats(Some(2))
        .build();
    assert_eq!(t.owner, "Alice".to_string());
    assert_eq!(t.num_wheels, 4);
    assert_eq!(t.num_seats, Some(2));
    assert_eq!(t.inventory, vec!["apple".to_string(), "banana".to_string()]);

    let t = Cart::builder()
        .owner("Alice".to_string())
        .num_wheels(4)
        .inventory(vec!["apple".to_string(), "banana".to_string()])
        .build();
    assert_eq!(t.owner, "Alice".to_string());
    assert_eq!(t.num_wheels, 4);
    assert_eq!(t.num_seats, None);
    assert_eq!(t.inventory, vec!["apple".to_string(), "banana".to_string()]);
}
