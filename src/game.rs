
use colored::*;
use card::*;

pub struct Game{
    deck:           Deck,
    tableau:        Vec<Stack>,
    target:         Vec<Stack>,
    open_tableau:   Vec<Stack>,
    stack:          Stack,
    round:          u16
}

impl Game{

    pub fn new() -> Game{
        let mut deck = Deck::new();
        deck.shuffle();
        
        let tableau = (0..7).map(|_| Stack::new()).collect();
        let target = (0..4).map(|_| Stack::new()).collect();
        let open_tableau = (0..7).map(|_| Stack::new()).collect();
        
        Game{
            deck:           deck,
            tableau:        tableau,
            open_tableau:   open_tableau,
            target:         target,
            stack:          Stack::new(),
            round:          0
        }
    }

    pub fn start(&mut self){
        println!("\n\t\t--------------------");
        println!("{}", "\t\tWelcome to Solitaire!".bold());
        println!("\t\t--------------------\n");
        
        
        // deal cards
        let tableau_len = self.tableau.len();
        let mut stacks_to_deal = tableau_len;
        
        while stacks_to_deal > 0 {
            for n in 1..(stacks_to_deal + 1){
                let card = self.deck.take(1);
                self.tableau[tableau_len - n].add_to_top(card);
            }
            stacks_to_deal = stacks_to_deal - 1;
        }
        
        
        for n in 0..(tableau_len){
            let card = self.tableau[n].take(1);
            self.open_tableau[n].add_to_top(card);
        }
        
        
        self.display_board();
    }
    
    pub fn display_board(&self){
        // deck
        let deck = "?".blue();
        
        // stack
        let stack = if self.stack.count() > 0 {
            if let Some(card) = self.stack.show(self.stack.count() - 1){
                card.to_string()
            }else{
                "  ".to_string()
            }
        } else {
            "".to_string()
        };
        
        
        // targets
        let target = &self.target.iter().map(|stack|
            if stack.count() > 0 {
                if let Some(card) = stack.show(stack.count() - 1){
                    card.to_string()
                }else{
                    " ".to_string()
                }
            } else {
                " ".to_string()
            }
        ).collect::<Vec<String>>();
        
        println!("\n\n{}: {}", "Moves".bold(), self.round);
        
        println!("\n\n<h>\t<s>\t\t\t<a>\t<b>\t<c>\t<d>");
        
        println!("[{}]\t{}\t\t\t[{}]\t[{}]\t[{}]\t[{}]", deck, stack, target[0], target[1], target[2], target[3]);
        
        let stacks = &self.tableau.iter().map(|stack|
            stack.count().to_string().blue()
        ).collect::<Vec<ColoredString>>();
        
        
        // stacks
      println!("\n\t<1>\t<2>\t<3>\t<4>\t<5>\t<6>\t<7>");
        println!("\t[{}]\t[{}]\t[{}]\t[{}]\t[{}]\t[{}]\t[{}]", stacks[0], stacks[1], stacks[2], stacks[3],
            stacks[4], stacks[5], stacks[6]);
            
            
        let stacks = &self.open_tableau.iter().map(|stack|
            if let Some(card) = stack.show(0){
                card.to_string()
            }else{
                "  ".to_string()
            }
        ).collect::<Vec<String>>();
        // cards
        println!("\t{}\t{}\t{}\t{}\t{}\t{}\t{}", stacks[0], stacks[1], stacks[2],
            stacks[3], stacks[4], stacks[5], stacks[6]);
            
            
        println!("\n");
    }
    
    pub fn deal(&mut self){
        self.round = self.round + 1;
        
        let card = self.deck.take(3);
        self.stack.add_to_top(card);
    }
}