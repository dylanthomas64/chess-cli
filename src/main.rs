use chess::Board;
use clap::Parser;

mod chess;

fn main() {
    //println!("♔ 	♕ 	♖ 	♗ 	♘ 	♙ 	♚ 	♛ 	♜ 	♝ 	♞ 	♟ ");
   let board = Board::new();
   Board::play(&board);
   
}



#[derive(Parser, Debug)]
struct Args {
    //your chess move
}