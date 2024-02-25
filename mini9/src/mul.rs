use super::*;
use std::cmp::*;

pub fn recursive_mul(x: &Matrix, y: &Matrix) -> Matrix {
    let size = size_assert!(x, y);
    let new_size = size_for_multiplication(size, 1);
    recursive_mul_impl(x.expand(new_size).borrow(), y.expand(new_size).borrow())
        .shrink(size)
}

fn recursive_mul_impl(x: &Matrix, y: &Matrix) -> Matrix {
    if x.size == 1 {
        return x.clone() * y.clone();
    }

    let [a, b, c, d] = x.submatrices();
    let [e, f, g, h] = y.submatrices();
    let blocks = [
        recursive_mul_impl(&a, &e) + recursive_mul_impl(&b, &g),
        recursive_mul_impl(&a, &f) + recursive_mul_impl(&b, &h),
        recursive_mul_impl(&c, &e) + recursive_mul_impl(&d, &g),
        recursive_mul_impl(&c, &f) + recursive_mul_impl(&d, &h),
    ];
    Matrix::from_blocks(blocks)
}

pub fn strassen_mul(x: &Matrix, y: &Matrix, fallback: usize) -> Matrix {
    let size = size_assert!(x, y);
    let new_size = size_for_multiplication(size, fallback);
    strassen_mul_impl(
        x.expand(new_size).borrow(),
        y.expand(new_size).borrow(),
        fallback,
    )
    .shrink(size)
}

fn strassen_mul_impl(x: &Matrix, y: &Matrix, fallback: usize) -> Matrix {
    if size_assert!(x, y) == fallback {
        return x.clone() * y.clone();
    }

    let [a, b, c, d] = x.submatrices();
    let [e, f, g, h] = y.submatrices();
    let p: [_; 7] = [
        strassen_mul_impl(&a, (&f - &h).borrow(), fallback),
        strassen_mul_impl((&a + &b).borrow(), &h, fallback),
        strassen_mul_impl((&c + &d).borrow(), &e, fallback),
        strassen_mul_impl(&d, (&g - &e).borrow(), fallback),
        strassen_mul_impl((&a + &d).borrow(), (&e + &h).borrow(), fallback),
        strassen_mul_impl((&b - &d).borrow(), (&g + &h).borrow(), fallback),
        strassen_mul_impl((&a - &c).borrow(), (&e + &f).borrow(), fallback),
    ];
    let blocks = [
        &p[4] + &p[3] - &p[1] + &p[5],
        &p[0] + &p[1],
        &p[2] + &p[3],
        &p[0] + &p[4] - &p[2] - &p[6],
    ];
    Matrix::from_blocks(blocks)
}

fn size_for_multiplication(size: usize, fallback: usize) -> usize {
    let mut new_size = min(size, fallback);
    while new_size < size {
        new_size *= 2
    }
    new_size
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_recursive_mul() {
        let a = matrix![[1, 2], [3, 4]];
        let b = matrix![[5, 6], [7, 8]];
        assert_eq!(recursive_mul(&a, &b), matrix![[19, 22], [43, 50]]);
        let a = matrix![[1, 0, 1], [0, 1, 0], [1, 0, 1]];
        assert_eq!(
            recursive_mul(&a, &a),
            matrix![[2, 0, 2], [0, 1, 0], [2, 0, 2]]
        );
    }

    #[test]
    fn test_strassen_mul() {
        let a = matrix![[1, 2], [3, 4]];
        let b = matrix![[5, 6], [7, 8]];
        assert_eq!(strassen_mul(&a, &b, 1), matrix![[19, 22], [43, 50]]);
        let c = matrix![[1, 0, 1], [0, 1, 0], [1, 0, 1]];
        assert_eq!(
            strassen_mul(&c, &c, 1),
            matrix![[2, 0, 2], [0, 1, 0], [2, 0, 2]]
        );
    }
}
