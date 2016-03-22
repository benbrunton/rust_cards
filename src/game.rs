
use card::*;
use std::io;

pub struct Game{
    deck:       Deck,
    tableau:    Vec<Stack>,
    target:     Vec<Stack>,
    stack:      Stack
}

impl Game{

    pub fn new() -> Game{
        let mut deck = Deck::new();
        deck.shuffle();
        
        let tableau = (1..7).map(|_| Stack::new()).collect();
        let target = (1..4).map(|_| Stack::new()).collect();
        
        Game{
            deck:       deck,
            tableau:    tableau,
            target:     target,
            stack:      Stack::new()
        }
    }

    pub fn start(&self){
        println!("Welcome to solitaire!");
        
        // let mut input = String::new();

    // match io::stdin().read_line(&mut input) {
    //     Ok(n) => {
    //         println!("{} bytes read", n);
    //         println!("{}", input);
    //     }
    //     Err(error) => println!("error: {}", error),
    // }
    }
}