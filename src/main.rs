extern crate rand;
extern crate colored;

mod card;
mod game;

use game::*;


fn main() {
    
    let mut game = Game::new();
    
    game.start();

}
