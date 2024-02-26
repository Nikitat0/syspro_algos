use rand::{random, seq::*, thread_rng};
use std::cmp::*;
use std::time::Instant;

fn quick_sort<T, F>(seq: &mut [T], partition: &F)
where
    T: Ord,
    F: Fn(&mut [T]) -> (usize, usize),
{
    let n = seq.len();
    if n < 2 {
        return;
    }

    let (i, j) = partition(seq);
    let (left, tail) = seq.split_at_mut(i);
    let (_, right) = tail.split_at_mut(j - i);
    quick_sort(left, partition);
    quick_sort(right, partition);
}

fn hoare_partition<T: Ord>(seq: &mut [T]) -> (usize, usize) {
    unsafe {
        let n = seq.len();
        seq.swap(0, random::<usize>() % n);

        let mut i = 0;
        let mut j = seq.len() - 1;
        if seq.get_unchecked(i) > seq.get_unchecked(j) {
            seq.swap(i, j);
        }
        loop {
            i += 1;
            while seq.get_unchecked(i) < seq.get_unchecked(0) {
                i += 1;
            }
            while seq.get_unchecked(j) > seq.get_unchecked(0) {
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
        (i, i + 1)
    }
}

fn lomuto_partition<T: Ord + Copy>(seq: &mut [T]) -> (usize, usize) {
    unsafe {
        let n = seq.len();
        seq.swap(0, random::<usize>() % n);
        if seq.get_unchecked(0) > seq.get_unchecked(n - 1) {
            seq.swap(0, n - 1);
        }

        let mut i = 1;
        while seq.get_unchecked(i) < seq.get_unchecked(0) {
            i += 1;
        }

        let mut j = i + 1;
        while j < n {
            let x = *seq.get_unchecked(j);
            let smaller = (-((x < *seq.get_unchecked(0)) as isize)) as usize;
            let delta = smaller & (j - i);
            *seq.get_unchecked_mut(i + delta) = *seq.get_unchecked(i);
            *seq.get_unchecked_mut(j - delta) = x;
            i -= smaller;
            j += 1;
        }

        i -= 1;
        seq.swap(0, i);

        let mut j = i + 1;
        while j < n && seq.get_unchecked(j) < seq.get_unchecked(i) {
            j += 1;
        }

        let mut k = j + 1;
        while k < n {
            let x = *seq.get_unchecked(k);
            let smaller = (-((x == *seq.get_unchecked(i)) as isize)) as usize;
            let delta = smaller & (k - j);
            *seq.get_unchecked_mut(j + delta) = *seq.get_unchecked(j);
            *seq.get_unchecked_mut(k - delta) = x;
            j -= smaller;
            k += 1;
        }

        (i, j)
    }
}

fn lomuto_partition_nonequal<T: Ord + Copy>(seq: &mut [T]) -> (usize, usize) {
    unsafe {
        let n = seq.len();
        seq.swap(0, random::<usize>() % n);
        if seq.get_unchecked(0) > seq.get_unchecked(n - 1) {
            seq.swap(0, n - 1);
        }

        let mut i = 1;
        while seq.get_unchecked(i) < seq.get_unchecked(0) {
            i += 1;
        }

        let mut j = i + 1;
        while j < n {
            let x = *seq.get_unchecked(j);
            let smaller = (-((x < *seq.get_unchecked(0)) as isize)) as usize;
            let delta = smaller & (j - i);
            *seq.get_unchecked_mut(i + delta) = *seq.get_unchecked(i);
            *seq.get_unchecked_mut(j - delta) = x;
            i -= smaller;
            j += 1;
        }

        i -= 1;
        seq.swap(0, i);

        (i, i + 1)
    }
}

fn main() {
    let mut report = vec![["", "Hoare", "Lomuto", "Lomuto non-equal"]
        .iter()
        .map(ToString::to_string)
        .collect::<Vec<_>>()];

    for i in 0..10 {
        let n = 2_i32.pow(i + 6);
        let mut seq = (0..n).collect::<Vec<_>>();

        let mut hoare_time = 0.0;
        let mut lomuto_time = 0.0;
        let mut lomuto_ne_time = 0.0;
        for _ in 0..512 {
            seq.shuffle(&mut thread_rng());
            let s = Instant::now();
            quick_sort(&mut seq, &hoare_partition);
            hoare_time += s.elapsed().as_secs_f64();

            seq.shuffle(&mut thread_rng());
            let s = Instant::now();
            quick_sort(&mut seq, &lomuto_partition);
            lomuto_time += s.elapsed().as_secs_f64();

            seq.shuffle(&mut thread_rng());
            let s = Instant::now();
            quick_sort(&mut seq, &lomuto_partition_nonequal);
            lomuto_ne_time += s.elapsed().as_secs_f64();
        }
        hoare_time /= 512.0;
        lomuto_time /= 512.0;
        lomuto_ne_time /= 512.0;

        report.push(vec![
            format!("{} elements", n),
            format!("{:.06}", hoare_time),
            format!("{:.06}", lomuto_time),
            format!("{:.06}", lomuto_ne_time),
        ]);
    }

    println!("# Hoare vs Lomuto\n");
    println!("```\n$ cargo run --release\n```\n");
    println!("{}", format_table(report));
}

fn format_table(mut rows: Vec<Vec<String>>) -> String {
    assert!(!rows.is_empty());
    let m = rows[0].len();
    assert!(m > 0);
    assert!(rows.iter().all(|row| row.len() == m));

    let column_widths = (0..m)
        .map(|j| rows.iter().map(|row| row[j].trim().len()).max().unwrap_or(0))
        .collect::<Vec<_>>();

    rows.iter_mut().for_each(|row| {
        row.iter_mut().enumerate().for_each(|(j, e)| {
            *e = format!("| {:<width$} ", e.trim(), width = column_widths[j]);
        });
        row.push("|".to_string());
    });
    rows.insert(
        1,
        (0..m)
            .map(|j| {
                format!(
                    "|{:->width$}",
                    String::new(),
                    width = column_widths[j] + 2
                )
            })
            .collect(),
    );
    rows[1].push("|".to_string());

    rows.into_iter().map(|row| row.join("")).collect::<Vec<_>>().join("\n")
}
