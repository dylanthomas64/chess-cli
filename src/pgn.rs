use std::{fmt::{self, Display}, str::FromStr};

use crate::errors::BoardError;

#[derive(Debug)]
pub struct Pgn {
    //tag_pairs: HashMap<String, String>,
    tag_text: Vec<String>,
    move_text: String,
}

impl Display for Pgn {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for tag in &self.tag_text {
            writeln!(f, "{}", tag)?
        }
        Ok(())
    }
}
impl FromStr for Pgn {
    type Err = BoardError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut tag_text: Vec<String> = Vec::new();
        for line in s.lines() {
            // locate [ and ] and push back slice instead of whole line.. in case of comments idk?
            if line.starts_with("[") {
                let b = line.find(']').unwrap();
                tag_text.push(line[0..=b].to_string());
            }
        }


        Ok(Pgn {
            tag_text,
            move_text: "".to_string(),
        })
    }
}