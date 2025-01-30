use core::fmt;
use std::{collections::HashMap, fs, str::FromStr};
mod board;
mod coordinate;
mod errors;
mod move_logic;
mod pgn;
mod pieces;

use board::Board;
use errors::BoardError;
use move_logic::Move;
use pgn::{parse_pgn, pgn_to_san_moves, san_to_uci, PgnGame};

fn main() {
    let pgn_str = pgn();
    let pgn = parse_pgn(&pgn_str).unwrap();
    let san_moves = pgn_to_san_moves(pgn.moves.clone()).unwrap();
    let uci_moves = san_to_uci(san_moves).unwrap();



    let result = run(uci_moves);
    match result {
        Ok(_) => println!("finished!"),
        Err(e) => eprintln!("{}", e),
    }
    println!("exiting...")
}

fn run(uci_moves: Vec<Move>) -> Result<(), BoardError> {
    let mut board = Board::startpos();
    println!("{}", board);

    /* 
    let test_move_vec = vec![
        "e2e4", "d7d5", "e4d5", "e7e5", "d5e6", "a7a5", "e6e7", "a5a4", "e7d8q", "e8d8", "g1f3",
        "c8g4", "f1c4", "b8c6", "e1g1", "d8a8",
    ];*/

    for mv in uci_moves {
        board.process_uci_move(&mv)?;
        board.display_unicode();
    }
    println!("{:?}", board);
    Ok(())
}

fn pgn() -> String {
    let mut entry = fs::read_dir("pgn").expect("error opening pgn directory");
    let file_name = entry.next().unwrap().unwrap().path();
    fs::read_to_string(file_name).expect("error opening file")
}
