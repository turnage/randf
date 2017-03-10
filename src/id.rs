//! Random entities.

use std::str::FromStr;

use parse;

static NAME: &'static str = "identifier";

// A_OFFSET is the ordinal value of 'a'; subtract this from a letter to yield an identifier index.
const A_OFFSET: u8 = 97;

/// ID is an identifier for a random entity.
pub struct ID {
    pub index: u8
}

impl FromStr for ID {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let error = parse::error(s, NAME);
        if s.len() == 1 {
            match s.chars().next().unwrap_or('.') {
                i @ 'a'..'z' => Ok(ID{index: (i as u8) - A_OFFSET}),
                _ => error
            }
        } else {
            error
        }
    }
}
