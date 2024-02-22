use std::fmt::{self, Debug};
use std::ops::*;

#[derive(Clone, PartialEq, Eq)]
pub struct Matrix {
    size: usize,
    elements: Box<[u64]>,
}

impl Matrix {
    unsafe fn new_uninit(size: usize) -> Self {
        let mut elements = Vec::with_capacity(size * size);
        elements.set_len(size * size);
        Self { size, elements: elements.into_boxed_slice() }
    }

    pub fn from_rows(rows: Vec<Vec<u64>>) -> Matrix {
        let size = rows.len();
        assert!(rows.iter().map(Vec::len).all(|n| n == size));
        Matrix {
            size,
            elements: rows
                .into_iter()
                .flatten()
                .collect::<Vec<_>>()
                .into_boxed_slice(),
        }
    }

    fn size_assert(&self, other: &Matrix) -> usize {
        assert_eq!(self.size, other.size);
        self.size
    }

    pub fn transpose(&mut self) {
        for i in 0..self.size {
            for j in (i + 1)..self.size {
                let e = self[i][j];
                self[i][j] = self[j][i];
                self[j][i] = e;
            }
        }
    }
}

impl Debug for Matrix {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let rows: Vec<_> =
            (0..self.size).into_iter().map(|i| &self[i]).collect();
        rows.fmt(f)
    }
}

impl Index<usize> for Matrix {
    type Output = [u64];

    #[inline(always)]
    fn index(&self, row: usize) -> &Self::Output {
        &self.elements[(row * self.size)..((row + 1) * self.size)]
    }
}

impl IndexMut<usize> for Matrix {
    #[inline(always)]
    fn index_mut(&mut self, row: usize) -> &mut Self::Output {
        &mut self.elements[(row * self.size)..((row + 1) * self.size)]
    }
}

impl Add for Matrix {
    type Output = Matrix;

    fn add(self, rhs: Self) -> Self::Output {
        let size = self.size_assert(&rhs);
        let mut sum = unsafe { Matrix::new_uninit(size) };
        for i in 0..size {
            for j in 0..size {
                sum[i][j] = self[i][j] + rhs[i][j];
            }
        }
        sum
    }
}

impl Mul for Matrix {
    type Output = Matrix;

    fn mul(self, mut rhs: Self) -> Self::Output {
        let size = self.size_assert(&rhs);
        rhs.transpose();
        let mut prod = unsafe { Matrix::new_uninit(self.size) };
        for i in 0..size {
            for j in 0..size {
                prod[i][j] =
                    (0..size).into_iter().map(|k| self[i][k] * rhs[j][k]).sum();
            }
        }
        prod
    }
}

pub fn 

#[macro_export]
macro_rules! matrix {
    [$([$($e:literal),+]),*] => (Matrix::from_rows(vec![$(vec![$($e),+]),*]))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    pub fn test_add() {
        let a = matrix![[1, 2], [3, 4]];
        let b = matrix![[5, 6], [7, 8]];
        assert_eq!(a + b, matrix![[6, 8], [10, 12]]);
    }

    #[test]
    pub fn test_prod() {
        let a = matrix![[1, 2], [3, 4]];
        let b = matrix![[5, 6], [7, 8]];
        assert_eq!(a * b, matrix![[19, 22], [43, 50]]);
    }
}
