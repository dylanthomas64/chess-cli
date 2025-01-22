use std::{
    fmt::{self, Display, Write},
    str::FromStr,
};

use crate::errors::BoardError;
use crate::piece::{Colour, Piece, PieceType};

// human readable chess coordinate, that can be converted to an index to board.squares vector
#[derive(Debug, PartialEq)]
struct Coordinate {
    file: char,
    rank: usize,
}
impl Display for Coordinate {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}{}", self.file, self.rank)
    }
}

impl FromStr for Coordinate {
    type Err = BoardError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut chars = s.chars();
        let file: char;
        let rank: usize;
        let possible_files = ['a', 'b', 'c', 'd', 'e', 'f', 'g', 'h'];
        if chars.clone().count() == 2usize {
            file = chars.next().unwrap();
            rank = chars.next().unwrap().to_string().parse::<usize>()?;
            for c in possible_files {
                if c == file {
                    if (1..=8).contains(&rank) {
                        return Ok(Coordinate { file, rank });
                    } else {
                        return Err(BoardError::CoordinateError(
                            "rank not in range 1..=8".to_string(),
                        ));
                    }
                }
            }
            return Err(BoardError::CoordinateError(
                "file not in range a-h".to_string(),
            ));
        }
        Err(BoardError::CoordinateError(
            "must contain exactly 2 chars (promotion not implemented...)".to_string(),
        ))
    }
}

impl TryInto<usize> for Coordinate {
    type Error = BoardError;
    fn try_into(self) -> Result<usize, Self::Error> {
        let file: usize = match self.file {
            'a' => 0,
            'b' => 1,
            'c' => 2,
            'd' => 3,
            'e' => 4,
            'f' => 5,
            'g' => 6,
            'h' => 7,
            _ => panic!(),
        };
        let rank: usize = self.rank - 1;
        Ok(file + (8 * rank))
    }
}

impl TryFrom<usize> for Coordinate {
    type Error = BoardError;
    fn try_from(value: usize) -> Result<Self, Self::Error> {
        let file = match value % 8 {
            0 => 'a',
            1 => 'b',
            2 => 'c',
            3 => 'd',
            4 => 'e',
            5 => 'f',
            6 => 'g',
            7 => 'h',
            _ => panic!(),
        };
        let rank = (value / 8) + 1;
        Ok(Coordinate { file, rank })
    }
}

// move struct that provides all necessary information for a uci move
#[derive(Debug, PartialEq)]
pub struct Move {
    from: Coordinate,
    destination: Coordinate,
    // eg. for e7e8q
    promotion: Option<PieceType>,
}

impl FromStr for Move {
    type Err = BoardError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.len() == 4 {
            let from = Coordinate::from_str(&s[0..2])?;
            let destination = Coordinate::from_str(&s[2..4])?;
            Ok(Move {
                from,
                destination,
                promotion: None,
            })
        } else if s.len() == 5 {
            let from = Coordinate::from_str(&s[0..2])?;
            let destination = Coordinate::from_str(&s[2..4])?;
            let promotion = Some(
                Piece::try_from(s[4..5].chars().next().ok_or(BoardError::PromotionError)?)?
                    .piece_type,
            );
            Ok(Move {
                from,
                destination,
                promotion,
            })
        } else {
            Err(BoardError::MoveError)
        }
    }
}

impl Display for Move {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}{}", self.from, self.destination)
    }
}
#[derive(Debug, PartialEq)]

// struct to represent castling rights
struct CastlingRights {
    //white side
    k_w: bool,
    q_w: bool,
    //black side
    k_b: bool,
    q_b: bool,
}
impl Display for CastlingRights {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.k_w {
            write!(f, "K")?;
        }
        if self.q_w {
            write!(f, "Q")?;
        }
        if self.k_b {
            write!(f, "k")?;
        }
        if self.q_b {
            write!(f, "q")?;
        }
        write!(f, "")
    }
}
impl FromStr for CastlingRights {
    type Err = BoardError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let len = s.chars().count();
        if len > 4 {
            return Err(BoardError::CastlingRightsError);
        }
        let (mut k_w, mut q_w, mut k_b, mut q_b) = (false, false, false, false);
        for c in s.chars() {
            match c {
                'K' => k_w = true,
                'Q' => q_w = true,
                'k' => k_b = true,
                'q' => q_b = true,
                _ => return Err(BoardError::CastlingRightsError),
            }
        }

