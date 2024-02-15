use std::mem::*;

pub fn merge_sort_inplace<T: Ord + Copy>(seq: &mut [T]) {
    sort_fallback!(seq);
    let n = seq.len();
    let (l, r) = split_equal(seq);
    merge_sort_prosperous(l, r);
    let mut m = n / 2 + n % 2;
    while m > 1 {
        let (l, r) = split_equal(&mut seq[..m]);
        merge_sort_prosperous(r, l);
        let p = m / 2;
        let q = n;
        let mut i = 0;
        let mut j = m;
        let mut k = m / 2 + m % 2;
        while i < p && j < q {
            if seq[i] <= seq[j] {
                seq.swap(i, k);
                i += 1;
            } else {
                seq.swap(j, k);
                j += 1;
            }
            k += 1;
        }
        while i < p {
            seq.swap(i, k);
            i += 1;
            k += 1;
        }
        while j < q {
            seq.swap(j, k);
            j += 1;
            k += 1;
        }
        m = m / 2 + m % 2;
    }
    let mut i = 1;
    while i < n && seq[i - 1] >= seq[i] {
        seq.swap(i - 1, i);
        i += 1
    }
}

fn merge_sort_prosperous<T: Ord + Copy>(a: &mut [T], b: &mut [T]) {
    debug_assert!(a.len() == b.len());
    sort_fallback!(b);
    let n = a.len();
    let (la, ra) = a.split_at_mut(n / 2);
    let (lb, rb) = b.split_at_mut(n / 2);
    merge_sort_prosperous(lb, la);
    merge_sort_prosperous(rb, ra);

    let p = la.len();
    let q = ra.len();
    let mut i = 0;
    let mut j = 0;
    let mut k = 0;
    while i < p && j < q {
        if la[i] <= ra[j] {
            swap(&mut la[i], &mut b[k]);
            i += 1;
        } else {
            swap(&mut ra[j], &mut b[k]);
            j += 1;
        }
        k += 1;
    }
    while i < p {
        swap(&mut la[i], &mut b[k]);
        i += 1;
        k += 1;
    }
    while j < q {
        swap(&mut ra[j], &mut b[k]);
        j += 1;
        k += 1;
    }
}

fn shell_sort<T: Ord>(seq: &mut [T]) {
    shell_sort_k(seq, 7);
    shell_sort_k(seq, 3);
    shell_sort_k(seq, 1);
}

#[inline(always)]
fn shell_sort_k<T: Ord>(seq: &mut [T], k: usize) {
    for i in k..(seq.len()) {
        let mut j = i;
        while seq[j - k] > seq[j] {
            seq.swap(j, j - k);
            j -= k;
            if j < k {
                break;
            }
        }
    }
}

macro_rules! sort_fallback {
    ($seq:ident) => {
        match $seq.len() {
            0..=1 => return,
            2..=15 => {
                shell_sort($seq);
                return;
            }
            _ => {}
        }
    };
}

use sort_fallback;

#[inline(always)]
fn split_equal<T>(seq: &mut [T]) -> (&mut [T], &mut [T]) {
    let n = seq.len();
    let (l, r) = seq.split_at_mut(n / 2);
    let (_, r) = r.split_at_mut(n % 2);
    (l, r)
}

pub fn count_inversions<T: Ord + Copy>(seq: &mut [T]) -> usize {
    let n = seq.len();
    let mut buf = Vec::with_capacity(n);
    unsafe { buf.set_len(n) };
    count_inversion_impl(seq, &mut buf)
}

fn count_inversion_impl<T: Ord + Copy>(seq: &mut [T], buf: &mut [T]) -> usize {
    let n = seq.len();
    if n == 1 {
        return 0;
    }
    let m = n / 2;

    let mut c = 0;
    c += count_inversion_impl(&mut seq[..m], &mut buf[..m]);
    c += count_inversion_impl(&mut seq[m..], &mut buf[..(m + n % 2)]);
    let mut i = 0;
    let mut j = m;
    let mut k = 0;
    while i < m && j < n {
        if seq[i] < seq[j] {
            buf[k] = seq[i];
            i += 1;
        } else {
            c += m - i;
            buf[k] = seq[j];
            j += 1;
        }
        k += 1;
    }
    while i < m {
        buf[k] = seq[i];
        i += 1;
        k += 1;
    }
    while j < n {
        buf[k] = seq[j];
        j += 1;
        k += 1;
    }
    seq.clone_from_slice(buf);
    c
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_shell_sort() {
        let mut nums =
            vec![1, 2, 15, 3, 4, 9, 10, 13, 0, 5, 6, 7, 12, 8, 14, 11];
        assert!(nums.len() == 16);
        shell_sort(&mut nums);
        assert_eq!(nums, (0..16).into_iter().collect::<Vec<_>>())
    }

    #[test]
    fn test_merge_sort_inplace() {
        let mut nums = (0..50).into_iter().rev().collect::<Vec<_>>();
        merge_sort_inplace(&mut nums);
        assert_eq!(nums, (0..50).into_iter().collect::<Vec<_>>());
        let mut nums = std::iter::once(2)
            .cycle()
            .take(50000)
            .into_iter()
            .collect::<Vec<_>>();
        merge_sort_inplace(&mut nums);
    }

    #[test]
    fn test_count_inversion() {
        assert_eq!(count_inversions(&mut vec![2, 1]), 1);
        assert_eq!(count_inversions(&mut vec![3, 4, 1, 2, 5]), 4);
        assert_eq!(count_inversions(&mut vec![3, 4, 2, 5, 1]), 6);
        assert_eq!(count_inversions(&mut vec![1, 2, 3, 4, 5]), 0);
    }
}
