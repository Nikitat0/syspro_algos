use std::cmp::*;

#[macro_export()]
macro_rules! swap_by_len {
    ($x:ident, $y: ident) => {
        if $x.len() < $y.len() {
            ($y, $x)
        } else {
            ($x, $y)
        }
    };
}

pub fn lsh(x: Vec<i8>, n: usize) -> Vec<i8> {
    let mut r = Vec::new();
    r.resize(x.len() + n, 0);
    for (i, d) in x.into_iter().enumerate() {
        r[i + n] = d;
    }
    r
}

pub fn addition(x: Vec<i8>, y: Vec<i8>) -> Vec<i8> {
    let (mut x, y) = swap_by_len!(x, y);
    for (i, d) in y.iter().copied().enumerate() {
        x[i] += d
    }
    carry(x)
}

pub fn substraction(mut x: Vec<i8>, y: Vec<i8>) -> (Vec<i8>, i8) {
    x.resize(max(x.len(), y.len()), 0);
    for (i, d) in y.iter().copied().enumerate() {
        x[i] -= d
    }
    let mut c = 0;
    for d in x.iter_mut() {
        *d += c;
        if *d < 0 {
            *d += 10;
            c = -1;
        } else {
            c = 0;
        }
    }
    if c != 0 {
        let (x, a) = substraction(lsh(vec![1], x.len()), x);
        assert!(a == 1);
        (x, -1)
    } else {
        (carry(x), 1)
    }
}

pub fn carry(mut x: Vec<i8>) -> Vec<i8> {
    let mut carry = 0;
    for d in x.iter_mut() {
        *d += carry;
        carry = *d / 10;
        *d %= 10;
    }
    if carry != 0 {
        x.push(carry)
    }
    x
}

pub fn trunc(mut x: Vec<i8>) -> Vec<i8> {
    while let Some(0) = x.last() {
        x.pop();
    }
    x
}
