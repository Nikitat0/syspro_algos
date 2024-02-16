use std::borrow::Borrow;

pub fn ascii_radix_sort<S: Borrow<str>>(seq: &mut [S]) {
    let n = seq.iter().map(|s| s.borrow().as_bytes().len()).max().unwrap_or(0);
    for i in (0..n).into_iter().rev() {
        count_sort(seq, |s| s.borrow().as_bytes().get(i).copied().unwrap_or(0));
    }
}

fn count_sort<T, F: Fn(&T) -> u8>(seq: &mut [T], key_of: F) {
    let key_of = |item: &T| key_of(item) as usize;
    let mut counters = vec![0; 0xFF];
    let n = seq.len();
    for item in &*seq {
        let k = key_of(item);
        counters[k] += 1;
    }
    for i in 1..(0xFF) {
        counters[i] += counters[i - 1];
    }

    let mut buf = Vec::<T>::with_capacity(n);
    unsafe {
        buf.set_len(n);
        for item in seq.iter().rev().map(|item| std::ptr::read(item as _)) {
            let k = key_of(&item);
            counters[k] -= 1;
            buf[counters[k]] = item;
        }
    }

    for (i, item) in buf.into_iter().enumerate() {
        unsafe {
            std::ptr::write(seq.get_unchecked_mut(i), item);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ascii_radix_sort() {
        let mut strs = vec!["group", "space", "ring", "field"];
        ascii_radix_sort(&mut strs);
        assert_eq!(strs, vec!["field", "group", "ring", "space"]);
    }

    #[test]
    fn test_count_sort() {
        let mut nums = vec![2, 1, 1, 2, 3, 0];
        count_sort(&mut nums, |n| *n);
        assert_eq!(nums, vec![0, 1, 1, 2, 2, 3]);
        let mut strs = vec!["cdf", "bcd", "abc"];
        count_sort(&mut strs, |s| s.as_bytes()[0]);
        assert_eq!(nums, vec![0, 1, 1, 2, 2, 3]);
    }
}
