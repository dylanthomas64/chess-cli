use std::fmt::{self, Display};
use std::str::FromStr;

use crate::pieces::{Colour, Piece};
use crate::{errors::BoardError, pieces::PieceType};

// human readable chess coordinate, that can be converted to an index to board.squares vector
#[derive(Debug, PartialEq, Clone, Copy)]
pub struct Coordinate {
    pub file: char,
    pub rank: usize,
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

impl TryInto<usize> for &Coordinate {
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
    pub from: Coordinate,
    pub destination: Coordinate,
    // eg. for e7e8q
    pub promotion: Option<PieceType>,
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

// move logic
#[derive(Debug, PartialEq)]
#[allow(dead_code)]
pub enum MoveType {
    Regular,
    // double pawn move providing index of en passant target square
    DoublePush(usize),
    Capture,
    CastleKingSide,
    CastleQueenSide,
    // provides index of piece capture by en passant
    EnPassant(usize),
    PromotionPush,
    PromotionCapture,
}

// piece logic

pub fn get_pawn_legal_moves(
    squares: &[Option<Piece>],
    piece_index: usize,
    colour: &Colour,
    en_passant_target: &Option<Coordinate>,
) -> Result<Vec<(usize, MoveType)>, BoardError> {
    let mut move_vec: Vec<(usize, MoveType)> = vec![];
    match colour {
        Colour::White => {
            // regular moves
            let mut target = piece_index + 8;
            // check target exists on the board and is empty
            if (0..64).contains(&target) && squares[target].is_none() {
                move_vec.push((target, MoveType::Regular));
                // if piece on starting file and can move 1 extra space -> can double move
                target += 8;
                if (0..64).contains(&target)
                    && (8..16).contains(&piece_index)
                    && squares[target].is_none()
                {
                    move_vec.push((target, MoveType::DoublePush(target - 8)));
                }
            }
            // captures
            target = piece_index + 7;
            if (0..64).contains(&target) {
                if let Some(piece) = squares[target] {
                    if piece.colour == Colour::Black {
                        move_vec.push((target, MoveType::Capture));
                    }
                }
                // check for en passant
                else if let Some(coord) = en_passant_target {
                    if target == coord.try_into()? {
                        move_vec.push((target, MoveType::EnPassant(target - 8)));
                    }
                }
            }
            target = piece_index + 9;
            if (0..64).contains(&target) {
                if let Some(piece) = squares[target] {
                    if piece.colour == Colour::Black {
                        move_vec.push((target, MoveType::Capture));
                    }
                }
                // check for en passant
                else if let Some(coord) = en_passant_target {
                    if target == coord.try_into()? {
                        move_vec.push((target, MoveType::EnPassant(target - 8)));
                    }
                }
            }
        }
        Colour::Black => {
            // regular moves
            // check for usize overflow
            if let Some(target) = usize::checked_sub(piece_index, 8) {
                // check target exists on the board and is empty
                if (0..64).contains(&target) && squares[target].is_none() {
                    move_vec.push((target, MoveType::Regular));
                    // if piece on starting file and can move 1 extra space -> can double move
                    if let Some(target) = usize::checked_sub(target, 8) {
                        if (0..64).contains(&target)
                            && (48..56).contains(&piece_index)
                            && squares[target].is_none()
                        {
                            move_vec.push((target, MoveType::DoublePush(target + 8)));
                        }
                    }
                }
            }

            // captures
            if let Some(target) = usize::checked_sub(piece_index, 7) {
                if (0..64).contains(&target) {
                    // if piece exists
                    if let Some(piece) = squares[target] {
                        if piece.colour == Colour::White {
                            move_vec.push((target, MoveType::Capture));
                        }
                    }
                    // check for en passant
                    else if let Some(coord) = en_passant_target {
                        if target == coord.try_into()? {
                            move_vec.push((target, MoveType::EnPassant(target + 8)));
                        }
                    }
                }
            }

            if let Some(target) = usize::checked_sub(piece_index, 9) {
                if (0..63).contains(&target) {
                    if let Some(piece) = squares[target] {
                        if piece.colour == Colour::White {
                            move_vec.push((target, MoveType::Capture));
                        }
                    }
                    // check for en passant
                    else if let Some(coord) = en_passant_target {
                        if target == coord.try_into()? {
                            move_vec.push((target, MoveType::EnPassant(target + 8)));
                        }
                    }
                }
            }
        }
    }
    Ok(move_vec)
}
