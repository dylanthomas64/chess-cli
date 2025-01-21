use std::str::FromStr;
mod board;

fn main() {
    let mut board = board::Board::startpos();
    println!("{}", board);

    let mv = board::Move::from_str("e2e4").unwrap();
    println!("{:?}", mv);
    board.process_move(mv).unwrap();
    println!("{}", board);
    println!("{:?}", board);
}
