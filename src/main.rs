use std::str::FromStr;
mod board;

fn main() {
    let mut board = board::Board::new(
        "rnbqkbnr/pp1ppppp/8/2p5/4P3/5N2/PPPP1PPP/RNBQKB1R b KQkq - 1 2".to_string(),
    )
    .unwrap();
    println!("{}", board);

    let mv = board::Move::from_str("b8c6").unwrap();
    println!("{:?}", mv);
    board.process_move(mv).unwrap();
    println!("{}", board);
    println!("{:?}", board);
}
