//! Relative operators.

use std::str::FromStr;

use parse;

static NAME: &'static str = "relative operator";

pub enum Relop {
    Gt,
    Ge,
    Eq,
    Ne,
    Le,
    Lt,
}

impl FromStr for Relop {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            ">" => Ok(Relop::Gt),
            ">=" => Ok(Relop::Ge),
            "==" => Ok(Relop::Eq),
            "!=" => Ok(Relop::Ne),
            "<=" => Ok(Relop::Le),
            "<" => Ok(Relop::Lt),
            _ => parse::error(s, NAME),
        }
    }
}