        Ok(CastlingRights { k_w, q_w, k_b, q_b })
    }
}
#[derive(Debug)]
#[allow(unused)]
enum GameState {
    Active,
    BlackWin,
    WhiteWin,
    Forfeit,
    Stalemate,
    Resignation,
}

// contains board representation and all relevant information. Constructed from FEN string.
pub struct Board {
    // where a1 == 0, a2 == 1, h8 == 63
    squares: Vec<Option<Piece>>,
    active_colour: Colour,
    castling_rights: CastlingRights,
    en_passant_target_square: Option<Coordinate>,
    // required for 50 move rule
    half_move_clock: usize,
    // starts at 1 and increments after black's move
    full_move_number: usize,
}

impl Board {
    pub fn new(fen: String) -> Result<Board, BoardError> {
        Self::from_str(&fen)
    }
    pub fn startpos() -> Board {
        Self::new("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1".to_string()).unwrap()
    }
    pub fn process_move(&mut self, mv: Move) -> Result<(), BoardError> {
        let i0: usize = mv.from.try_into()?;
        let i: usize = mv.destination.try_into()?;
        // causes error if no legal moves exist
        let move_type = Self::validate_move(self, i0, i)?;
        let piece = &self.squares[i0].unwrap();
        self.squares[i] = Some(*(piece));
        self.squares[i0] = None;
        // change active colour
        // change castling rights
        // change en passant sq
        // change half-move clock
        // change full-move number
        Ok(())
    }
    // check that move is legal
    fn validate_move(&self, i0: usize, i: usize) -> Result<MoveType, BoardError> {
        let move_vec = match Self::legal_moves(self, i0) {
            Ok(v) => v,
            Err(e) => return Err(e),
        };
        // for (x, m) in &move_vec {
        //     //println!("[{}] {:?}", x, m);
        // }
        // check if index is in vec of legal moves
        match move_vec.into_iter().find(|(x, _)| *x == i) {
            Some((_, m)) => Ok(m),
            None => Err(BoardError::InvalidMove),
        }
    }

    // provides a tuple of (possible indexs, move type) that the piece at supplied index can legally move to
    fn legal_moves(&self, i0: usize) -> Result<Vec<(usize, MoveType)>, BoardError> {
        let mut move_vec: Vec<(usize, MoveType)> = vec![];
        //println!("checking legal moves at v[{}]", i0);
        let piece = match &self.squares[i0] {
            Some(p) => p,
            None => return Err(BoardError::EmptySquare),
        };

        match piece.piece_type {
            PieceType::Pawn => {
                match piece.colour {
                    Colour::White => {
                        // regular moves
                        let mut target = i0 + 8;
                        // check target exists on the board and is empty
                        if (0..63).contains(&target) && self.squares[target].is_none() {
                            move_vec.push((target, MoveType::Regular));
                            // if piece on starting file and can move 1 extra space -> can double move
                            target += 8;
                            if (0..63).contains(&target)
                                && (8..16).contains(&i0)
                                && self.squares[target].is_none()
                            {
                                move_vec.push((target, MoveType::Regular));
                            }
                        }
                        // captures
                        target = i0 + 7;
                        if (0..63).contains(&target) {
                            if let Some(piece) = self.squares[target] {
                                if piece.colour == Colour::Black {
                                    move_vec.push((target, MoveType::Capture));
                                }
                            }
                        }
                        target = i0 + 9;
                        if (0..63).contains(&target) {
                            if let Some(piece) = self.squares[target] {
                                if piece.colour == Colour::Black {
                                    move_vec.push((target, MoveType::Capture));
                                }
                            }
                        }
                    }
                    Colour::Black => {}
                }
            }
            _ => todo!(),
        }

        if move_vec.is_empty() {
            println!("empty board");
            Err(BoardError::NoLegalMoves)
        } else {
            Ok(move_vec)
        }
    }

