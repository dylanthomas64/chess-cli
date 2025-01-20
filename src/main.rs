
use std::io;
use std::io::Write;
mod board;
use crate::board::Board;

fn main() {
    let board = Board::default();
    println!("{}", board);
}