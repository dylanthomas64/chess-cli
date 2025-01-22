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

    let move_vec = vec!["e2e3"];
    for mv_str in move_vec {
        let mv = Move::from_str(mv_str)?;
        board.process_move(mv)?;
        println!("{}", board);
    }
    println!("{:?}", board);
    Ok(())
}
