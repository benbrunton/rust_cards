
use colored::*;
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
        
        let tableau = (0..7).map(|_| Stack::new()).collect();
        let target = (0..4).map(|_| Stack::new()).collect();
        
        Game{
            deck:       deck,
            tableau:    tableau,
            target:     target,
            stack:      Stack::new()
        }
    }

    pub fn start(&mut self){
        println!("\n\t--------------------");
        println!("\tWelcome to solitaire!");
        println!("\t--------------------\n");
        
        
        // deal cards
        let tableau_len = self.tableau.len();
        let mut stacks_to_deal = tableau_len;
        
        while stacks_to_deal > 0 {
            for n in 1..(stacks_to_deal + 1){
                println!("{}", n);
                let card = self.deck.take(1);
                self.tableau[tableau_len - n].add_to_top(card);
            }
            stacks_to_deal = stacks_to_deal - 1;
        }
        
        self.display_board();
        
        // let mut input = String::new();

    // match io::stdin().read_line(&mut input) {
    //     Ok(n) => {
    //         println!("{} bytes read", n);
    //         println!("{}", input);
    //     }
    //     Err(error) => println!("error: {}", error),
    // }
    }
    
    fn display_board(&self){
        // deck
        let deck = "[🂠]".blue();
        
        // stack
        let stack = if self.stack.count() > 0 {
            "1, 2, 3"
        } else {
            ""
        };
        
        
        // targets
        let target = &self.target.iter().map(|stack|
            if stack.count() > 0 {
                stack.top_card().to_string()
            } else {
                String::from("🂠")
            }
        ).collect::<Vec<String>>();
        
        println!("[h]\t[s]\t\t[a]\t[b]\t[c]\t[d]");
        
        println!("{}\t{}\t\t[{}]\t[{}]\t[{}]\t[{}]", deck, stack, target[0], target[1], target[2], target[3]);
        
        let stacks = &self.tableau.iter().map(|stack|
            if stack.count() > 0 {
                stack.count().to_string().blue()
            } else {
                "🂠".normal()
            }
        ).collect::<Vec<ColoredString>>();
        
        
        // stacks
      println!("\n\t[1]\t[2]\t[3]\t[4]\t[5]\t[6]\t[7]");
        println!("\t[{}]\t[{}]\t[{}]\t[{}]\t[{}]\t[{}]\t[{}]", stacks[0], stacks[1], stacks[2], stacks[3],
            stacks[4], stacks[5], stacks[6]);
    }
}