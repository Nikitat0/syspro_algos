use std::borrow::Borrow;
use std::cmp::*;
use std::fmt::{self, Debug};
use std::ops::*;

mod mul;
pub use mul::*;

#[derive(Clone, PartialEq, Eq)]
pub struct Matrix {
    size: usize,
    elements: Box<[i64]>,
}

impl Matrix {
    fn new_zeroed(size: usize) -> Self {
        assert!(size > 0);
        Self { size, elements: vec![0; size * size].into_boxed_slice() }
    }

    unsafe fn new_uninit(size: usize) -> Self {
        assert!(size > 0);
        let mut elements = Vec::with_capacity(size * size);
        elements.set_len(size * size);
        Self { size, elements: elements.into_boxed_slice() }
    }

    pub fn from_rows(rows: Vec<Vec<i64>>) -> Matrix {
        let size = rows.len();
        assert!(size > 0);
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

    pub fn from_blocks(blocks: [Matrix; 4]) -> Matrix {
        let n = blocks[0].size;
        assert!(blocks.iter().all(|m| m.size == n));

        let mut result = unsafe { Matrix::new_uninit(n * 2) };
        for (k, m) in blocks.into_iter().enumerate() {
            let sh = (k & 1) * n;
            let sv = (k >> 1) * n;
            for i in 0..n {
                for j in 0..n {
                    result[i + sv][j + sh] = m[i][j];
                }
            }
        }
        result
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

    pub fn expand(&mut self, base: usize) {
        let n = self.size;
        let mut m = min(n, base);
        while m < n {
            m *= 2;
        }

        let mut expanded = Matrix::new_zeroed(m);
        for i in 0..n {
            for j in 0..n {
                expanded[i][j] = self[i][j];
            }
        }
        *self = expanded;
    }

    pub fn shrink(&mut self, new_size: usize) {
        assert!(self.size >= new_size);

        let mut shrank = Matrix::new_zeroed(new_size);
        for i in 0..new_size {
            for j in 0..new_size {
                shrank[i][j] = self[i][j];
            }
        }
        *self = shrank;
    }

    pub fn submatrices(self) -> [Self; 4] {
        let n = self.size;
        assert!(n % 2 == 0);

        (0..4)
            .into_iter()
            .map(|k| {
                let sh = (k & 1) * n / 2;
                let sv = (k >> 1) * n / 2;
                let mut m = unsafe { Matrix::new_uninit(n / 2) };
                for i in 0..n / 2 {
                    for j in 0..n / 2 {
                        m[i][j] = self[i + sv][j + sh];
                    }
                }
                m
            })
            .collect::<Vec<_>>()
            .try_into()
            .unwrap()
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
    type Output = [i64];

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

impl<T: Borrow<Matrix>> Add<T> for Matrix {
    type Output = Matrix;

    fn add(self, rhs: T) -> Self::Output {
        let rhs = rhs.borrow();

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

impl<T: Borrow<Matrix>> Add<T> for &Matrix {
    type Output = Matrix;

    fn add(self, rhs: T) -> Self::Output {
        self.clone() + rhs
    }
}

impl<T: Borrow<Matrix>> Sub<T> for Matrix {
    type Output = Matrix;

    fn sub(self, rhs: T) -> Self::Output {
        self + (rhs.borrow().clone() * -1)
    }
}

impl<T: Borrow<Matrix>> Sub<T> for &Matrix {
    type Output = Matrix;

    fn sub(self, rhs: T) -> Self::Output {
        self.clone() - rhs
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

impl Mul<i64> for Matrix {
    type Output = Matrix;

    fn mul(mut self, scalar: i64) -> Self::Output {
        let n = self.size;
        for i in 0..n {
            for j in 0..n {
                self[i][j] *= scalar
            }
        }
        self
    }
}

#[macro_export]
macro_rules! matrix {
    [$([$($e:literal),+]),*] => (Matrix::from_rows(vec![$(vec![$($e),+]),*]))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add() {
        let a = matrix![[1, 2], [3, 4]];
        let b = matrix![[5, 6], [7, 8]];
        assert_eq!(a + b, matrix![[6, 8], [10, 12]]);
    }

    #[test]
    fn test_mul() {
        let a = matrix![[1, 2], [3, 4]];
        let b = matrix![[5, 6], [7, 8]];
        assert_eq!(a * b, matrix![[19, 22], [43, 50]]);
    }

    #[test]
    fn test_submatrices() {
        let m = matrix![[1, 2], [3, 4]];
        assert_eq!(
            m.submatrices(),
            [matrix![[1]], matrix![[2]], matrix![[3]], matrix![[4]]]
        );
    }

    #[test]
    fn test_concat() {
        let submatrices =
            [matrix![[1]], matrix![[2]], matrix![[3]], matrix![[4]]];
        assert_eq!(Matrix::from_blocks(submatrices), matrix![[1, 2], [3, 4]]);
    }

    #[test]
    fn test_expand() {
        let mut m = matrix![[1, 2, 3], [4, 5, 6], [7, 8, 9]];
        m.expand(1);
        assert_eq!(
            m,
            matrix![[1, 2, 3, 0], [4, 5, 6, 0], [7, 8, 9, 0], [0, 0, 0, 0]]
        )
    }

    #[test]
    fn test_shrink() {
        let mut m = matrix![[1, 2, 0], [3, 4, 0], [0, 0, 0]];
        m.shrink(2);
        assert_eq!(m, matrix![[1, 2], [3, 4]])
    }
}
