use core::fmt;
use std::{fmt::Display, str::FromStr};

use crate::errors::BoardError;

#[derive(PartialEq, Debug, Clone, Copy)]
pub enum Colour {
    White,
    Black,
}

impl Colour {
    pub fn change_colour(&mut self) {
        match self {
            Colour::White => *self = Colour::Black,
            Colour::Black => *self = Colour::White,
        }
    }
}

impl FromStr for Colour {
    type Err = BoardError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "w" => Ok(Colour::White),
            "b" => Ok(Colour::Black),
            _ => Err(BoardError::ColourError),
        }
    }
}

impl Display for Colour {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            Colour::White => "w",
            Colour::Black => "b",
        };
        write!(f, "{}", s)
    }
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum PieceType {
    Pawn,
    Bishop,
    Knight,
    Rook,
    Queen,
    King,
}
#[derive(Clone, Debug, Copy)]
pub struct Piece {
    pub piece_type: PieceType,
    pub colour: Colour,
}

impl TryFrom<char> for Piece {
    type Error = BoardError;
    fn try_from(c: char) -> Result<Self, BoardError> {
        let piece_type = match &c.to_ascii_lowercase() {
            'p' => PieceType::Pawn,
            'b' => PieceType::Bishop,
            'n' => PieceType::Knight,
            'r' => PieceType::Rook,
            'q' => PieceType::Queen,
            'k' => PieceType::King,
            _ => return Err(BoardError::PieceError),
        };
        let colour = if c.is_ascii_uppercase() {
            Colour::White
        } else {
            Colour::Black
        };
        Ok(Piece { piece_type, colour })
    }
}

impl From<Piece> for char {
    fn from(value: Piece) -> Self {
        let piece_type = match value.piece_type {
            PieceType::Pawn => 'p',
            PieceType::Bishop => 'b',
            PieceType::Knight => 'n',
            PieceType::Rook => 'r',
            PieceType::Queen => 'q',
            PieceType::King => 'k',
        };
        match value.colour {
            Colour::White => piece_type.to_ascii_uppercase(),
            Colour::Black => piece_type,
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
            write!(f, "\x1B[36m{}", p.to_ascii_uppercase())?;
        } else {
            write!(f, "\x1B[31m{}", p)?;
        }
        // rest to default colour (black)
        write!(f, "\x1B[30m")
    }
}

impl Piece {
    pub fn unicode_symbol(&self) -> String {
        
        if self.colour == Colour::White {
            let p = match &self.piece_type {
                PieceType::Pawn => "♟",
                PieceType::Bishop => "♝",
                PieceType::Knight => "♞",
                PieceType::Rook => "♜",
                PieceType::Queen => "♛",
                PieceType::King => "♚",
            };
            format!("\x1B[36m{}\x1B[30m", p)
        } else {
            let p = match &self.piece_type {
                PieceType::Pawn => "♙",
                PieceType::Bishop => "♗",
                PieceType::Knight => "♘",
                PieceType::Rook => "♖",
                PieceType::Queen => "♕",
                PieceType::King => "♔",
            };
            format!("\x1B[31m{}\x1B[30m", p)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

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
}
