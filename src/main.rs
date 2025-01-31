use std::str::FromStr;
mod board;
mod coordinate;
mod errors;
mod move_logic;
mod pieces;

use board::Board;
use errors::BoardError;
use move_logic::Move;

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

    let move_vec = vec![
        "e2e4", "d7d5", "e4d5", "e7e5", "d5e6", "a7a5", "e6e7", "a5a4", "e7d8q", "a4a3", "d8d9",
    ];
    for mv_str in move_vec {
        let mv = Move::from_str(mv_str)?;
        board.process_move(&mv)?;
        board.display_unicode();
    }
    println!("{:?}", board);
    Ok(())
}
