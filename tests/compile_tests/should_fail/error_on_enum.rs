use better_builder::BetterBuilder;

#[derive(BetterBuilder)]
enum Cart {
    Owner(String),
    NumWheels(u8),
    NumSeats(u8),
    Inventory(Vec<String>),
}

fn main() {}
