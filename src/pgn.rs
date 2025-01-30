use std::collections::HashMap;
use std::error::Error;
use std::fmt::{self, Display};
use std::str::FromStr;

use clap::builder::styling::Color;

use crate::board::{self, Board};
use crate::coordinate::Coordinate;
use crate::errors::BoardError;
use crate::move_logic::{find_legal_departure_squares, find_legal_moves, Move, MoveType};
use crate::pieces::{Colour, Piece, PieceType};

#[derive(Debug, PartialEq)]
pub struct PgnGame {
    pub headers: HashMap<String, String>,
    pub moves: Vec<String>,
    pub comments: Vec<String>, // Store comments separately
}

#[derive(Debug)]
pub enum PgnError {
    InvalidFormat,
}

impl fmt::Display for PgnError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            PgnError::InvalidFormat => write!(f, "Invalid PGN format"),
        }
    }
}

impl Error for PgnError {}

pub fn parse_pgn(input: &str) -> Result<PgnGame, PgnError> {
    let mut headers = HashMap::new();
    let mut moves = Vec::new();
    let mut comments = Vec::new();

    let lines = input.lines();

    for line in lines {
        let line = line.trim();
        if line.is_empty() {
            continue;
        }

        if line.starts_with('[') && line.ends_with(']') {
            // Parse header
            let line = &line[1..line.len() - 1]; // Remove brackets
            let parts: Vec<&str> = line.split_whitespace().collect();
            if parts.len() < 2 {
                return Err(PgnError::InvalidFormat);
            }
            let key = parts[0].to_string();
            let value = parts[1..].join(" ").trim_matches('"').to_string();
            headers.insert(key, value);
        } else {
            let mut chars = line.chars().peekable();
            while let Some(c) = chars.next() {
                match c {
                    '{' => {
                        // Parse comment in curly braces
                        let mut comment = String::new();
                        while let Some(c) = chars.next() {
                            if c == '}' {
                                break;
                            }
                            comment.push(c);
                        }
                        comments.push(comment.trim().to_string());
                    }
                    ';' => {
                        // Parse comment after semicolon (until end of line)
                        let comment: String = chars.collect();
                        comments.push(comment.trim().to_string());
                        break;
                    }
                    // NAGs
                    '$' => {
                        // Parse NAG
                        let mut nag = String::new();
                        while let Some(c) = chars.next() {
                            if c.is_whitespace() {
                                break;
                            }
                            nag.push(c);
                        }
                        println!("found NAG! {}", nag);
                    }
                    _ if c.is_whitespace() => continue,
                    _ => {
                        // Parse moves
                        let mut token = String::new();
                        token.push(c);
                        while let Some(&c) = chars.peek() {
                            if c.is_whitespace() || c == '{' || c == ';' {
                                break;
                            }
                            token.push(chars.next().unwrap());
                        }
                        if token.ends_with('.') {
                            // Skip move numbers (e.g., "1.", "2.")
                            continue;
                        }
                        if token == "1-0" || token == "0-1" || token == "1/2-1/2" {
                            // Skip game result
                            continue;
                        }
                        moves.push(token);
                    }
                }
            }
        }
    }

    if headers.is_empty() || moves.is_empty() {
        return Err(PgnError::InvalidFormat);
    }

    Ok(PgnGame {
        headers,
        moves,
        comments,
    })
}

// converts san to uci moves by validating them on a board
// this function is very ugly and should be remade

