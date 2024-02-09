use std::ops::{Mul, MulAssign};

pub use Sign::*;

#[derive(Debug, Clone, Copy)]
pub enum Sign {
    Positive,
    Negative,
}

impl Mul for Sign {
    type Output = Sign;

    fn mul(self, rhs: Self) -> Self::Output {
        match (self, rhs) {
            (Positive, Positive) => Positive,
            (Negative, Negative) => Positive,
            _ => Negative,
        }
    }
}

impl Sign {
    pub fn digit(self) -> i64 {
        match self {
            Positive => 0,
            Negative => -1,
        }
    }
}
