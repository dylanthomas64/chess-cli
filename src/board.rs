use std::{
    fmt::{self, Display, Write},
    str::FromStr,
};


use crate::{
    coordinate::Coordinate, errors::BoardError, move_logic::{self, in_check, Move, MoveType}, pieces::{Colour, Piece}
};

// struct to represent castling rights
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
    pub fn process_move(&mut self, mv: &Move) -> Result<(), BoardError> {
        let i0: usize = mv.from.into();
        let i: usize = mv.destination.into();
        // causes error if no legal moves exist
        let move_type = Self::validate_move(self, i0, i)?;

        // get Piece to move
        let piece = match move_type {
            // if promotion
            MoveType::PromotionPush | MoveType::PromotionCapture => match mv.promotion {
                Some(piece_type) => Piece {
                    piece_type,
                    colour: self.active_colour,
                },
                None => return Err(BoardError::PromotionError),
            },
            // if anything else
            _ => self.squares[i0].unwrap(),
        };


        // clone squares and apply changes
        // check copy for possible attacks on king
        let mut squares_copy = self.squares.clone();
        squares_copy[i] = Some(piece);
        squares_copy[i0] = None;
        if in_check(&self.active_colour, &squares_copy) {
            println!("in check!");
        }
        // in_check(colour: Colour, squares: &[Option<Piece>]) -> Bool
        /*



        >> does this move put opponent king in check? (only required for PGN '+' notation)

        does this move put this colour king in check?

        create a copy of squares with applied move.
        check all of opponent's new legal moves. If any index contains this king => Err
        (maybe check for each legal move one at a time to reduce search time)

        squares = squares_copy

        */

        // move OK, apply changes to board

        self.squares[i] = Some(piece);
        self.squares[i0] = None;

        match move_type {
            // add en passant sq
            MoveType::DoublePush(target) => {
                self.en_passant_target_square = Some(target.try_into()?)
            }
            // remove en passant sq
            MoveType::EnPassant(captured) => {
                self.squares[captured] = None;
                self.en_passant_target_square = None;
            }
            _ => {}
        }
        // change active colour
        self.active_colour.change_colour();
        // change castling rights
        // change half-move clock
        // change full-move number
        Ok(())
    }
    // check that move is legal
    fn validate_move(&self, i0: usize, i: usize) -> Result<MoveType, BoardError> {
        // check piece is exists
        let piece = match &self.squares[i0] {
            Some(p) => p,
            None => return Err(BoardError::EmptySquare),
        };
        // check piece is correct colour
        if piece.colour != self.active_colour {
            return Err(BoardError::WrongColour);
        }
        // OK! find all legal moves from squares[i0]
        let mut legal_moves: Vec<(usize, MoveType)> = vec![];

        move_logic::find_legal_moves(&self.squares, &mut legal_moves, i0, &self.en_passant_target_square);
        // for (x, m) in &move_vec {
        //     //println!("[{}] {:?}", x, m);
        // }
        // check if index is in vec of legal moves
        match legal_moves.into_iter().find(|(x, _)| *x == i) {
            Some((_, m)) => Ok(m),
            None => Err(BoardError::InvalidMove),
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
mod tests {
    use super::*;

    
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