pub fn san_to_uci(input: Vec<SanMove>) -> Result<Vec<Move>, BoardError> {
    let mut board = board::Board::startpos();
    let mut uci_moves: Vec<Move> = Vec::new();
    for san in input {
        println!("{}", san);
        if let Some(c) = san.castling {
            let (i0, i): (usize, usize) = match c {
                Castling::KingSide => {
                    // validate move as we know the from square is king initial position
                    if board.active_colour == Colour::White {
                        (4, 6)
                    } else {
                        (59, 57)
                    }
                }
                Castling::QueenSide => {
                    if board.active_colour == Colour::White {
                        (4, 1)
                    } else {
                        (59, 62)
                    }
                }
            };
            let uci_move = Move {
                from: Coordinate::try_from(i0)?,
                destination: Coordinate::try_from(i)?,
                promotion: None,
            };
            board.process_uci_move(&uci_move)?;
            uci_moves.push(uci_move);
        } else if let Some(piece_coord) = san.destination {
            let i: usize = piece_coord.into();
            let i0 = match san.piece {
                PieceType::Pawn => {
                    // if no capture must be push
                    if !san.capture {
                        // check push and double pushes
                        if board.active_colour == Colour::White {
                            // check for pawn below
                            if let Some(p) = board.get_squares()[i - 8] {
                                if p.piece_type == PieceType::Pawn
                                    && p.colour == board.active_colour
                                {
                                    // OK regular pawn push
                                    i - 8
                                } else {
                                    return Err(BoardError::InvalidMove);
                                }
                            } else if let Some(p) = board.get_squares()[i - 16] {
                                if p.piece_type == PieceType::Pawn
                                    && p.colour == board.active_colour
                                {
                                    // ok double push
                                    i - 16
                                } else {
                                    // no valid pawn moves
                                    return Err(BoardError::InvalidMove);
                                }
                            } else {
                                // no valid pawn moves
                                return Err(BoardError::InvalidMove);
                            }
                        } else {
                            // check for pawn above
                            if let Some(p) = board.get_squares()[i + 8] {
                                if p.piece_type == PieceType::Pawn
                                    && p.colour == board.active_colour
                                {
                                    // OK regular pawn push
                                    i + 8
                                } else {
                                    return Err(BoardError::InvalidMove);
                                }
                            } else if let Some(p) = board.get_squares()[i + 16] {
                                if p.piece_type == PieceType::Pawn
                                    && p.colour == board.active_colour
                                {
                                    // ok double push
                                    i + 16
                                } else {
                                    // no valid pawn moves
                                    return Err(BoardError::InvalidMove);
                                }
                            } else {
                                // no valid pawn moves
                                return Err(BoardError::InvalidMove);
                            }
                        }
                    } else {
                        // pawn capture
                        // check diagonals below
                        match board.active_colour {
                            Colour::White => {
                                // below left
                                let mut possible_from: Vec<Coordinate> = Vec::new();
                                if let Some(p) = board.get_squares()[i - 9] {
                                    if p.piece_type == PieceType::Pawn
                                        && p.colour == board.active_colour
                                    {
                                        // pawn below left
                                        // necessary validation for possible edge of board wrapping
                                        if board::Board::validate_move(&board, i - 9, i).is_ok() {
                                            possible_from.push((i - 9).try_into()?);
                                        };
                                    }
                                }
                                // below right
                                if let Some(p) = board.get_squares()[i - 7] {
                                    if p.piece_type == PieceType::Pawn
                                        && p.colour == board.active_colour
                                    {
                                        // pawn below left
                                        if board::Board::validate_move(&board, i - 7, i).is_ok() {
                                            possible_from.push((i - 7).try_into()?);
                                        };
                                    }
                                }
                                if possible_from.len() == 1 {
                                    possible_from.pop().unwrap().into()
                                } else if possible_from.is_empty() {
                                    return Err(BoardError::InvalidMove);
                                } else {
                                    // apply qualifiers
                                    //let mut it = possible_from.iter();
                                    if let Some(fq) = san.file_qual {
                                        possible_from = possible_from
                                            .into_iter()
                                            .filter(|&c| c.file == fq)
                                            .collect();
                                    } else if let Some(rq) = san.rank_qual {
                                        possible_from = possible_from
                                            .into_iter()
                                            .filter(|&c| {
                                                c.rank == rq.to_digit(10).unwrap() as usize
                                            })
                                            .collect();
                                    } else if let Some(coord) = san.coord_qual {
                                        possible_from = possible_from
                                            .into_iter()
                                            .filter(|&c| c == coord)
                                            .collect();
                                    }

                                    if possible_from.len() == 1 {
                                        possible_from.pop().unwrap().into()
                                    } else {
                                        return Err(BoardError::BadQualifier);
                                    }
                                }
                            }

                            Colour::Black => {
                                // above left
                                let mut possible_from: Vec<Coordinate> = Vec::new();
                                if let Some(p) = board.get_squares()[i + 7] {
                                    if p.piece_type == PieceType::Pawn
                                        && p.colour == board.active_colour
                                    {
                                        // pawn below left
                                        // necessary validation for possible edge of board wrapping
                                        if board::Board::validate_move(&board, i + 7, i).is_ok() {
                                            possible_from.push((i + 7).try_into()?);
                                        };
                                    }
                                }
                                // below right
                                if let Some(p) = board.get_squares()[i + 9] {
                                    if p.piece_type == PieceType::Pawn
                                        && p.colour == board.active_colour
                                    {
                                        // pawn below left
                                        if board::Board::validate_move(&board, i + 9, i).is_ok() {
                                            possible_from.push((i + 9).try_into()?);
                                        };
                                    }
                                }

                                if possible_from.len() == 1 {
                                    possible_from.pop().unwrap().into()
                                } else if possible_from.is_empty() {
                                    return Err(BoardError::InvalidMove);
                                } else {
                                    // apply qualifiers
                                    //let mut it = possible_from.iter();
                                    if let Some(fq) = san.file_qual {
                                        possible_from = possible_from
                                            .into_iter()
                                            .filter(|&c| c.file == fq)
                                            .collect();
                                    } else if let Some(rq) = san.rank_qual {
                                        possible_from = possible_from
                                            .into_iter()
                                            .filter(|&c| {
                                                c.rank == rq.to_digit(10).unwrap() as usize
                                            })
                                            .collect();
                                    } else if let Some(coord) = san.coord_qual {
                                        possible_from = possible_from
                                            .into_iter()
                                            .filter(|&c| c == coord)
                                            .collect();
                                    }

                                    if possible_from.len() == 1 {
                                        possible_from.pop().unwrap().into()
                                    } else {
                                        return Err(BoardError::BadQualifier);
                                    }
                                }
                            }
                        }
                    }
                } // not a pawn, all other moves
                // does not work for pawns or castling because of different move directions and rules. ie/ all other pieces can return to where they came from
                _ => {
                    // find all legal moves from/to position
                    let mut legal_moves: Vec<(usize, MoveType)> = Vec::new();
                    println!("!!!");
                    find_legal_departure_squares(
                        board.get_squares(),
                        &mut legal_moves,
                        i,
                        Piece {piece_type: san.piece, colour: board.active_colour}
                    );

                    // convert legal moves to coords for qualifier checking
                    let mut possible_from: Vec<Coordinate> = legal_moves
                        .into_iter()
                        .map(|(i, _)| i.try_into().unwrap())
                        .collect();
                    
                    if possible_from.len() == 1 {
                        possible_from[0].into()
                    } else if possible_from.is_empty() {
                        return Err(BoardError::InvalidMove);
                    } else {
                        // apply qualifiers
                        //let mut it = possible_from.iter();
                        if let Some(fq) = san.file_qual {
                            possible_from = possible_from
                                .into_iter()
                                .filter(|&c| c.file == fq)
                                .collect();
                        } else if let Some(rq) = san.rank_qual {
                            possible_from = possible_from
                                .into_iter()
                                .filter(|&c| c.rank == rq.to_digit(10).unwrap() as usize)
                                .collect();
                        } else if let Some(coord) = san.coord_qual {
                            possible_from =
                                possible_from.into_iter().filter(|&c| c == coord).collect();
                        }

                        if possible_from.len() == 1 {
                            possible_from[0].into()
                        } else {
                            println!("{:?}", possible_from);
                            return Err(BoardError::BadQualifier);
                        }
                    }
                }
            };

            let uci_move = Move {
                from: i0.try_into()?,
                destination: san.destination.unwrap(),
                promotion: san.promotion,
            };
            println!("{}\n", uci_move);
            board.process_uci_move(&uci_move)?;
            uci_moves.push(uci_move)
        }
    }

    Ok(uci_moves)
}

