use std::fmt;
use colored::*;
use rand;
use rand::Rng;

#[derive(Clone, Debug, PartialEq)]
pub enum Suit{
    Diamonds,
    Clubs,
    Hearts,
    Spades
}

impl fmt::Display for Suit{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let icon = match self {
            &Suit::Diamonds    => "♦",
            &Suit::Clubs       => "♣",
            &Suit::Hearts      => "♥",
            &Suit::Spades      => "♠"
        };
        write!(f, "{}", icon)
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum Colour{
    Red,
    Black
}

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum Rank{
    Ace, Two, Three, Four, Five, Six, Seven, Eight, Nine, Ten, Jack, Queen, King
}

impl fmt::Display for Rank{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let display = match self {
            &Rank::Ace      => "A",
            &Rank::Two      => "2",
            &Rank::Three    => "3",
            &Rank::Four     => "4",
            &Rank::Five     => "5",
            &Rank::Six      => "6",
            &Rank::Seven    => "7",
            &Rank::Eight    => "8",
            &Rank::Nine     => "9",
            &Rank::Ten      => "10",
            &Rank::Jack     => "J",
            &Rank::Queen    => "Q",
            &Rank::King     => "K",
        };
        write!(f, "{}", display)
    }
}


#[derive(Clone, Debug, PartialEq)]
pub struct Card{
    pub rank: Rank,
    pub suit: Suit,
    pub colour: Colour
}

impl Card {

    pub fn new(rank: Rank, suit: Suit) -> Card {
        let colour = match suit {
            Suit::Diamonds  => Colour::Red,
            Suit::Hearts    => Colour::Red,
            _               => Colour::Black
        };
        Card{suit: suit, rank: rank, colour: colour}
    }
    
    pub fn previous_rank(&self) -> Option<Rank>{
        previous_rank(&self.rank)
    }
    
    pub fn alternate_colour(&self) -> Colour{
        if self.colour == Colour::Red {
            Colour::Black
        } else {
            Colour::Red
        }
    }
}

impl fmt::Display for Card {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let card = self.suit.to_string() + &self.rank.to_string();
        let decorated_card = if self.colour == Colour::Red {
            card.red()
        } else {
            card.black()
        };
        write!(f, "{}", decorated_card.bold().on_white())
    }
}


pub struct Deck(Vec<Card>);

impl Deck {
    pub fn new() -> Deck {
        let mut cards:Vec<Card> = Vec::with_capacity(52);
        for suit in &[Suit::Spades, Suit::Hearts, Suit::Diamonds, Suit::Clubs] {
            for rank in &[Rank::Ace, Rank::Two, Rank::Three, Rank::Four, 
                Rank::Five, Rank::Six, Rank::Seven, Rank::Eight, Rank::Nine, 
                Rank::Ten, Rank::Jack, Rank::Queen, Rank::King] {
                cards.push( Card::new(rank.clone(), suit.clone()) );
            }
        }
        Deck(cards)
    }
    
    pub fn deal(&mut self) -> Option<Card> {
        self.0.pop()
    }
 
    pub fn shuffle(&mut self) {
        let mut rng = rand::thread_rng();
        rng.shuffle(&mut self.0)
    }
    
    pub fn take(&mut self, n: usize) -> Vec<Card>{
    
        let mut temp_stack:Vec<Card> = Vec::new();
        while temp_stack.len() < n {
            if let Some(card) = self.deal(){
                temp_stack.push(card);
            }
        }
        
        temp_stack
    }
    
    pub fn count(&self) -> usize {
        self.0.len()
    }
    
    pub fn add_to_top(&mut self, cards: Vec<Card>){
        for card in cards.iter() {
            self.0.push(card.clone());
        }
    }
}
 
impl fmt::Display for Deck {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for card in self.0.iter() {
            writeln!(f, "{}", card);
        }
        write!(f, "")
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct Stack(Vec<Card>);

impl Stack {

    pub fn new() -> Stack{
        Stack(Vec::new())
    }

    pub fn count(&self) -> usize {
        self.0.len()
    }
    
    pub fn add_to_top(&mut self, cards: Vec<Card>){
        for card in cards.iter() {
            self.0.push(card.clone());
        }
    }
    
    pub fn show(&self, n: usize) -> Option<Card>{
        if self.0.len() <= n {
            None
        }else{
            Some(self.0[self.0.len() - 1].clone())
        }
    }
    
    pub fn take(&mut self, n: usize) -> Vec<Card>{
    
        let mut temp_stack:Vec<Card> = Vec::new();
        while temp_stack.len() < n {
            if let Some(card) = self.deal(){
                temp_stack.push(card);
            }
        }
        
        temp_stack
    }
    
    pub fn deal(&mut self) -> Option<Card> {
        self.0.pop()
    }
}

fn previous_rank(rank:&Rank) -> Option<Rank> {
    match rank {
        &Rank::Ace      => None,
        &Rank::Two      => Some(Rank::Ace),
        &Rank::Three    => Some(Rank::Two),
        &Rank::Four     => Some(Rank::Three),
        &Rank::Five     => Some(Rank::Four),
        &Rank::Six      => Some(Rank::Five),
        &Rank::Seven    => Some(Rank::Six),
        &Rank::Eight    => Some(Rank::Seven),
        &Rank::Nine     => Some(Rank::Eight),
        &Rank::Ten      => Some(Rank::Nine),
        &Rank::Jack     => Some(Rank::Ten),
        &Rank::Queen    => Some(Rank::Jack),
        &Rank::King     => Some(Rank::Queen)
    }
}
