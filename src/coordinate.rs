use std::{
    fmt::{self, Display},
    str::FromStr,
};

use crate::errors::BoardError;

// COORDINATE
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

impl From<Coordinate> for usize {
    fn from(value: Coordinate) -> Self {
        let file: usize = match value.file {
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
        let rank: usize = value.rank - 1;
        file + (8 * rank)
    }
}

impl From<&Coordinate> for usize {
    fn from(value: &Coordinate) -> Self {
        let file: usize = match value.file {
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
        let rank: usize = value.rank - 1;
        file + (8 * rank)
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

#[cfg(test)]
mod tests {
    use super::*;

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
