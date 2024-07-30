//! This test checks that the macro can be used when the renaming may create multiple duplicate
//! fields.
#![allow(non_snake_case)]

use better_builder::BetterBuilder;

#[derive(Debug, BetterBuilder)]
struct Cart {
    num_wheels: u8,
    numwheels: Option<u8>,
    NuMWheels: u16,
    num_Wheels: u32,
    num_wheels_: u64,
}

fn main() {
    let t = Cart::builder()
        .num_wheels(4u8)
        .NuMWheels(8u16)
        .num_Wheels(16u32)
        .num_wheels_(32u64)
        .numwheels(Some(64u8))
        .build();

    assert_eq!(t.num_wheels, 4u8);
    assert_eq!(t.numwheels, Some(64u8));
    assert_eq!(t.NuMWheels, 8u16);
    assert_eq!(t.num_Wheels, 16u32);
    assert_eq!(t.num_wheels_, 32u64);
}
