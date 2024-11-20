use chess::Board;
use clap::Parser;

mod chess;

fn main() {
    //println!("♔ 	♕ 	♖ 	♗ 	♘ 	♙ 	♚ 	♛ 	♜ 	♝ 	♞ 	♟ ");
   let mut board = Board::new();
   Board::play(&mut board);
   
}



#[derive(Parser, Debug)]
struct Args {

}