use rand;
use std::io;

fn hoare_partition<T: Ord>(seq: &mut [T]) -> usize {
    let n = seq.len();
    if n < 2 {
        return 0;
    }

    seq.swap(0, rand::random::<usize>() % n);

    let mut i = 0;
    let mut j = n - 1;
    if seq[i] > seq[j] {
        seq.swap(i, j);
    }
    loop {
        i += 1;
        while seq[i] < seq[0] {
            i += 1;
        }
        while seq[j] > seq[0] {
            j -= 1;
        }
        if i >= j {
            break;
        }
        seq.swap(i, j);
        j -= 1;
    }

    i -= 1;
    seq.swap(0, i);
    i
}

fn k_stat<T: Ord>(seq: &mut [T], k: usize) -> &T {
    let n = seq.len();
    debug_assert!(k < n);

    if n < 2 {
        seq.sort();
        return &seq[k];
    }

    let i = hoare_partition(seq);
    if i > k {
        k_stat(&mut seq[0..i], k)
    } else if i < k {
        k_stat(&mut seq[(i + 1)..], k - i - 1)
    } else {
        &seq[k]
    }
}

fn solution(seq: &mut [i64]) -> i64 {
    *k_stat(seq, seq.len() / 2)
}

fn main() {
    let mut oil_wells: Vec<i64> = {
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        input
    }
    .split_ascii_whitespace()
    .map(|e| e.parse().unwrap())
    .collect();

    oil_wells.sort();
    println!("Best place: {}", solution(&mut oil_wells));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solution() {
        assert_eq!(solution(&mut vec![42]), 42);
        assert_eq!(solution(&mut vec![-1, 1]).abs(), 1);
        assert_eq!(solution(&mut vec![1, 2, 3, 5]), 3);
        assert_eq!(solution(&mut vec![-2, -1, 1, 2]).abs(), 1);
        assert_eq!(solution(&mut vec![-3, -1, 1, 2]).abs(), 1);
        assert_eq!(solution(&mut vec![1, 2, 3, 4, 5]), 3);
    }
}
