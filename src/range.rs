//! Traits for and operations on Ranges.

use std::str::FromStr;

use parse;

static NAME: &'static str = "range";

/// Range is inclusive at start and exclusive at end.
#[derive(PartialEq, Debug)]
pub struct Range {
    pub start: f64,
    pub end: f64,
}

impl FromStr for Range {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let error = parse::error(s, NAME);
        match s.split_terminator('-').collect::<Vec<&str>>().as_slice() {
            &[ref s, ref e] => {
                match (s.parse::<f64>(), e.parse::<f64>()) {
                    (Ok(s), Ok(e)) => Ok(Range { start: s, end: e }),
                    _ => error,
                }
            }
            _ => error,
        }
    }
}

#[cfg(test)]
mod test {
    use super::{Range, NAME};
    use parse::error;

    #[test]
    fn test_parse() {
        assert_eq!("0-255".parse::<Range>(),
                   Ok(Range {
                       start: 0.0,
                       end: 255.0,
                   }));
        assert_eq!("5.5-5.7".parse::<Range>(),
                   Ok(Range {
                       start: 5.5,
                       end: 5.7,
                   }));
        assert_eq!("ii".parse::<Range>(), error("ii", NAME));
        assert_eq!("6.7-ii".parse::<Range>(), error("6.7-ii", NAME));
        assert_eq!("-0.0".parse::<Range>(), error("-0.0", NAME));
    }
}
