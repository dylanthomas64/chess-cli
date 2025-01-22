use std::{
    fmt::{self, Display},
    num::ParseIntError,
};

// error types
#[derive(Debug, PartialEq)]
pub enum BoardError {
    FenError(String),
    PieceError,
    ParseInt(std::num::ParseIntError),
    ColourError,
    CastlingRightsError,
    CoordinateError(String),
    MoveError,
    PromotionError,
    InvalidMove(String),
    PgnError,
    UciError,
}

impl From<ParseIntError> for BoardError {
    fn from(value: ParseIntError) -> Self {
        BoardError::ParseInt(value)
    }
}
impl Display for BoardError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let error_msg: &str = match self {
            Self::FenError(s) => s,
            Self::PieceError => "error creating piece",
            Self::ParseInt(e) => &format!("parse int error: {}", e),
            Self::ColourError => "error creating colour",
            Self::CastlingRightsError => "error creating castling rights",
            Self::CoordinateError(s) => &format!("error creating coordinate: {}", s),
            Self::MoveError => "error creating move",
            Self::PromotionError => "error trying to promote",
            Self::InvalidMove(s) => &format!("invalid move: {}", s),
            Self::PgnError => "error pgn",
            Self::UciError => "error uci",
        };
        write!(f, "{}", error_msg)
    }
}
impl std::error::Error for BoardError {}
