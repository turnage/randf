//! Specifications for generated random entities.

use std::str::FromStr;

use id::ID;
use range::Range;
use repr::Repr;
use parse;

static NAME: &'static str = "specification";
static DEFAULT_REPR: &'static str = "d";
static DEFAULT_RANGE: &'static str = "0-256";

/// Spec defines the range and representation of a random entity.
pub struct Spec {
    id: ID,
    range: Range,
    repr: Repr,
}

impl FromStr for Spec {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut fields = s.split_terminator(':');
        let id = fields.next().unwrap_or("").parse::<ID>()?;
        let range = fields.next().unwrap_or(DEFAULT_RANGE).parse::<Range>()?;
        let repr = fields.next().unwrap_or(DEFAULT_REPR).parse::<Repr>()?;
        if let Some(_) = fields.next() {
            parse::error(s, NAME)
        } else {
            Ok(Spec {
                id: id,
                range: range,
                repr: repr,
            })
        }
    }
}