// SAN Move Struct
// used to create uci moves
#[derive(Debug)]
pub struct SanMove {
    pub destination: Option<Coordinate>,
    // qualifiers
    pub piece: PieceType,
    pub file_qual: Option<char>,
    pub rank_qual: Option<char>,
    pub coord_qual: Option<Coordinate>,
    // status
    pub capture: bool,
    pub promotion: Option<PieceType>,
    pub castling: Option<Castling>,
    pub check: bool,
    pub checkmate: bool,
}

impl Display for SanMove {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "dest: {:?}", self.destination)?;
        writeln!(f, "piece: {:?}", self.piece)?;
        writeln!(f, "file: {:?}", self.file_qual)?;
        writeln!(f, "rank: {:?}", self.rank_qual)?;
        writeln!(f, "coord: {:?}", self.coord_qual)?;
        writeln!(f, "capt: {:?}", self.capture)?;
        writeln!(f, "promotion: {:?}", self.promotion)?;
        writeln!(f, "castling: {:?}", self.castling)
    }
}

// castle, promotion

#[derive(Debug)]
enum Castling {
    QueenSide,
    KingSide,
}

// God this is awful !!
// parses pgn.moves into valid uci moves by playing out the game on a temporary board
// start position is startpos unless a fen header exists
pub fn pgn_to_san_moves(input: Vec<String>) -> Result<Vec<SanMove>, BoardError> {
    let mut san_moves: Vec<SanMove> = Vec::new();
    for mut san in input {
        let mut destination: Option<Coordinate> = None;
        // qualifiers
        let mut piece: PieceType = PieceType::Pawn;
        let mut file_qual: Option<char> = None;
        let mut rank_qual: Option<char> = None;
        let mut coord_qual: Option<Coordinate> = None;
        //
        let mut capture: bool = false;
        let mut promotion: Option<PieceType> = None;
        let mut castling: Option<Castling> = None;
        let mut check: bool = false;
        let mut checkmate: bool = false;
        if san.len() < 2 {
            panic!()
        }
        if san.ends_with('+') {
            check = true;
            san = san.strip_suffix('+').unwrap().to_string();
        } else if san.ends_with('#') {
            checkmate = true;
            san = san.strip_suffix('#').unwrap().to_string();
        }

        // checks for promotion
        if let Some(i) = san.chars().position(|x| x == '=') {
            let promotion_str = san[i + 1..].to_string();
            promotion = Some(promotion_str.chars().next().unwrap().try_into()?);
            san.truncate(i);
        }
        // kingside castling
        if san == "O-O" || san == "0-0" {
            castling = Some(Castling::KingSide);
            san.clear();
            piece = PieceType::King;
        }
        // queenside castling
        else if san == "O-O-O" || san == "0-0-0" {
            castling = Some(Castling::QueenSide);
            san.clear();
            piece = PieceType::King;
        } else {
            // san stripped of +#=
            let mut chars = san.chars().rev();
            let rank = chars.next().unwrap();
            let file = chars.next().unwrap();
            let s: String = format!("{}{}", file, rank);
            destination = Some(s.parse().unwrap());
            san.truncate(san.len() - 2);

            // san stripped of destination
            // check captures and qualifiers

            if !san.is_empty() {
                /*
                (type)[(file)(rank)(coord)](x)
                if x strip suffix and caprue = true
                iter over chars
                if first is uppercase. remove and push piece qual
                [(file)(rank)(coord)]
                if is of size 1 = try for file, try for rank, else error
                else //must be a coord qual
                try for coord
                 */
                if san.ends_with("x") {
                    capture = true;
                    san = san.strip_suffix("x").unwrap().to_string()
                }

                let mut chars = san.chars().peekable();
                let c = chars.peek().unwrap();
                // if a piece qualifier
                if c.is_ascii_uppercase() {
                    piece = PieceType::try_from(chars.next().unwrap()).unwrap();
                }
                let other_qualifiers = Some(chars.collect::<String>()).unwrap();
                if !other_qualifiers.is_empty() {
                    if other_qualifiers.len() == 1 {
                        let c = other_qualifiers.chars().next().unwrap();
                        if ['a', 'b', 'c', 'd', 'e', 'f', 'g', 'h'].contains(&c) {
                            file_qual = Some(c);
                        } else if ['1', '2', '3', '4', '5', '6', '7', '8'].contains(&c) {
                            rank_qual = Some(c);
                        } else {
                            println!("{}", c);
                            return Err(BoardError::BadQualifier);
                        }
                    } else if other_qualifiers.len() == 2 {
                        //coord
                        if let Ok(coord) = other_qualifiers.parse() {
                            coord_qual = Some(coord);
                        } else {
                            println!("{}", other_qualifiers);
                            return Err(BoardError::BadQualifier);
                        }
                    } else {
                        return Err(BoardError::BadQualifier);
                    }
                }
            }
        }

        san_moves.push(SanMove {
            destination,
            piece,
            file_qual,
            rank_qual,
            coord_qual,
            capture,
            promotion,
            castling,
            check,
            checkmate,
        });
    }
    Ok(san_moves)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_pgn_with_comments() {
        let pgn_data = r#"
[Event "F/S Return Match"]
[Site "Belgrade, Serbia JUG"]
[Date "1992.11.04"]
[Round "29"]
[White "Fischer, Robert J."]
[Black "Spassky, Boris V."]
[Result "1/2-1/2"]

1. e4 {This is a great move!} e5 2. Nf3 Nc6 {Developing the knight} 3. Bb5 a6
4. Ba4 Nf6 5. O-O Be7 6. Re1 b5 7. Bb3 d6 8. c3 O-O 9. h3 Na5 10. Bc2 c5
11. d4 Qc7 12. Nbd2 cxd4 13. cxd4 Nc6 14. Nb3 Nb4 15. Bb1 Bb7 16. Nbd2 Rfe8
17. a3 Nc6 18. Bf4 Bf8 19. Qd3 g6 20. Rac1 Qb8 21. Rc2 Bg7 22. Qe2 Qb7
23. Rxc6 Qxc6 24. Qxc6 Bxc6 25. Bxa6 Bxa6 26. Rxa6 Rxa6 27. Kf1 Kf8
28. Ke2 Ke7 29. Kd3 Kd7 30. Kc4 Kc6 31. Kb4 Rb6 32. a4 Rb7 33. a5 Kb8
34. Ka4 Kc7 35. Ka3 Kc6 36. Ka4 Kc7 1/2-1/2
"#;

        let game = parse_pgn(pgn_data).unwrap();
        assert_eq!(game.headers.get("Event").unwrap(), "F/S Return Match");
        assert_eq!(game.moves[0], "e4");
        assert_eq!(game.moves[1], "e5");
        assert_eq!(game.comments[0], "This is a great move!");
        assert_eq!(game.comments[1], "Developing the knight");
    }

    #[test]
    fn test_parse_pgn_with_semicolon_comments() {
        let pgn_data = r#"
[Event "Test Game"]
[Site "Nowhere"]
[Date "2023.10.01"]
[Round "1"]
[White "Alice"]
[Black "Bob"]
[Result "1-0"]

1. e4 e5 ; This is a comment
2. Nf3 Nc6 ; Another comment
3. Bb5 a6 1-0
"#;

        let game = parse_pgn(pgn_data).unwrap();
        assert_eq!(game.comments[0], "This is a comment");
        assert_eq!(game.comments[1], "Another comment");
    }
}
