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

    let mut usr_input = String::new();

    board.print();

    // main loop
    while usr_input != "q" {
        // white
        usr_input.clear();
        println!("White to move: ");

        io::stdout().flush().expect("Cannot flush stdout");
        io::stdin()
            .read_line(&mut usr_input)
            .expect("failed to read user input");
        usr_input = usr_input
            .strip_suffix("\r\n")
            .or(usr_input.strip_suffix("\n"))
            .unwrap_or(&usr_input)
            .to_string();
        Board::play_move(&mut board, &white, &usr_input);

        //black
        usr_input.clear();
        println!("Black to move: ");

        io::stdout().flush().expect("Cannot flush stdout");
        io::stdin()
            .read_line(&mut usr_input)
            .expect("failed to read user input");
        usr_input = usr_input
            .strip_suffix("\r\n")
            .or(usr_input.strip_suffix("\n"))
            .unwrap_or(&usr_input)
            .to_string();
        Board::play_move(&mut board, &black, &usr_input);
    }
    println!("Exiting program...");
}

#[derive(Parser, Debug)]
struct Args {}
