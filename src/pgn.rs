use std::collections::HashMap;
use std::error::Error;
use std::fmt::{self, Display};
use std::str::FromStr;

use crate::board::{self, Board};
use crate::coordinate::Coordinate;
use crate::move_logic::Move;
use crate::pieces::PieceType;

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

fn san_to_uci(input: Vec<SanMove>) -> Result<Vec<Move>, PgnError> {
    let mut board = board::Board::startpos();
    let mut uci_moves: Vec<Move> = Vec::new();
    for san in input {
        if let Some(c) = san.castling {
            match c {
                Castling::KingSide => {todo!()},
                Castling::QueenSide => {},
                _ => {
                    // if pawn should be simple enough
                    // else
                    // find all legal moves FROM destination. if one ok, if more then use qualifier
                }
            }
        }
    }


    todo!()
}

// SAN Move Struct
// used to create uci moves
#[derive(Debug)]
pub struct SanMove {
    pub destination: Option<Coordinate>,
    pub qualifier: Option<String>,
    pub capture: bool,
    pub promotion: Option<String>,
    pub castling: Option<Castling>,
    pub check: bool,
    pub checkmate: bool,
}

impl Display for SanMove {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "dest: {:?}", self.destination)?;
        writeln!(f, "qual: {:?}", self.qualifier)?;
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

// parses pgn.moves into valid uci moves by playing out the game on a temporary board
// start position is startpos unless a fen header exists
pub fn pgn_to_san_moves(input: Vec<String>) -> Vec<SanMove> {
    let board = Board::startpos();
    let mut san_moves: Vec<SanMove> = Vec::new();
    for mut san in input {
        let mut destination: Option<Coordinate> = None;
        let mut qualifier: Option<String> = None;
        let mut capture: bool = false;
        let mut promotion: Option<String> = None;
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
            promotion = Some(san[i + 1..].to_string());
            san.truncate(i);
        }

        if san.contains("x") {
            let mut split = san.split('x');
            if let Some(q) = split.next() {
                qualifier = Some(q.to_string());
            } else {
                qualifier = None;
            }
            if let Some(coord) = split.next() {
                destination = Some(Coordinate::from_str(coord).expect(&format!(
                    "couldn't convert SAN string '{}' to Coordinate",
                    coord
                )));
            }
            san.clear();
            capture = true;
        }
        // kingside castling
        else if san == "O-O" || san == "0-0" {
            castling = Some(Castling::KingSide);
            san.clear();
        }
        // queenside castling
        else if san == "O-O-O" || san == "0-0-0" {
            castling = Some(Castling::QueenSide);
            san.clear();
        }

        if !san.is_empty() {
            let mut chars = san.chars().rev();
            let rank = chars.next().unwrap();
            let file = chars.next().unwrap();
            let s: String = format!("{}{}", file, rank);
            destination = Some(s.parse().unwrap());
            san.truncate(san.len() - 2);
            if !san.is_empty() {
                qualifier = Some(san);
            }
        }
        san_moves.push(SanMove {
            destination,
            qualifier,
            capture,
            promotion,
            castling,
            check,
            checkmate,
        });
    }
    san_moves
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
