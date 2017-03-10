//! Traits for and operations on Ranges.

use std::str::FromStr;

/// Range is inclusive at start and exclusive at end.
#[derive(PartialEq, Debug)]
pub struct Range {
    pub start: f64,
    pub end: f64,
}

fn invalid_range_error(s: &str) -> Result<Range, String> {
    Err(format!("\"{}\" is not a valid range.", s))
}

impl FromStr for Range {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let error = invalid_range_error(s);
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
    use super::{Range, invalid_range_error};

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
        assert_eq!("ii".parse::<Range>(), invalid_range_error("ii"));
        assert_eq!("6.7-ii".parse::<Range>(), invalid_range_error("6.7-ii"));
        assert_eq!("-0.0".parse::<Range>(), invalid_range_error("-0.0"));
    }
}
