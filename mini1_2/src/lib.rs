use std::cmp::*;
use std::ops::*;
use std::str::FromStr;

mod ops;

use ops::*;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BigInteger {
    digits: Vec<i8>,
    signum: i8,
}

impl BigInteger {
    pub const ZERO: BigInteger = BigInteger { digits: Vec::new(), signum: 0 };

    fn from_parts(digits: Vec<i8>, factor: i8) -> Self {
        let digits = trunc(digits);
        let signum = (min(digits.len(), 1) as i8) * factor;
        BigInteger { digits, signum }
    }

    pub fn last_digit(&self) -> i8 {
        self.digits.last().copied().unwrap_or(0)
    }

    pub fn signum(&self) -> i8 {
        if *self == Self::ZERO {
            0
        } else {
            self.signum
        }
    }

    pub fn abs(self) -> Self {
        match self.signum() {
            -1 => -self,
            _ => self,
        }
    }

    fn len(&self) -> usize {
        self.digits.len()
    }

    fn split_on(self, n: usize) -> (Self, Self) {
        if self.len() <= n {
            return (self, Default::default());
        }
        let mut ls = self.digits;
        let bs = ls.drain(n..).collect();
        (
            BigInteger { digits: ls, signum: self.signum },
            BigInteger { digits: bs, signum: self.signum },
        )
    }
}

impl Default for BigInteger {
    fn default() -> Self {
        Self::ZERO
    }
}

impl FromStr for BigInteger {
    type Err = char;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut signum = 1;
        let digits = s
            .chars()
            .rev()
            .map(|c| match c {
                d @ '0'..='9' => {
                    Ok(unsafe { d.to_digit(10).unwrap_unchecked() } as i8)
                }
                '-' => {
                    if signum == 1 {
                        signum *= -1;
                        Ok(0)
                    } else {
                        Err('-')
                    }
                }
                c => Err(c),
            })
            .collect::<Result<_, _>>()?;
        Ok(Self::from_parts(digits, signum))
    }
}

impl Neg for BigInteger {
    type Output = Self;

    fn neg(mut self) -> Self::Output {
        self.signum *= -1;
        self
    }
}

impl Add for BigInteger {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        let (lhs, rhs) =
            if self.signum >= rhs.signum { (self, rhs) } else { (rhs, self) };
        match (lhs.signum, rhs.signum) {
            (_, 0) => lhs,
            (0, _) => rhs,
            (1, 1) => BigInteger {
                digits: addition(lhs.digits, rhs.digits),
                signum: 1,
            },
            (1, -1) => {
                let (digits, factor) = substraction(lhs.digits, rhs.digits);
                Self::from_parts(digits, factor)
            }
            (-1, -1) => -((-lhs) + (-rhs)),
            _ => unreachable!(),
        }
    }
}

impl Sub for BigInteger {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        self + (-rhs)
    }
}

impl Mul for BigInteger {
    type Output = BigInteger;

    fn mul(self, rhs: Self) -> Self::Output {
        let (l, r) = (self.len(), rhs.len());
        if let (0, _) | (_, 0) = (l, r) {
            return Self::ZERO;
        }
        if let (1, 1) = (l, r) {
            let prod =
                self.digits.first().unwrap() * rhs.digits.first().unwrap();
            return BigInteger::from_parts(vec![prod % 10, prod / 10], 1);
        }

        let n = max(l, r) + max(l, r) % 2;
        let signum = self.signum * rhs.signum;
        let (b, a) = self.abs().split_on(n / 2);
        let (d, c) = rhs.abs().split_on(n / 2);
        let ac = a.clone() * c.clone();
        let bd = b.clone() * d.clone();
        let adbc = (a + b) * (c + d) - ac.clone() - bd.clone();

        let mut r = (ac << n) + ((adbc) << (n / 2)) + bd;
        r.signum = signum;
        r
    }
}

impl Shl<usize> for BigInteger {
    type Output = Self;

    fn shl(mut self, n: usize) -> Self::Output {
        self.digits = lsh(self.digits, n);
        self
    }
}

impl PartialOrd for BigInteger {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(match (self.clone() - other.clone()).signum() {
            1 => Ordering::Greater,
            0 => Ordering::Equal,
            -1 => Ordering::Less,
            _ => unsafe { std::hint::unreachable_unchecked() },
        })
    }
}

impl Ord for BigInteger {
    fn cmp(&self, other: &Self) -> Ordering {
        unsafe { self.partial_cmp(other).unwrap_unchecked() }
    }
}

pub fn division(mut x: BigInteger, y: BigInteger) -> BigInteger {
    assert!(x.signum() >= 0);
    assert!(y.signum() > 0);
    let n = y.len();
    let m = x.len();
    if n > m {
        return BigInteger::ZERO;
    }

    let mut result = Vec::new();
    for i in (0..=(m - n)).into_iter().rev() {
        let z = y.clone() << i;
        let mut c = 0;
        while x >= z {
            x = x - z.clone();
            c += 1
        }
        result.push(c);
        if x == BigInteger::ZERO {
            break;
        }
    }
    result.reverse();

    BigInteger::from_parts(result, 1)
}

macro_rules! big_int {
    ($value:literal) => {
        $value.parse::<BigInteger>().unwrap()
    };
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn add_sub() {
        assert_eq!(big_int!("-42") + big_int!("42"), BigInteger::ZERO);
        assert_eq!(big_int!("120") - big_int!("132"), big_int!("-12"));
    }

    #[test]
    fn mul() {
        assert_eq!(big_int!("42") * big_int!("42"), big_int!("1764"));
        assert_eq!(big_int!("-42") * -big_int!("42"), big_int!("1764"));
        assert_eq!(big_int!("0") * big_int!("42"), big_int!("0"));
        assert_eq!(big_int!("42") * big_int!("1764"), big_int!("74088"));
    }

    #[test]
    fn div() {
        assert_eq!(division(big_int!("141"), big_int!("7")), big_int!("20"));
        assert_eq!(division(big_int!("10"), big_int!("2")), big_int!("5"));
        assert_eq!(division(big_int!("2"), big_int!("42")), big_int!("0"));
        assert_eq!(division(big_int!("121"), big_int!("11")), big_int!("11"));
    }
}