    #[allow(dead_code)]
    fn export_fen(&self) -> Result<String, BoardError> {
        let mut piece_data: Vec<String> = vec![];

        for rank in 0..8usize {
            let mut counter: u8 = 0;
            let mut rank_str: String = "".to_string();
            for file in 0..8usize {
                match &self.squares[file + (rank * 8)] {
                    Some(piece) => {
                        if counter == 0 {
                            rank_str.push((*piece).into());
                        } else {
                            rank_str.push(counter.to_string().chars().next().unwrap());
                            rank_str.push((*piece).into());
                            counter = 0;
                        }
                    }
                    None => counter += 1,
                }
            }
            if counter > 0 {
                rank_str.push(counter.to_string().chars().next().unwrap());
            }
            piece_data.push(rank_str);
        }

        let state: String = piece_data
            .iter()
            .rev()
            .fold(String::new(), |mut output, x| {
                let _ = write!(output, "{x}/");
                output
            })
            .strip_suffix('/')
            .unwrap()
            .to_string();

        // let mut state: String = piece_data
        //     .into_iter()
        //     .rev()
        //     .map(|s| format!("{}/", s))
        //     .collect();
        // state = state.strip_suffix('/').unwrap().to_string();

        let en_passant = match &self.en_passant_target_square {
            Some(coord) => coord.to_string(),
            None => "-".to_string(),
        };
        let fen_output = format!(
            "{} {} {} {} {} {}",
            state,
            self.active_colour,
            self.castling_rights,
            en_passant,
            self.half_move_clock,
            self.full_move_number
        );
        Ok(fen_output)
    }
}

// move logic
#[derive(Debug)]
enum MoveType {
    Regular,
    Capture,
    Castle,
    EnPassant,
    Promotion,
}

impl FromStr for Board {
    type Err = BoardError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        // accept only valid fen strings and return a board
        // startpos = rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1
        // possible pos = rnbqkbnr/pp1ppppp/8/2p5/4P3/5N2/PPPP1PPP/RNBQKB1R b KQkq - 1 2
        let mut fen_it = s.split_ascii_whitespace();
        let fen_piece_data = match fen_it.next() {
            Some(str) => str,
            None => return Err(BoardError::FenError("no piece data found".to_string())),
        };

        let active_colour = match fen_it.next() {
            Some(str) => str.parse::<Colour>()?,
            None => return Err(BoardError::FenError("no colour data found".to_string())),
        };
        let castling_rights = match fen_it.next() {
            Some(str) => str.parse::<CastlingRights>()?,
            None => {
                return Err(BoardError::FenError(
                    "no castling rights data found".to_string(),
                ))
            }
        };
        let en_passant_target_square = match fen_it.next() {
            Some(str) => {
                if str == '-'.to_string() {
                    None
                } else {
                    Some(str.parse::<Coordinate>().unwrap())
                }
            }
            None => {
                return Err(BoardError::FenError(
                    "no en passant target data found".to_string(),
                ))
            }
        };
        let half_move_clock = match fen_it.next() {
            Some(str) => str.parse::<usize>().unwrap(),
            None => {
                return Err(BoardError::FenError(
                    "no half move clock data found".to_string(),
                ))
            }
        };
        let full_move_number = match fen_it.next() {
            Some(str) => str.parse::<usize>().unwrap(),
            None => {
                return Err(BoardError::FenError(
                    "no full move number data found".to_string(),
                ))
            }
        };

        // check that the fen iterator has been exhausted
        let extra_args = fen_it.next();
        if extra_args.is_some() {
            return Err(BoardError::FenError(format!(
                "too many arguments! Extra args found: {}",
                extra_args.unwrap()
            )));
        }

