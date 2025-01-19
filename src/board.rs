use std::{fmt::{self, Display}, str::FromStr};
#[derive(PartialEq)]
enum Colour {
    White,
    Black,
}

impl FromStr for Colour {
    type Err = ParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "w" => Ok(Colour::White),
            "b" => Ok(Colour::Black),
            _ => Err(ParseError::ParseColourError)
        }
    }
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


impl TryFrom<char> for Piece {
    type Error = ParseError;
    fn try_from(c: char) -> Result<Self, ParseError> {
        let piece_type = match &c.to_ascii_lowercase() {
            'p' => PieceType::Pawn,
            'b' => PieceType::Bishop,
            'n' => PieceType::Knight,
            'r' => PieceType::Rook,
            'q' => PieceType::Queen,
            'k' => PieceType::King,
            _ => return Err(ParseError::ParseCharError)
        };
        let colour = if c.is_ascii_uppercase() {
            Colour::White
        } else {
            Colour::Black
        };
        Ok(Piece {
            piece_type, colour
        })
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
impl FromStr for CastlingRights {
    type Err = ParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let len = s.chars.count();
        if len > 4 {
            return Err(ParseError::ParseCastlingRightsError)
        }
        let (mut K, mut Q, mut k, mut q) = (false, false, false, false);
        for c in s.chars() {
            match c {
               'K' => {K = true},
               'Q' => {Q = true},
               'k' => {k = true},
               'q' => {k = true},
               _ => return Err(ParseError::ParseCastlingRightsError)
            }
        }

        Ok(CastlingRights{K, Q, k, q})
    }
}
enum GameState {
    Active, 
    BlackWin, 
    WhiteWin, 
    Forfeit, 
    Stalemate, 
    Resignation, 
}

// struct to hold index values of board.squares
// created so that the from str can be implemented ie. a1 -> u8
struct SquaresIndex(u8);

// contains board representation and relevant information
struct Board {
    // where a1 == 0, a2 == 1, h8 == 63
    squares: Vec<Option<Piece>>,
    active_colour: Colour,
    castling_rights: CastlingRights,
    en_passant_target_square: Option<SquaresIndex>,
    // required for 50 move rule
    half_move_clock: u8,
    // starts at 1 and increments after black's move
    full_move_number: usize,
}

impl Board {
    fn new(fen: String) -> Result<Board, ParseError> {
        Board::from_str(&fen)
    }
    fn default() -> Board {
        Board::new("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1".to_string()).unwrap()
    }
}
// error types
#[derive(Debug)]
enum ParseError {
    ParseFenError(String),
    ParseCharError,
    ParseIntError,
    ParseColourError,
    ParseCastlingRightsError,
}

impl FromStr for Board {
    type Err = ParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        // accept only valid fen strings and return a board
        // startpos = rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1
        // possible pos = rnbqkbnr/pp1ppppp/8/2p5/4P3/5N2/PPPP1PPP/RNBQKB1R b KQkq - 1 2
        let fen_it = s.split_ascii_whitespace();
        let fen_piece_data = match fen_it.next() {
            Some(str) => str,
            None => return Err(ParseError::ParseFenError("no piece data found".to_string()))
        };

        let active_colour = match fen_it.next() {
            Some(str) => str.parse::<Colour>()?,
            None => return Err(ParseError::ParseFenError("no colour data found".to_string()))
        };
        let castling_rights = match fen_it.next() {
            Some(str) => str.parse::<CastlingRights>()?,
            None => return Err(ParseError::ParseFenError("no castling rights data found".to_string()))
        };
        let en_passant_target_square = match fen_it.next() {
            todo!()
            Some(str) => st
            None => return Err(ParseError::ParseFenError("no en passant target data found".to_string()))
        };
        let half_move_clock = match fen_it.next() {
            Some(str) => str.parse::<u8>()?,
            None => return Err(ParseError::ParseFenError("no half move clock data found".to_string()))
        };
        let full_move_number = match fen_it.next() {
            Some(str) => str.parse::<usize>()?,
            None => return Err(ParseError::ParseFenError("no full move number data found".to_string()))
        };

        // check that the fen iterator has been exhausted
        let extra_args = fen_it.next();
        if extra_args.is_some() {
            return Err(ParseError::ParseFenError(format!("too many arguments! Extra args found: {}", extra_args.unwrap())))
        }
        

        // Parse pieces

        let mut squares: Vec<Option<Piece>> = Vec::new();
        // reverse iterator to fill pieces in correct order (rank 1 to rank 8)
        for rank in fen_piece_data.split('/').rev() {
            // create temporary rank data to fill board.squares
            let mut board_rank_data: Vec<Option<Piece>> = Vec::new();
            let pieces = rank.chars();
            for piece in pieces {
                if piece.is_alphabetic() {
                    board_rank_data.push(Some(Piece::try_from(piece)?));
                } else if piece.is_numeric() {
                    match piece.to_string().parse::<u8>() {
                        Ok(n) => {
                            for i in 0..n {
                                board_rank_data.push(None);
                            }
                        },
                        Err(e) => return Err(ParseError::ParseIntError)
                    }
                }
            }
            if board_rank_data.len() != 8 {
                return Err(ParseError::ParseFenError(format!("rank length must equal 8. Current rank == {}", rank)))
            } else {
                squares.append(&mut board_rank_data)
            }
        }

        // parse everything else



        Ok(Board {
            squares,
            active_colour,
            castling_rights,
            en_passant_target_square,
            half_move_clock,
            full_move_number,
        })
    }
}