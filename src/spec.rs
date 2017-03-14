//! Specifications for generated random entities.

use std::iter::Iterator;
use std::str::FromStr;

use rand::{Rng, thread_rng};

use id::ID;
use range::Range;
use repr::Repr;
use parse;
use value::Value;

static NAME: &'static str = "specification";
static DEFAULT_REPR: &'static str = "d";
static DEFAULT_RANGE: &'static str = "0-256";

/// Spec defines the range and representation of a random entity.
#[derive(PartialEq, Debug)]
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

impl Iterator for Spec {
    type Item = Value;
    fn next(&mut self) -> Option<Self::Item> {
        Some(Value::from(thread_rng().gen_range::<f64>(self.range.start, self.range.end),
                         self.repr))
    }
}

#[cfg(test)]
mod test {
    use id::ID;
    use range::Range;
    use repr::Repr;
    use parse::error;

    use super::*;

    #[test]
    fn test_parse() {
        assert_eq!("b:0-255:x".parse::<Spec>(),
                   Ok(Spec {
                       id: ID { index: 1 },
                       range: Range {
                           start: 0.0,
                           end: 255.0,
                       },
                       repr: Repr::Hex,
                   }));
        assert_eq!("z:0-1".parse::<Spec>(),
                   Ok(Spec {
                       id: ID { index: 25 },
                       range: Range {
                           start: 0.0,
                           end: 1.0,
                       },
                       repr: Repr::Dec,
                   }));
        assert_eq!("z:0-1:x:9".parse::<Spec>(), error("z:0-1:x:9", NAME));
    }
}
