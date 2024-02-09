use std::cmp::max;
use std::iter::once;
use std::ops::{Add, Mul, Neg, Sub};

mod sign;

use sign::*;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct BigInteger {
    digits: Vec<u64>,
}

impl BigInteger {
    pub fn zero() -> Self {
        0_i32.into()
    }

    pub fn one() -> Self {
        1_i32.into()
    }

    fn from_digits<I: IntoIterator<Item = u64>>(digits: I) -> Self {
        let mut digits: Vec<_> = digits.into_iter().collect();
        while digits.len() > 1 {
            let n = digits.len();
            match (digits[n - 1], digits[n - 2]) {
                (0, prev) if (prev as i64) >= 0 => digits.pop(),
                (u64::MAX, prev) if (prev as i64) < 0 => digits.pop(),
                _ => break,
            };
        }
        Self { digits }
    }

    fn len(&self) -> usize {
        self.digits.len()
    }

    fn sign(&self) -> Sign {
        if *self.digits.last().unwrap() as i64 >= 0 {
            Positive
        } else {
            Negative
        }
    }

    fn multiplication_form(self) -> (Sign, Vec<u64>) {
        let sign = self.sign();
        let digits = match sign {
            Positive => self.digits,
            Negative => (-self).digits,
        };
        (sign, digits)
    }
}

impl Default for BigInteger {
    fn default() -> Self {
        Self::zero()
    }
}

impl From<i32> for BigInteger {
    fn from(value: i32) -> Self {
        Self { digits: vec![value as u64] }
    }
}

impl From<i64> for BigInteger {
    fn from(value: i64) -> Self {
        Self { digits: vec![value as u64] }
    }
}

impl Add for BigInteger {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        let lsign = self.sign().digit();
        let rsign = rhs.sign().digit();
        let mut digits = addition(self.digits, rhs.digits);
        let carry = digits.pop().unwrap_or(0);

        let sign_sum = (lsign as i128) + (rsign as i128) + carry as i128;
        digits.push(sign_sum as u64);
        digits.push((sign_sum >> 32) as u64);

        Self::from_digits(digits)
    }
}

impl Mul<Sign> for BigInteger {
    type Output = Self;

    fn mul(self, sign: Sign) -> Self::Output {
        match sign {
            Positive => self,
            Negative => -self,
        }
    }
}

impl Mul for BigInteger {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        let (lsign, ldigits) = self.multiplication_form();
        let (rsign, rdigits) = rhs.multiplication_form();
        BigInteger::from_digits(karatsuba(ldigits, rdigits)) * (lsign * rsign)
    }
}

impl Sub for BigInteger {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        self + (-rhs)
    }
}

impl Neg for BigInteger {
    type Output = Self;

    fn neg(self) -> Self::Output {
        let sign = self.sign().digit() as u64;
        Self::from_digits(self.digits.into_iter().chain(once(sign)).map(|d| !d))
            + Self::one()
    }
}

fn addition(x: Vec<u64>, y: Vec<u64>) -> Vec<u64> {
    let n = max(x.len(), y.len());
    let mut result = Vec::new();
    result.reserve(n + 1);
    if n == 0 {
        return result;
    }

    let mut carry = 0;
    for i in 0..n {
        let sum = (x[i] as u128) + (y[i] as u128) + carry;
        result.push(sum as u64);
        carry = sum >> 64;
    }
    result.push(carry as u64);
    result
}

fn karatsuba(x: Vec<u64>, y: Vec<u64>) -> Vec<u64> {
    let mut n = max(x.len(), y.len());
    if n == 1 {
        let prod = x[0] as u128 * y[0] as u128;
        return vec![prod as u64, (prod >> 64) as u64];
    }


    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn main() {
        assert_eq!(-1_i64 as u64, u64::MAX);
        assert_eq!(-1_i32 as i64, -1_i64);
    }

    #[test]
    fn test_sign_digit() {
        assert_eq!(BigInteger::zero().sign().digit(), 0);
        assert_eq!(BigInteger::one().sign().digit(), 0);
        assert_eq!(BigInteger::from(-1).sign().digit(), -1);
    }

    #[test]
    fn add() {
        assert_eq!(
            BigInteger::from(-1) + BigInteger::from(-1),
            BigInteger::from(-2)
        );
        assert_eq!(
            BigInteger::from(-1) + BigInteger::zero(),
            BigInteger::from(-1)
        );
        assert_eq!(
            BigInteger::from(-1) + BigInteger::from(2),
            BigInteger::one()
        );
    }

    #[test]
    fn sub() {
        assert_eq!(BigInteger::zero(), -BigInteger::zero());
        assert_eq!(
            BigInteger::zero() - BigInteger::one(),
            BigInteger::from(-1)
        );
        assert_eq!(--BigInteger::one(), BigInteger::one());
    }

    #[test]
    fn mul() {
        assert_eq!(BigInteger::zero(), -BigInteger::zero());
    }
}
