//! Random entities.

use std::error::Error;
use std::str::FromStr;

use regex::Regex;
use meval;

use parse;

static NAME: &'static str = "identifier";
static TEMPORAL_NAME: &'static str = "temporal identifier";

// A_OFFSET is the ordinal value of 'a'; subtract this from a letter to yield an identifier index.
const A_OFFSET: u8 = 97;

/// ID is an identifier for a random entity.
#[derive(PartialEq, Debug)]
pub struct ID {
    pub index: u8,
}

impl FromStr for ID {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let error = parse::error(s, NAME);
        if s.len() == 1 {
            match s.chars().next().unwrap_or('.') {
                i @ 'a'...'z' => Ok(ID { index: (i as u8) - A_OFFSET }),
                _ => error,
            }
        } else {
            error
        }
    }
}

/// TemporalID is an identifier relative to itself over time. E.g. in a[i+2]<a[i] means a, two
/// instances ahead of the given a, must be less than that given a. time is the function which
/// generates the next index when provided i.
pub struct TemporalID {
    pub id: ID,
    pub time: Box<Fn(usize) -> Result<usize, String>>,
}

impl FromStr for TemporalID {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let matcher = Regex::new(r"(([a-z])\[(.+)\])").unwrap();
        if matcher.is_match(s) {
            let group = matcher.captures_iter(s).next().unwrap();
            let id = group.get(2).unwrap().as_str().parse::<ID>()?;
            let expr = group.get(3).unwrap().as_str();
            match expr.parse::<meval::Expr>() {
                Err(e) => Err(format!("invalid temporal expression: {}", e.description())),
                Ok(expr) => {
                    match expr.bind("i") {
                        Err(_) => Err(String::from("temporal expression must be function of i")),
                        Ok(t) => {
                            Ok(TemporalID {
                                id: id,
                                time: floor_temporal_expr(t),
                            })
                        }
                    }
                }
            }
        } else {
            parse::error(s, TEMPORAL_NAME)
        }
    }
}

fn floor_temporal_expr(te: Box<Fn(f64) -> f64>) -> Box<Fn(usize) -> Result<usize, String>> {
    Box::new(move |x| {
        let frame = (*te)(x as f64);
        if frame >= x as f64 {
            Ok(frame as usize)
        } else {
            Err(format!("temporal expression yielded {}, a frame prior to this one ({})",
                        frame,
                        x))
        }
    })
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_parse() {
        match "d[i * 9]".parse::<TemporalID>() {
            Ok(tid) => {
                assert_eq!(tid.id, ID { index: 3 });
                assert_eq!((*tid.time)(2), Ok(18));
            }
            Err(e) => {
                panic!(format!("d[i * 9] did not parse but is a valid temporal id: {}", e));
            }
        }

        match "a[i - 2]".parse::<TemporalID>() {
            Ok(tid) => {
                if let Ok(frame) = (*tid.time)(50) {
                    panic!("returning a past frame ({} from {}) should have yielded an error",
                           frame,
                           50)
                }
            }
            Err(e) => panic!(format!("a[i - 2] did not parse but is a valid temporal id: {}", e)),
        }
    }
}
