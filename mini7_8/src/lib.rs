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
    fn test_count_inversion() {
        assert_eq!(count_inversions(&mut vec![2, 1]), 1);
        assert_eq!(count_inversions(&mut vec![3, 4, 1, 2, 5]), 4);
        assert_eq!(count_inversions(&mut vec![3, 4, 2, 5, 1]), 6);
        assert_eq!(count_inversions(&mut vec![1, 2, 3, 4, 5]), 0);
    }
}
