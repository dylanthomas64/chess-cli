use std::{
    fmt::{self, Display},
    str::FromStr,
};

#[derive(PartialEq, Debug, Clone)]
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
            _ => Err(ParseError::ColourError),
        }
    }
}
#[derive(Debug, PartialEq, Clone)]
enum PieceType {
    Pawn,
    Bishop,
    Knight,
    Rook,
    Queen,
    King,
}
#[derive(Clone, Debug)]
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
            _ => return Err(ParseError::CharError),
        };
        let colour = if c.is_ascii_uppercase() {
            Colour::White
        } else {
            Colour::Black
        };
        Ok(Piece { piece_type, colour })
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

#[derive(Debug)]
pub struct Move {
    from: Coordinate,
    destination: Coordinate,
}

impl FromStr for Move {
    type Err = ParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.len() != 4 {
            Err(ParseError::MoveError)
        } else {
            let from = Coordinate::from_str(&s[0..2])?;
            let destination = Coordinate::from_str(&s[2..4])?;
            Ok(Move { from, destination })
        }
    }
}
#[derive(Debug, PartialEq)]
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
    type Err = ParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let len = s.chars().count();
        if len > 4 {
            return Err(ParseError::CastlingRightsError);
        }
        let (mut k_w, mut q_w, mut k_b, mut q_b) = (false, false, false, false);
        for c in s.chars() {
            match c {
                'K' => k_w = true,
                'Q' => q_w = true,
                'k' => k_b = true,
                'q' => q_b = true,
                _ => return Err(ParseError::CastlingRightsError),
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
    type Err = ParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut chars = s.chars();
        let file: char;
        let rank: usize;
        let possible_files = ['a', 'b', 'c', 'd', 'e', 'f', 'g', 'h'];
        if chars.clone().count() == 2usize {
            file = chars.next().unwrap();
            rank = match chars.next().unwrap().to_string().parse::<usize>() {
                Ok(rank) => rank,
                Err(_) => return Err(ParseError::IntError),
            };
            for c in possible_files {
                if c == file {
                    if (1..=8).contains(&rank) {
                        return Ok(Coordinate { file, rank });
                    } else {
                        return Err(ParseError::CoordinateError(
                            "file not in range 1..=8".to_string(),
                        ));
                    }
                }
            }
            return Err(ParseError::CoordinateError(
                "file not in range a-h".to_string(),
            ));
        }
        Err(ParseError::CoordinateError(
            "requires exactly 2 chars eg. a1".to_string(),
        ))
    }
}

impl TryInto<usize> for Coordinate {
    type Error = ParseError;
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
    type Error = ParseError;
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

// contains board representation and relevant information
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
    pub fn new(fen: String) -> Result<Board, ParseError> {
        Board::from_str(&fen)
    }
    pub fn default() -> Board {
        Board::new("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1".to_string()).unwrap()
    }
    pub fn process_move(&mut self, mv: Move) -> Result<(), ParseError> {
        let i0: usize = mv.from.try_into()?;
        let i: usize = mv.destination.try_into()?;
        if let Some(piece) = &self.squares[i0] {
            self.squares[i] = Some(piece.clone());
            self.squares[i0] = None;
            Ok(())
        } else {
            panic!()
        }
    }
}
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

// error types
#[derive(Debug, PartialEq)]
pub enum ParseError {
    FenError(String),
    CharError,
    IntError,
    ColourError,
    CastlingRightsError,
    CoordinateError(String),
    MoveError,
}

impl FromStr for Board {
    type Err = ParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        // accept only valid fen strings and return a board
        // startpos = rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1
        // possible pos = rnbqkbnr/pp1ppppp/8/2p5/4P3/5N2/PPPP1PPP/RNBQKB1R b KQkq - 1 2
        let mut fen_it = s.split_ascii_whitespace();
        let fen_piece_data = match fen_it.next() {
            Some(str) => str,
            None => return Err(ParseError::FenError("no piece data found".to_string())),
        };

        let active_colour = match fen_it.next() {
            Some(str) => str.parse::<Colour>()?,
            None => return Err(ParseError::FenError("no colour data found".to_string())),
        };
        let castling_rights = match fen_it.next() {
            Some(str) => str.parse::<CastlingRights>()?,
            None => {
                return Err(ParseError::FenError(
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
                return Err(ParseError::FenError(
                    "no en passant target data found".to_string(),
                ))
            }
        };
        let half_move_clock = match fen_it.next() {
            Some(str) => str.parse::<usize>().unwrap(),
            None => {
                return Err(ParseError::FenError(
                    "no half move clock data found".to_string(),
                ))
            }
        };
        let full_move_number = match fen_it.next() {
            Some(str) => str.parse::<usize>().unwrap(),
            None => {
                return Err(ParseError::FenError(
                    "no full move number data found".to_string(),
                ))
            }
        };

        // check that the fen iterator has been exhausted
        let extra_args = fen_it.next();
        if extra_args.is_some() {
            return Err(ParseError::FenError(format!(
                "too many arguments! Extra args found: {}",
                extra_args.unwrap()
            )));
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
                    match piece.to_string().parse::<usize>() {
                        Ok(n) => {
                            for _ in 0..n {
                                board_rank_data.push(None);
                            }
                        }
                        Err(_) => return Err(ParseError::IntError),
                    }
                }
            }
            if board_rank_data.len() != 8 {
                return Err(ParseError::FenError(format!(
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

/* ------- T E S T S ---------*/

#[cfg(test)]
mod conversions {
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
        assert_eq!(Err(ParseError::ColourError), "-".parse::<Colour>());
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
}
