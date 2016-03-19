use std::fmt;
use colored::*;

#[derive(Clone, Debug, PartialEq, )]
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

#[derive(Clone, Debug, PartialEq)]
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
    rank: Rank,
    suit: Suit,
    colour: Colour
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
    // fn deal(&mut self) -> Option<Card> {
    //     self.0.pop()
    // }
 
    // fn shuffle(&mut self) {
    //     rand::thread_rng().shuffle(&mut self.0)
    // }
}
 
impl fmt::Display for Deck {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for card in self.0.iter() {
            writeln!(f, "{}", card);
        }
        write!(f, "")
    }
}
