extern crate rand;
extern crate colored;

mod card;
mod game;

use std::io;

use game::*;
use colored::*;


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
    println!("Move from <1> to <a> : 1-a");
    println!("Deal from main stack: h");
    println!("Exit: q");
    
    println!("\nyour move:");
}


// todo - process different inputs
// todo - validate moves
// todo - apply moves
fn process_move(input:String, game:&mut Game) -> bool{
    match &*input {
        "q\n"   => false,
        "h\n"   => {
            game.deal();
            true
        },
        _       => {
            println!("\n\n{}", "Invalid move!".red().bold());
            true
        }
    }
    
}