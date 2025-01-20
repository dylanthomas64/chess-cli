mod board;

fn main() {
    let board = board::Board::new("rnbqkbnr/pp1ppppp/8/2p5/4P3/5N2/PPPP1PPP/RNBQKB1R b KQkq - 1 2".to_string()).unwrap();
    println!("{}", board);
    let puzzle = board::Board::new("q3k1nr/1pp1nQpp/3p4/1P2p3/4P3/B1PP1b2/B5PP/5K2 b k - 0 17".to_string()).unwrap();
    println!("\n\n{}", puzzle);
    println!("\n{:?}", puzzle);
}
