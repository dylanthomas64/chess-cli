use std::{fmt::{self, Display}, str::FromStr};

#[derive(PartialEq)]
enum Colour {
    White,
    Black,
}
enum PieceType {
    Pawn,
    Bishop,
    Knight,
    Rook,
    Queen,
    King,
}
struct Piece {
    piece_type: PieceType,
    colour: Colour,
}

struct ParsePieceError;

impl TryFrom<char> for Piece {
    type Error = ParsePieceError;
    fn try_from(c: char) -> Result<Self, ParsePieceError> {
        let piece_type = match c.to_ascii_lowercase() {
            'p' => PieceType::Pawn,
            'b' => PieceType::Bishop,
            'n' => Piece
            _ => return Err(ParsePieceError)
        }
    }
}


impl Display for Piece {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let p = match &self.piece_type {
            PieceType::Pawn => "p",
            PieceType::Bishop => "b",
            PieceType::Knight => "n",
            PieceType::Rook => "r",
            PieceType::Queen => "q",
            PieceType::King => "k",
        };
        if self.colour == Colour::White {
            write!(f, "{}", p.to_ascii_uppercase())
        } else {
            write!(f, "{}", p)
        }
        
    }
}


struct Move {
    colour: Colour,
    from: u8,
    destination: u8,
}

struct CastlingRights {
    //white side
    K: bool,
    Q: bool,
    //black side
    k: bool,
    q: bool,
}

enum GameState {
    Active, 
    BlackWin, 
    WhiteWin, 
    Forfeit, 
    Stalemate, 
    Resignation, 
}



// contains board representation and relevant information
struct Board {
    // where a1 == 0, a2 == 1, h8 == 63
    squares: Vec<Option<Piece>>,
    active_colour: Colour,
    castling_rights: CastlingRights,
    en_passant_target_square: Option<u8>,
    // required for 50 move rule
    half_move_clock: u8,
    // starts at 1 and increments after black's move
    full_move_number: usize,
}

impl Board {
    fn new(fen: String) -> Result<Board, ParseFenError> {
        Board::from_str(&fen)
    }
}
// error types
#[derive(Debug)]
enum ParseFenError {

}

impl FromStr for Board {
    type Err = ParseFenError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        // accept only valid fen strings and return a board
        // startpos = rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1
        let fen_it = s.split_ascii_whitespace();
        let piece_data = fen_it.next();
        let active_colour = fen_it.next();
        let castling_rights = fen_it.next();
        let en_passant_target_square = fen_it.next();
        let half_move_clock = fen_it.next();
        let full_move_number = fen_it.next();


    }
}