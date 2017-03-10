//! Constraints on random entity relationships.

use std::str::FromStr;

use regex::Regex;

use id::TemporalID;
use parse;
use relop::Relop;

static NAME: &'static str = "constraint";

pub struct Constraint {
    pub left: TemporalID,
    pub rel: Relop,
    pub right: TemporalID,
}

impl FromStr for Constraint {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let matcher = Regex::new(r"([a-z]\[.+\])(<=|==|>=)([a-z]\[.+\])").unwrap();
        if matcher.is_match(s) {
            let group = matcher.captures_iter(s).next().unwrap();
            let left = group.get(2).unwrap().as_str().parse::<TemporalID>()?;
            let rel = group.get(3).unwrap().as_str().parse::<Relop>()?;
            let right = group.get(4).unwrap().as_str().parse::<TemporalID>()?;
            Ok(Constraint {
                left: left,
                rel: rel,
                right: right,
            })
        } else {
            parse::error(s, NAME)
        }
    }
}
