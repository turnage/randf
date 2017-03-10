//! Representations of random data.

use std::str::FromStr;

use parse;

static NAME: &'static str = "representation";

/// Repr can be any of the supported representations of random bits.
pub enum Repr {
    Hex,
    Dec,
    FP,
    Bin,
}

impl FromStr for Repr {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "x" => Ok(Repr::Hex),
            "d" => Ok(Repr::Dec),
            "f" => Ok(Repr::FP),
            "b" => Ok(Repr::Bin),
            _ => parse::error(s, NAME),
        }
    }
}
