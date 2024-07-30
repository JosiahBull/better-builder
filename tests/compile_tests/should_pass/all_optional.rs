//! This test checks that the macro works when all fields are optional.

use better_builder::BetterBuilder;

#[derive(Debug, BetterBuilder)]
struct Cart {
    owner: Option<String>,
    num_wheels: Option<u8>,
    num_seats: Option<u8>,
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

    let t = Cart::builder()
        .owner(Some("Alice".to_string()))
        .num_wheels(Some(4))
        .build();
    assert_eq!(t.owner, Some("Alice".to_string()));
    assert_eq!(t.num_wheels, Some(4));
    assert_eq!(t.num_seats, None);

    let t = Cart::builder()
        .num_seats(Some(2))
        .num_wheels(Some(4))
        .owner(Some("Alice".to_string()))
        .build();
    assert_eq!(t.owner, Some("Alice".to_string()));
    assert_eq!(t.num_wheels, Some(4));
    assert_eq!(t.num_seats, Some(2));

    let t = Cart::builder()
        .num_seats(Some(2))
        .num_wheels(Some(4))
        .num_wheels(Some(2))
        .owner(Some("Alice".to_string()))
        .build();
    assert_eq!(t.owner, Some("Alice".to_string()));
    assert_eq!(t.num_wheels, Some(2));
    assert_eq!(t.num_seats, Some(2));
}
