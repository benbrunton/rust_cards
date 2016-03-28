
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
        
        
        // -- returning here if moving from empty stack
        if source_stack.count() < 1 { return; }



        // 1 - target is open_tableau then needs to be alternate colour and smaller card
        // OR - a King
        
        // 2 - target is a target stack so card needs to be same suit and higher card
        // OR an ace
        
        // index of target stack card
        let n = if target_stack.count() > 1 {
            target_stack.count() - 1
        } else {
            0
        };

        let do_move = match target.chars().next().unwrap() {
            'a' ... 'd'  => {
                match target_stack.show(n) {
                
                    Some(target_card) => {
                        let next_rank = target_card.next_rank().unwrap_or(Rank::King);
                            
                        if let Some(source_card) = source_stack.show(source_stack.count() - 1){
                            if source_card.suit == target_card.suit && source_card.rank == next_rank {
                                let card = source_stack.take(1);
                                target_stack.add_to_top(card);
                                true
                            }else {
                                false
                            }
                        }else{
                            false
                        }
                    },
                    
                    None => {
                        let source_card = source_stack.show(source_stack.count() - 1).unwrap_or(Card::new(Rank::King, Suit::Spades));
                        if source_card.rank == Rank::Ace {
                            let card = source_stack.take(1);
                            target_stack.add_to_top(card);
                            true
                        }else {
                            false
                        }
                    }
                }
            },
            '1' ... '7' => {
                
                match target_stack.show(n) {
        
                    Some(target_card) => {
            
                        let previous_rank = target_card.previous_rank().unwrap_or(Rank::Ace);
                            
                        
                        //
                        // todo - this is where we need to check for moving stacks of cards
                        //
                    
                        let mut n = 1;
                        let mut moved = false;
                        
                        while n <= source_stack.count() {
                            let source_card = source_stack.show(source_stack.count() - n).unwrap();
                            if source_card.colour == target_card.alternate_colour() && source_card.rank == previous_rank {
                                let mut card = source_stack.take(n);
                                card.reverse();
                                target_stack.add_to_top(card);
                                moved = true;
                                break;
                            }
                            n = n + 1;
                        }
                        
                        moved
                        
                        // if let Some(source_card) = source_stack.show(source_stack.count() - 1){
                        
                            
                            
                        //     if source_card.colour == target_card.alternate_colour() && source_card.rank == previous_rank {
                        //         let card = source_stack.take(1);
                        //         target_stack.add_to_top(card);
                        //         true
                        //     }else {
                        //         false
                        //     }
                            
                            
                        // }else{
                        //     false
                        // }
                        
                        //
                        // end todo
                        //
                    
                    },
                    None => { 
                        let source_card = source_stack.show(source_stack.count() - 1).unwrap_or(Card::new(Rank::Ace, Suit::Spades));
                        if source_card.rank == Rank::King {
                            let card = source_stack.take(1);
                            target_stack.add_to_top(card);
                            true
                        }else {
                            false
                        }
                    }
                }
            },
            _           => {false}
        };

        
        
        
        // replace new source and target with modified stacks
        if do_move {
        
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
            
            self.round = self.round + 1;
        }

    }
    
    pub fn deal_stack(&mut self, source:&str){
        println!("Checking stack {}", source);
        match self.get_source_stack(source) {
            Some(source_stack) => {
                let mut stack = source_stack.clone();
                if stack.count() == 0 {
                    let mut deck = self.get_source_deck(source).unwrap();
                    if deck.count() > 0 {
                        let card = deck.take(1);
                        stack.add_to_top(card);
                        
                        match &*source {
                            "1" => {self.open_tableau[0] = stack; self.tableau[0] = deck;},
                            "2" => {self.open_tableau[1] = stack; self.tableau[1] = deck;},
                            "3" => {self.open_tableau[2] = stack; self.tableau[2] = deck;},
                            "4" => {self.open_tableau[3] = stack; self.tableau[3] = deck;},
                            "5" => {self.open_tableau[4] = stack; self.tableau[4] = deck;},
                            "6" => {self.open_tableau[5] = stack; self.tableau[5] = deck;},
                            "7" => {self.open_tableau[6] = stack; self.tableau[6] = deck;},
                            _   => ()
                        }
                    }
                }
            },
            None => ()
        }
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
    
    // need better naming but this is the pile of hidden cards that we can
    // pick from
    fn get_source_deck(&self, source:&str)->Option<Stack>{
        let tableau = &self.tableau;
        
        match &*source {
            "1" => Some(tableau[0].clone()),
            "2" => Some(tableau[1].clone()),
            "3" => Some(tableau[2].clone()),
            "4" => Some(tableau[3].clone()),
            "5" => Some(tableau[4].clone()),
            "6" => Some(tableau[5].clone()),
            "7" => Some(tableau[6].clone()),
            _   => None
        }
    }
}