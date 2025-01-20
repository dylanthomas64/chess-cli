mod board;

fn main() {
    let board = board::Board::new("rnbqkbnr/pp1ppppp/8/2p5/4P3/5N2/PPPP1PPP/RNBQKB1R b KQkq - 1 2".to_string()).unwrap();
    println!("{}", board);
}