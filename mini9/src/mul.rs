use super::*;

pub fn recursive_mul<M1, M2>(x: M1, y: M2) -> Matrix
where
    M1: std::borrow::Borrow<Matrix>,
    M2: std::borrow::Borrow<Matrix>,
{
    let mut x = x.borrow().clone();
    let mut y = y.borrow().clone();
    let n = x.size_assert(&y);
    x.expand(1);
    y.expand(1);
    let mut prod = recursive_mul_impl(x, y);
    prod.shrink(n);
    prod
}

pub fn recursive_mul_impl(x: Matrix, y: Matrix) -> Matrix {
    let n = x.size_assert(&y);
    if n == 1 {
        return Matrix {
            size: 1,
            elements: vec![x[0][0] * y[0][0]].into_boxed_slice(),
        };
    }

    let mut x = x.clone();
    let mut y = y.clone();
    x.expand(1);
    y.expand(1);
    let [a, b, c, d] = x.submatrices();
    let [e, f, g, h] = y.submatrices();
    let blocks = [
        recursive_mul_impl(a.clone(), e.clone())
            + recursive_mul_impl(b.clone(), g.clone()),
        recursive_mul_impl(a, f.clone()) + recursive_mul_impl(b, h.clone()),
        recursive_mul_impl(c.clone(), e) + recursive_mul_impl(d.clone(), g),
        recursive_mul_impl(c, f) + recursive_mul_impl(d, h),
    ];
    Matrix::from_blocks(blocks)
}

pub fn strassen_mul<M1, M2>(x: M1, y: M2, fallback: usize) -> Matrix
where
    M1: std::borrow::Borrow<Matrix>,
    M2: std::borrow::Borrow<Matrix>,
{
    let mut x = x.borrow().clone();
    let mut y = y.borrow().clone();
    let n = x.size_assert(&y);
    x.expand(fallback);
    y.expand(fallback);
    let mut prod = strassen_mul_impl(x, y, fallback);
    prod.shrink(n);
    prod
}

pub fn strassen_mul_impl(x: Matrix, y: Matrix, fallback: usize) -> Matrix {
    if x.size_assert(&y) == fallback {
        return x * y;
    }

    let [a, b, c, d] = x.submatrices();
    let [e, f, g, h] = y.submatrices();
    let p: [_; 7] = [
        strassen_mul_impl(a.clone(), &f - &h, fallback),
        strassen_mul_impl(&a + &b, h.clone(), fallback),
        strassen_mul_impl(&c + &d, e.clone(), fallback),
        strassen_mul_impl(d.clone(), &g - &e, fallback),
        strassen_mul_impl(&a + &d, &e + &h, fallback),
        strassen_mul_impl(&b - &d, &g + &h, fallback),
        strassen_mul_impl(&a - &c, &e + &f, fallback),
    ];
    let blocks = [
        &p[4] + &p[3] - &p[1] + &p[5],
        &p[0] + &p[1],
        &p[2] + &p[3],
        &p[0] + &p[4] - &p[2] - &p[6],
    ];
    Matrix::from_blocks(blocks)
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
        assert_eq!(strassen_mul(a, b, 1), matrix![[19, 22], [43, 50]]);
        let a = matrix![[1, 0, 1], [0, 1, 0], [1, 0, 1]];
        assert_eq!(
            strassen_mul(&a, &a, 1),
            matrix![[2, 0, 2], [0, 1, 0], [2, 0, 2]]
        );
    }
}
