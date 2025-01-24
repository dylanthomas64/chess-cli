use std::fmt::{self, Display};
use std::str::FromStr;

use crate::{
    coordinate::Coordinate,
    errors::BoardError,
    pieces::{Colour, Piece, PieceType},
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

// find all legal moves from VALID index. If squares[index] will panic
// modify array of legal_moves in place (for speed)
pub fn find_legal_moves(squares: &[Option<Piece>], legal_moves: &mut Vec<(usize, MoveType)>, index: usize, en_passant_target: &Option<Coordinate>) {
    
    let piece = squares[index].unwrap();

    // create vec of legal moves to be passed around to each
    //let mut move_vec: Vec<(usize, MoveType)> = vec![];
    match piece.piece_type {
        PieceType::Pawn => get_pawn_legal_moves(
            legal_moves,
            squares,
            index,
            &piece.colour,
            en_passant_target,
        ),
        _ => todo!(),
    }
}

// is given colour currently in check?
pub fn in_check(this_colour: &Colour, squares: &[Option<Piece>]) -> bool {

    let opp_colour = match this_colour {
        Colour::White => Colour::Black,
        Colour::Black => Colour::White,
    };

    let mut legal_moves: Vec<(usize, MoveType)> = vec![];

    // iterator of all pieces
    let piece_it = squares
    .iter()
    .enumerate()
    .filter(|(_i, &x)| x.is_some())
    .map(|(_i, x)| (_i, x.unwrap()));

    let (king_location, _) = piece_it.clone().find(|(_, x)| x.piece_type == PieceType::King && x.colour == *this_colour).unwrap();


    piece_it
    .filter(|(_i, x)| x.colour == opp_colour)
    // ^ an iterator of opponent piece indexes ^
    .for_each(|(i, _)| find_legal_moves(squares, &mut legal_moves, i, &None)); // king cannot exist in en passant target sq


    for (index, _) in legal_moves {
        if king_location == index {
            return true
        }
    }

    false
}





// Individual piece logic

fn get_pawn_legal_moves(
    legal_moves: &mut Vec<(usize, MoveType)>,
    squares: &[Option<Piece>],
    piece_index: usize,
    colour: &Colour,
    en_passant_target: &Option<Coordinate>,
) {
    match colour {
        Colour::White => {
            // regular moves
            let mut target = piece_index + 8;
            if squares[target].is_none() {
                // check for promotion
                if (56..64).contains(&target) {
                    legal_moves.push((target, MoveType::PromotionPush));
                } else {
                    legal_moves.push((target, MoveType::Regular));
                    // if piece on starting file and can move 1 extra space -> can double move
                    target += 8;
                    if (8..16).contains(&piece_index) && squares[target].is_none() {
                        legal_moves.push((target, MoveType::DoublePush(target - 8)));
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
                            legal_moves.push((target, MoveType::PromotionCapture));
                        } else {
                            legal_moves.push((target, MoveType::Capture));
                        }
                    }
                }
                // check for en passant
                else if let Some(coord) = en_passant_target {
                    if target == coord.into() {
                        legal_moves.push((target, MoveType::EnPassant(target - 8)));
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
                            legal_moves.push((target, MoveType::PromotionCapture));
                        } else {
                            legal_moves.push((target, MoveType::Capture));
                        }
                    }
                }
                // check for en passant
                else if let Some(coord) = en_passant_target {
                    if target == coord.into() {
                        legal_moves.push((target, MoveType::EnPassant(target - 8)));
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
                    legal_moves.push((target, MoveType::PromotionPush));
                } else {
                    legal_moves.push((target, MoveType::Regular));
                    // if piece on starting file and can move 1 extra space -> can double move
                    if (48..56).contains(&piece_index) {
                        target -= 8;
                        if squares[target].is_none() {
                            legal_moves.push((target, MoveType::DoublePush(target + 8)));
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
                            legal_moves.push((target, MoveType::PromotionCapture));
                        } else {
                            legal_moves.push((target, MoveType::Capture));
                        }
                    }
                }
                // check for en passant
                else if let Some(coord) = en_passant_target {
                    if target == coord.into() {
                        legal_moves.push((target, MoveType::EnPassant(target + 8)));
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
                            legal_moves.push((target, MoveType::PromotionCapture));
                        } else {
                            legal_moves.push((target, MoveType::Capture));
                        }
                    }
                }
                // check for en passant
                else if let Some(coord) = en_passant_target {
                    if target == coord.into() {
                        legal_moves.push((target, MoveType::EnPassant(target + 8)));
                    }
                }
            }
        }
    }
}

#[allow(dead_code)]
#[allow(unused_variables)]
fn get_bishop_legal_moves(
    legal_moves: &mut Vec<(usize, MoveType)>,
    squares: &[Option<Piece>],
    piece_index: usize,
    colour: &Colour,
) -> Result<Vec<(usize, MoveType)>, BoardError> {
    todo!()
}

#[allow(dead_code)]
#[allow(unused_variables)]
fn get_knight_legal_moves(
    legal_moves: &mut Vec<(usize, MoveType)>,
    squares: &[Option<Piece>],
    piece_index: usize,
    colour: &Colour,
) -> Result<Vec<(usize, MoveType)>, BoardError> {
    todo!()
}

#[allow(dead_code)]
#[allow(unused_variables)]
fn get_rook_legal_moves(
    legal_moves: &mut Vec<(usize, MoveType)>,
    squares: &[Option<Piece>],
    piece_index: usize,
    colour: &Colour,
) -> Result<Vec<(usize, MoveType)>, BoardError> {
    // search up until
    todo!()
}

#[allow(dead_code)]
#[allow(unused_variables)]
fn get_queen_legal_moves(
    legal_moves: &mut Vec<(usize, MoveType)>,
    squares: &[Option<Piece>],
    piece_index: usize,
    colour: &Colour,
) -> Result<Vec<(usize, MoveType)>, BoardError> {
    todo!()
}

#[allow(dead_code)]
#[allow(unused_variables)]
fn get_king_legal_moves(
    legal_moves: &mut Vec<(usize, MoveType)>,
    squares: &[Option<Piece>],
    piece_index: usize,
    colour: &Colour,
) -> Result<Vec<(usize, MoveType)>, BoardError> {
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

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