        // Board pieces

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
                    let n = piece.to_string().parse::<usize>()?;
                    for _ in 0..n {
                        board_rank_data.push(None);
                    }
                }
            }
            if board_rank_data.len() != 8 {
                return Err(BoardError::FenError(format!(
                    "rank length must equal 8. Current rank == {}",
                    rank
                )));
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

// basic ascii chess board
impl fmt::Display for Board {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let v: Vec<String> = self
            .squares
            .iter()
            .map(|opt| match opt {
                Some(piece) => format!("{}", piece),
                None => " ".to_string(),
            })
            .collect();
        let row1 = format!(
            "1 |  {}  |  {}  |  {}  |  {}  |  {}  |  {}  |  {}  |  {}  |",
            v[0], v[1], v[2], v[3], v[4], v[5], v[6], v[7]
        );
        let row2 = format!(
            "2 |  {}  |  {}  |  {}  |  {}  |  {}  |  {}  |  {}  |  {}  |",
            v[8], v[9], v[10], v[11], v[12], v[13], v[14], v[15]
        );
        let row3 = format!(
            "3 |  {}  |  {}  |  {}  |  {}  |  {}  |  {}  |  {}  |  {}  |",
            v[16], v[17], v[18], v[19], v[20], v[21], v[22], v[23]
        );
        let row4 = format!(
            "4 |  {}  |  {}  |  {}  |  {}  |  {}  |  {}  |  {}  |  {}  |",
            v[24], v[25], v[26], v[27], v[28], v[29], v[30], v[31]
        );
        let row5 = format!(
            "5 |  {}  |  {}  |  {}  |  {}  |  {}  |  {}  |  {}  |  {}  |",
            v[32], v[33], v[34], v[35], v[36], v[37], v[38], v[39]
        );
        let row6 = format!(
            "6 |  {}  |  {}  |  {}  |  {}  |  {}  |  {}  |  {}  |  {}  |",
            v[40], v[41], v[42], v[43], v[44], v[45], v[46], v[47]
        );
        let row7 = format!(
            "7 |  {}  |  {}  |  {}  |  {}  |  {}  |  {}  |  {}  |  {}  |",
            v[48], v[49], v[50], v[51], v[52], v[53], v[54], v[55]
        );
        let row8 = format!(
            "8 |  {}  |  {}  |  {}  |  {}  |  {}  |  {}  |  {}  |  {}  |",
            v[56], v[57], v[58], v[59], v[60], v[61], v[62], v[63]
        );
        // set background colour
        //write!(f, "\x1B[106m")?;
        writeln!(f, "  —------------------------------------------------")?;
        writeln!(f, "{}", row8)?;
        writeln!(f, "  —------------------------------------------------")?;
        writeln!(f, "{}", row7)?;
        writeln!(f, "  —------------------------------------------------")?;
        writeln!(f, "{}", row6)?;
        writeln!(f, "  —------------------------------------------------")?;
        writeln!(f, "{}", row5)?;
        writeln!(f, "  —------------------------------------------------")?;
        writeln!(f, "{}", row4)?;
        writeln!(f, "  —------------------------------------------------")?;
        writeln!(f, "{}", row3)?;
        writeln!(f, "  —------------------------------------------------")?;
        writeln!(f, "{}", row2)?;
        writeln!(f, "  —------------------------------------------------")?;
        writeln!(f, "{}", row1)?;
        writeln!(f, "  —------------------------------------------------")?;
        writeln!(f, "     A     B     C     D     E     F     G     H   ")
    }
}

// debug information
impl fmt::Debug for Board {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "to move: '{:?}'", self.active_colour)?;
        writeln!(f, "castling rights: '{}'", self.castling_rights)?;
        writeln!(
            f,
            "en passant target: '{:?}'",
            self.en_passant_target_square
        )?;
        writeln!(f, "half-moves: '{:?}'", self.half_move_clock)?;
        write!(f, "moves: '{:?}'", self.full_move_number)
    }
}
/* ------- T E S T S ---------*/

#[cfg(test)]
mod constructors {
    use super::*;

