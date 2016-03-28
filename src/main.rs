extern crate rand;
extern crate colored;
extern crate regex;

mod card;
mod game;

use std::io;

use game::*;
use colored::*;
use regex::Regex;


fn main() {
    
    let mut game = Game::new();
    game.start();
    
    while next_round(&mut game){
    
        if game.check_win() {
            show_win_message();
            
            game = Game::new();
        } else {
            game.display_board();
        }
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

fn show_win_message(){
    println!("\n\n\nYou completed the game!\n\nWell done!!!\n\n");
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
    let move_re = Regex::new(r"^(\w)-(\w)\n$").unwrap();
    let deal_re = Regex::new(r"^(\d)\n$").unwrap();

    match &*input {
        "q\n"   => false,
        "r\n"   => {
            main();
            false
        },
        "h\n"   => {
            game.deal();
            true
        },
        x if move_re.is_match(x) => {
            for cap in move_re.captures_iter(x) {
                game.move_card(cap.at(1).unwrap(), cap.at(2).unwrap());
            }
            true
        },
        x if deal_re.is_match(x) => {
            for cap in deal_re.captures_iter(x) {
                game.deal_stack(cap.at(1).unwrap());
            }
            true
        },
        x       => {
            println!("\n\n{}", "Invalid move!".red().bold());
            println!("{:?}", x);
            true
        }
    }
    
}