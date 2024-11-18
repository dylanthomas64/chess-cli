use std::vec::Vec;
use std::fmt;
fn main() {
    //println!("♔ 	♕ 	♖ 	♗ 	♘ 	♙ 	♚ 	♛ 	♜ 	♝ 	♞ 	♟ ");

    let board = Board::new();
    print_board(&board);


}

enum TYPE {
    PAWN,
    BISHOP,
    KNIGHT,
    ROOK,
    QUEEN,
    KING,
}

//piece defined by colour and type
enum PIECE {
    BLACK(TYPE),
    WHITE(TYPE),
}
impl fmt::Display for PIECE {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        
        let s = match &self {
            PIECE::WHITE(t) => match t {
                TYPE::PAWN => "♟",
                TYPE::BISHOP => "♝",
                TYPE::KNIGHT => "♞",
                TYPE::ROOK => "♜",
                TYPE::QUEEN => "♛",
                TYPE::KING => "♚",
            },
            PIECE::BLACK(t) => match t {
                TYPE::PAWN => "♙",
                TYPE::BISHOP => "♗",
                TYPE::KNIGHT => "♘",
                TYPE::ROOK => "♖",
                TYPE::QUEEN => "♕",
                TYPE::KING => "♔",
            },
        };
        write!(f, "{}", s)
    }
}


//2d vector representation of the board

struct Board {
    pieces: Vec<Vec<Option<PIECE>>>,
}

impl Board {
    fn new() -> Board {
        let mut pieces: Vec<Vec<Option<PIECE>>> = Vec::new();
        // generate inital board state
        // start pushing from the bottom (white first so indexing will translate easily to board coordinates)
       
        let mut row1: Vec<Option<PIECE>> = Vec::new();
        row1.push(Some(PIECE::WHITE(TYPE::ROOK)));
        row1.push(Some(PIECE::WHITE(TYPE::KNIGHT)));
        row1.push(Some(PIECE::WHITE(TYPE::BISHOP)));
        row1.push(Some(PIECE::WHITE(TYPE::QUEEN)));
        row1.push(Some(PIECE::WHITE(TYPE::KING)));
        row1.push(Some(PIECE::WHITE(TYPE::BISHOP)));
        row1.push(Some(PIECE::WHITE(TYPE::KNIGHT)));
        row1.push(Some(PIECE::WHITE(TYPE::ROOK)));
        pieces.push(row1);
        let mut row2: Vec<Option<PIECE>> = Vec::new();
        for _ in 0..8 {
            row2.push(Some(PIECE::WHITE(TYPE::PAWN)));
        }
        pieces.push(row2);


        for _ in 0..4 {
            let mut empty_row: Vec<Option<PIECE>> = Vec::new();
            for _ in 0..8 {
                empty_row.push(None);
            }
            pieces.push(empty_row);
        }
        println!("size of pieces: {}", pieces.len());
        let mut row7: Vec<Option<PIECE>> = Vec::new();
        for _ in 0..8 {
            row7.push(Some(PIECE::BLACK(TYPE::PAWN)));
        }
        pieces.push(row7);
        let mut row8: Vec<Option<PIECE>> = Vec::new();
        row8.push(Some(PIECE::BLACK(TYPE::ROOK)));
        row8.push(Some(PIECE::BLACK(TYPE::KNIGHT)));
        row8.push(Some(PIECE::BLACK(TYPE::BISHOP)));
        row8.push(Some(PIECE::BLACK(TYPE::QUEEN)));
        row8.push(Some(PIECE::BLACK(TYPE::KING)));
        row8.push(Some(PIECE::BLACK(TYPE::BISHOP)));
        row8.push(Some(PIECE::BLACK(TYPE::KNIGHT)));
        row8.push(Some(PIECE::BLACK(TYPE::ROOK)));
        pieces.push(row8);
        println!("size of pieces: {}", pieces.len());
        Board {pieces}
    }
}





fn print_board(board: &Board) {
    //unroll 2d vector into a 1d vector of strs
    println!("size: {}", board.pieces.len());
    let v: Vec<String> = board.pieces.iter().flatten()
    .map(|opt| match opt {
        Some(piece) => format!("{}", piece),
        None => " ".to_string(),
    }).collect();

    println!("size: {}", v.len());
    let row1 = format!("1 #  {}  #  {}  #  {}  #  {}  #  {}  #  {}  #  {}  #  {}  #", v[0], v[1], v[2], v[3], v[4], v[5], v[6], v[7]);
    let row2 = format!("2 #  {}  #  {}  #  {}  #  {}  #  {}  #  {}  #  {}  #  {}  #", v[8], v[9], v[10], v[11], v[12], v[13], v[14], v[15]);
    let row3 = format!("3 #  {}  #  {}  #  {}  #  {}  #  {}  #  {}  #  {}  #  {}  #", v[16], v[17], v[18], v[19], v[20], v[21], v[22], v[23]);
    let row4 = format!("4 #  {}  #  {}  #  {}  #  {}  #  {}  #  {}  #  {}  #  {}  #", v[24], v[25], v[26], v[27], v[28], v[29], v[30], v[31]);
    let row5 = format!("5 #  {}  #  {}  #  {}  #  {}  #  {}  #  {}  #  {}  #  {}  #", v[32], v[33], v[34], v[35], v[36], v[37], v[38], v[39]);
    let row6 = format!("6 #  {}  #  {}  #  {}  #  {}  #  {}  #  {}  #  {}  #  {}  #", v[40], v[41], v[42], v[43], v[44], v[45], v[46], v[47]);
    let row7 = format!("7 #  {}  #  {}  #  {}  #  {}  #  {}  #  {}  #  {}  #  {}  #", v[48], v[49], v[50], v[51], v[52], v[53], v[54], v[55]);
    let row8 = format!("8 #  {}  #  {}  #  {}  #  {}  #  {}  #  {}  #  {}  #  {}  #", v[56], v[57], v[58], v[59], v[60], v[61], v[62], v[63]);


    println!("     A     B     C     D     E     F     G     H   ");
    println!("  #################################################");
    println!("  #     #     #     #     #     #     #     #     #");
    println!("{row8}");
    println!("  #     #     #     #     #     #     #     #     #");
    println!("  #################################################");
    println!("  #     #     #     #     #     #     #     #     #");
    println!("{row7}");
    println!("  #     #     #     #     #     #     #     #     #");
    println!("  #################################################");
    println!("  #     #     #     #     #     #     #     #     #");
    println!("{row6}");
    println!("  #     #     #     #     #     #     #     #     #");
    println!("  #################################################");
    println!("  #     #     #     #     #     #     #     #     #");
    println!("{row5}");
    println!("  #     #     #     #     #     #     #     #     #");
    println!("  #################################################");
    println!("  #     #     #     #     #     #     #     #     #");
    println!("{row4}");
    println!("  #     #     #     #     #     #     #     #     #");
    println!("  #################################################");
    println!("  #     #     #     #     #     #     #     #     #");
    println!("{row3}");
    println!("  #     #     #     #     #     #     #     #     #");
    println!("  #################################################");
    println!("  #     #     #     #     #     #     #     #     #");
    println!("{row2}");
    println!("  #     #     #     #     #     #     #     #     #");
    println!("  #################################################");
    println!("  #     #     #     #     #     #     #     #     #");
    println!("{row1}");
    println!("  #     #     #     #     #     #     #     #     #");
    println!("  #################################################");
}