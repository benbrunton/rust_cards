
use std::cmp;

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
            
        
        let mut longest_stack = 0;
        let mut stack_iter = 0;
        
        for stack in &self.open_tableau{
            longest_stack = cmp::max(stack.count(), longest_stack);
        }
        
        
        while stack_iter < longest_stack {
            let stacks = &self.open_tableau.iter().map(|stack|
                
                if let Some(card) = stack.show(stack_iter){
                    card.to_string()
                }else{
                    "  ".to_string()
                }
                
            ).collect::<Vec<String>>();
            // cards
            println!("\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t\t:{}", stacks[0], stacks[1], stacks[2],
                stacks[3], stacks[4], stacks[5], stacks[6], stack_iter);
                
            stack_iter = stack_iter + 1;
        }
        
        
            
            
        println!("\n");
    }
    
    pub fn deal(&mut self){
        self.round = self.round + 1;
        let take = cmp::min(3, self.deck.count()); 
        
        if take > 0 {
            let card = self.deck.take(take);
            self.stack.add_to_top(card);
        } else {
            let count = self.stack.count();
            let cards = self.stack.take(count);
            self.deck.add_to_top(cards);
        }
    }
    
    pub fn move_card(&mut self, source:&str, target:&str){
    
        // todo - move king to empty space
        // todo - move stacks of cards
        // todo - move suits up to targets
        
        println!("Checking move {} to {}", source, target);
        
        let empty_target_stack = Stack::new();
        let empty_source_stack = Stack::new();
        
        let mut target_stack: Stack = match self.get_target_stack(target){
            Some(target_stack)  => target_stack,
            None                => empty_target_stack
        };
        
        let mut source_stack: Stack = match self.get_source_stack(source){
            Some(source_stack)  => source_stack,
            None                => empty_source_stack
        };



        // todo - clean this horrible shit up
        if let Some(target_card) = target_stack.show(target_stack.count() - 1) {
        
            if let Some(previous_rank) = target_card.previous_rank() {
                
                if let Some(source_card) = source_stack.show(source_stack.count() - 1){
                    if source_card.colour == target_card.alternate_colour() && source_card.rank == previous_rank {
                        let card = source_stack.take(1);
                        target_stack.add_to_top(card);
                        
                        match &*target {
                            "a" => {self.target[0] = target_stack;},
                            "b" => {self.target[1] = target_stack;},
                            "c" => {self.target[2] = target_stack;},
                            "d" => {self.target[3] = target_stack;},
                            "1" => {self.open_tableau[0] = target_stack;},
                            "2" => {self.open_tableau[1] = target_stack;},
                            "3" => {self.open_tableau[2] = target_stack;},
                            "4" => {self.open_tableau[3] = target_stack;},
                            "5" => {self.open_tableau[4] = target_stack;},
                            "6" => {self.open_tableau[5] = target_stack;},
                            "7" => {self.open_tableau[6] = target_stack;},
                            _   => {}
                        }
                        
                        
                        match &*source {
                            "s" => {self.stack = source_stack;},
                            "1" => {self.open_tableau[0] = source_stack;},
                            "2" => {self.open_tableau[1] = source_stack;},
                            "3" => {self.open_tableau[2] = source_stack;},
                            "4" => {self.open_tableau[3] = source_stack;},
                            "5" => {self.open_tableau[4] = source_stack;},
                            "6" => {self.open_tableau[5] = source_stack;},
                            "7" => {self.open_tableau[6] = source_stack;},
                            _   => {}
                        }
                    }
                }
            }
        }

    }
    
    pub fn deal_stack(&mut self, source:&str){
        println!("Checking stack {}", source);
    }
    
    fn get_target_stack(&self, target:&str)->Option<Stack>{
        let open_tableau = &self.open_tableau;
        let target_ref = &self.target;
        match &*target {
            "a" => Some(target_ref[0].clone()),
            "b" => Some(target_ref[1].clone()),
            "c" => Some(target_ref[2].clone()),
            "d" => Some(target_ref[3].clone()),
            "1" => Some(open_tableau[0].clone()),
            "2" => Some(open_tableau[1].clone()),
            "3" => Some(open_tableau[2].clone()),
            "4" => Some(open_tableau[3].clone()),
            "5" => Some(open_tableau[4].clone()),
            "6" => Some(open_tableau[5].clone()),
            "7" => Some(open_tableau[6].clone()),
            _   => None
        }
        
    }
    
    fn get_source_stack(&self, source:&str)->Option<Stack>{
        
        let open_tableau = &self.open_tableau;
        
        match &*source {
            "s" => Some(self.stack.clone()),
            "1" => Some(open_tableau[0].clone()),
            "2" => Some(open_tableau[1].clone()),
            "3" => Some(open_tableau[2].clone()),
            "4" => Some(open_tableau[3].clone()),
            "5" => Some(open_tableau[4].clone()),
            "6" => Some(open_tableau[5].clone()),
            "7" => Some(open_tableau[6].clone()),
            _   => None
        }
    }
}