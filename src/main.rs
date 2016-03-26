extern crate rand;
extern crate colored;

mod card;
mod game;

use std::io;

use game::*;


fn main() {
    
    let mut game = Game::new();
    game.start();
    
    while next_round(&mut game){
        game.display_board();
    }

}


fn next_round(game:&mut Game) -> bool{
    
    show_instructions();
    
    let mut input = String::new();

    match io::stdin().read_line(&mut input) {
        Ok(_) => process_move(input, game),
        Err(error) => {println!("error: {:?}", error); false}
    }
}

fn show_instructions() {
    println!("Move from <a> to <b> :");
    println!("a-b");
    println!("Deal from main stack:");
    println!("h");
    
    println!("\nyour move:");
}

fn process_move(input:String, game:&mut Game) -> bool{
    // match &*input
    true
}