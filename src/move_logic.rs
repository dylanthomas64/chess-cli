use std::fmt::{self, Display};
use std::str::FromStr;


use crate::{
    pieces::{Colour, PieceType, Piece},
    errors::BoardError,
    coordinate::Coordinate,
};



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
            if squares[target].is_none() {
                // check for promotion
                if (56..64).contains(&target) {
                    move_vec.push((target, MoveType::PromotionPush));
                } else {
                    move_vec.push((target, MoveType::Regular));
                    // if piece on starting file and can move 1 extra space -> can double move
                    target += 8;
                    if (8..16).contains(&piece_index) && squares[target].is_none() {
                        move_vec.push((target, MoveType::DoublePush(target - 8)));
                    }
                }
            }
            // up left capture
            // if not on a-file
            if &piece_index % 8 != 0 {
                target = piece_index + 7;
                if let Some(piece) = squares[target] {
                    if piece.colour == Colour::Black {
                        // check for promotion
                        if (56..64).contains(&target) {
                            move_vec.push((target, MoveType::PromotionCapture));
                        } else {
                            move_vec.push((target, MoveType::Capture));
                        }
                    }
                }
                // check for en passant
                else if let Some(coord) = en_passant_target {
                    if target == coord.try_into()? {
                        move_vec.push((target, MoveType::EnPassant(target - 8)));
                    }
                }
            }

            // up right capture
            // if piece is not on h-file
            if &piece_index % 8 != 7 {
                target = piece_index + 9;
                if let Some(piece) = squares[target] {
                    if piece.colour == Colour::Black {
                        // check for promotion
                        if (56..64).contains(&target) {
                            move_vec.push((target, MoveType::PromotionCapture));
                        } else {
                            move_vec.push((target, MoveType::Capture));
                        }
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
            let mut target = piece_index - 8;
            if squares[target].is_none() {
                // check for promotion
                if (0..8).contains(&target) {
                    move_vec.push((target, MoveType::PromotionPush));
                } else {
                    move_vec.push((target, MoveType::Regular));
                    // if piece on starting file and can move 1 extra space -> can double move
                    if (48..56).contains(&piece_index) {
                        target -= 8;
                        if squares[target].is_none() {
                            move_vec.push((target, MoveType::DoublePush(target + 8)));
                        }
                    }
                }
            }

            // down right capture

            // if target on h file then not allowed
            if &piece_index % 8 != 7 {
                target = &piece_index - 7;
                if let Some(piece) = squares[target] {
                    if piece.colour == Colour::White {
                        if (0..8).contains(&target) {
                            move_vec.push((target, MoveType::PromotionCapture));
                        } else {
                            move_vec.push((target, MoveType::Capture));
                        }
                    }
                }
                // check for en passant
                else if let Some(coord) = en_passant_target {
                    if target == coord.try_into()? {
                        move_vec.push((target, MoveType::EnPassant(target + 8)));
                    }
                }
            }

            // down left capture
            //if is not on a-file
            if &piece_index % 8 != 0 {
                target = &piece_index - 9;
                if let Some(piece) = squares[target] {
                    if piece.colour == Colour::White {
                        if (0..8).contains(&target) {
                            move_vec.push((target, MoveType::PromotionCapture));
                        } else {
                            move_vec.push((target, MoveType::Capture));
                        }
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
    Ok(move_vec)
}

#[allow(dead_code)]
#[allow(unused_variables)]
pub fn get_bishop_legal_moves(
    squares: &[Option<Piece>],
    piece_index: usize,
    colour: &Colour,
) -> Result<Vec<(usize, MoveType)>, BoardError> {
    todo!()
}
