use std::str::FromStr;
mod board;
mod errors;
mod piece;

use board::{Board, Move};
use errors::BoardError;

fn main() {
    let result = run();
    match result {
        Ok(_) => println!("finished!"),
        Err(e) => eprintln!("{}", e),
    }
    println!("exiting...")
}

fn run() -> Result<(), BoardError> {
    let mut board = Board::startpos();
    println!("{}", board);

    let move_vec = vec!["e2e4", "e7e5", "g1f3", "d7d6", "d2d4", "c8g4", "d4e5"];
    for mv_str in move_vec {
        let mv = Move::from_str(mv_str)?;
        board.process_move(mv)?;
        println!("{}", board);
    }
    println!("{:?}", board);
    Ok(())
}
