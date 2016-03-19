extern crate colored;

mod card;

use card::*;

fn main() {
    println!("Hello, world!");
    
    let deck = Deck::new();
    println!("{}", deck);
}