    // piece tests
    // from char
    #[test]
    fn piece_from_char() {
        let piece: Piece = 'K'.try_into().unwrap();
        assert_eq!(piece.piece_type, PieceType::King);
        assert_eq!(piece.colour, Colour::White);
        let piece: Piece = 'q'.try_into().unwrap();
        assert_eq!(piece.piece_type, PieceType::Queen);
        assert_eq!(piece.colour, Colour::Black);
        let piece: Piece = 'P'.try_into().unwrap();
        assert_eq!(piece.piece_type, PieceType::Pawn);
        assert_eq!(piece.colour, Colour::White);
    }
    #[test]
    fn colour_from_str() {
        assert_eq!(Colour::White, "w".parse().unwrap());
        assert_eq!(Colour::Black, "b".parse().unwrap());
        assert_eq!(Err(BoardError::ColourError), "-".parse::<Colour>());
    }
    #[test]
    fn castling_rights_from_str() {
        let from_str = CastlingRights::from_str("KQkq").unwrap();
        let castling_rights = CastlingRights {
            k_w: true,
            q_w: true,
            k_b: true,
            q_b: true,
        };
        assert_eq!(from_str, castling_rights);

        let from_str = CastlingRights::from_str("Qq").unwrap();
        let castling_rights = CastlingRights {
            k_w: false,
            q_w: true,
            k_b: false,
            q_b: true,
        };
        assert_eq!(from_str, castling_rights);
    }
    #[test]
    fn coord_from_str() {
        let coord = Coordinate { file: 'a', rank: 1 };
        assert_eq!(coord, Coordinate::from_str("a1").unwrap());
        let coord = Coordinate { file: 'h', rank: 8 };
        assert_eq!(coord, Coordinate::from_str("h8").unwrap());
    }
    #[test]
    fn coord_from_usize() {
        let coord = Coordinate { file: 'a', rank: 1 };
        let index = 0usize;
        assert_eq!(coord, Coordinate::try_from(index).unwrap());

        let coord = Coordinate { file: 'h', rank: 8 };
        let index = 63usize;
        assert_eq!(coord, Coordinate::try_from(index).unwrap());
    }
    #[test]
    fn coord_into_usize() {
        let coord = Coordinate { file: 'a', rank: 1 };
        let index = 0usize;
        assert_eq!(index, Coordinate::try_into(coord).unwrap());
        let coord = Coordinate { file: 'h', rank: 8 };
        let index = 63usize;
        assert_eq!(index, Coordinate::try_into(coord).unwrap());
        let coord = Coordinate { file: 'b', rank: 1 };
        let index = 1usize;
        assert_eq!(index, Coordinate::try_into(coord).unwrap());
        let coord = Coordinate { file: 'b', rank: 2 };
        let index = 9usize;
        assert_eq!(index, Coordinate::try_into(coord).unwrap());
    }

    #[test]
    fn move_from_str() {
        let from = Coordinate::from_str("e3").unwrap();
        let destination = Coordinate::from_str("e4").unwrap();
        let mv = Move {
            from,
            destination,
            promotion: None,
        };
        assert_eq!(mv, Move::from_str("e3e4").unwrap());
        // promotion
        let from = Coordinate::from_str("e7").unwrap();
        let destination = Coordinate::from_str("e8").unwrap();
        let mv = Move {
            from,
            destination,
            promotion: Some(PieceType::Queen),
        };
        assert_eq!(mv, Move::from_str("e7e8q").unwrap());
    }
}

#[cfg(test)]
mod exports {
    use super::*;
    #[test]
    fn fen_export() {
        let fen_vec: Vec<String> = vec![
            "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1".to_string(),
            "rnbqkbnr/pp1ppppp/8/2p5/4P3/8/PPPP1PPP/RNBQKBNR w KQkq c6 0 2".to_string(),
            "rn2kb1r/ppp1pppp/8/8/4q3/3P1N1b/PPP1BPnP/RNBQ1K1R b kq - 0 1".to_string(),
        ];
        for fen in fen_vec {
            let board = Board::new(fen.clone()).unwrap();
            assert_eq!(fen, board.export_fen().unwrap());
        }
    }
}
