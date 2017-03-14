use std::fmt;

use repr::Repr;

pub struct Value {
    raw: f64,
    repr: Repr,
}

impl Value {
    pub fn from(raw: f64, repr: Repr) -> Self {
        Self {
            raw: raw,
            repr: repr,
        }
    }
}

impl fmt::Display for Value {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self.repr {
            Repr::Hex => write!(f, "{:x}", self.raw as u64),
            Repr::Dec => write!(f, "{}", self.raw as i64),
            Repr::FP => write!(f, "{}", self.raw),
            Repr::Bin => write!(f, "{:b}", self.raw as u64),
        }
    }
}
