use chess::{Board, PLAYER};
use clap::Parser;
use std::io;
use std::io::Write;

mod chess;

fn main() {
    //println!("♔ 	♕ 	♖ 	♗ 	♘ 	♙ 	♚ 	♛ 	♜ 	♝ 	♞ 	♟ ");
    let mut board = Board::new();
    let white = PLAYER::WHITE;
    let black = PLAYER::BLACK;

    /* loop {
                loop {get user white input -> if good play move -> if err continue, else break}
                loop {black}


    }

    */

    board.print();

    // main loop
    loop {
        // white turn
        if let Ok(usr_input) = get_usr_input(&white) {
            if let Err(_) = board.play_move(&white, &usr_input) {
                continue;
            }
        } else {
            continue;
        }

        // black turn
        if let Ok(usr_input) = get_usr_input(&black) {
            if let Err(_) = board.play_move(&black, &usr_input) {
                continue;
            }
        } else {
            continue;
        }
    }
    println!("Exiting program...");
}

fn get_usr_input(player: &PLAYER) -> Result<String, std::io::Error> {
    let mut usr_input = String::new();
    match player {
        PLAYER::WHITE => {
            println!("White to move: ");
        }
        PLAYER::BLACK => {
            println!("Black to move: ");
        }
    }

    usr_input.clear();
    io::stdout().flush().expect("Cannot flush stdout");
    io::stdin().read_line(&mut usr_input)?;

    usr_input = usr_input
        .strip_suffix("\r\n")
        .or(usr_input.strip_suffix("\n"))
        .unwrap_or(&usr_input)
        .to_string();
    Ok(usr_input)
}

#[derive(Parser, Debug)]
struct Args {}
