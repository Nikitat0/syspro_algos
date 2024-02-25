mod bench;

use std::cmp::*;

use bench::*;
use matrix_mul::*;

fn main() {
    do_experimental_bench();
    println!();
    do_showcase_bench();
}

fn do_experimental_bench() {
    let stand = bench_stand![
        |(a, b): (Matrix, Matrix)| {
            let _ = a.clone() * b.clone();
        },
        |(a, b): (Matrix, Matrix)| {
            recursive_mul(&a, &b);
        },
        |(a, b)| {
            strassen_mul(&a, &b, 1);
        },
    ];

    let mut report: Vec<Vec<String>> = Vec::new();
    report.push(
        ["", "Trivial", "Recursive", "Strassen"]
            .iter()
            .map(ToString::to_string)
            .collect(),
    );
    for p in 0..8 {
        let n = 2_usize.pow(p + 1);
        let iterations = max(0x1000 / 4_usize.pow(p), 0x10);

        let a = Matrix::new_zeroed(n);
        let stats = stand.run((a.clone(), a), iterations);
        report.push(stats.iter().map(ToString::to_string).collect());
        report.last_mut().unwrap().insert(0, format!("Matrix {n:}x{n:}"));
    }

    println!("# Experimental bench\n");
    println!("{}", format_table(report));
}

fn do_showcase_bench() {
    let stand = bench_stand![
        |(a, b): (Matrix, Matrix)| {
            let _ = a.clone() * b.clone();
        },
        |(a, b)| {
            strassen_mul(&a, &b, 64);
        },
    ];

    let mut report: Vec<Vec<String>> = Vec::new();
    report.push(
        ["", "Trivial", "Strassen (fallback on 64x64)"]
            .iter()
            .map(ToString::to_string)
            .collect(),
    );
    for p in 0..12 {
        let n = 2_usize.pow(p + 1);
        let iterations = max(0x1000 / 4_usize.pow(p), 0x10);

        let a = Matrix::new_zeroed(n);
        let stats = stand.run((a.clone(), a), iterations);
        report.push(stats.iter().map(ToString::to_string).collect());
        report.last_mut().unwrap().insert(0, format!("Matrix {n:}x{n:}"));
    }

    println!("# Showcase bench\n");
    println!("{}", format_table(report));
}

pub fn format_table(mut rows: Vec<Vec<String>>) -> String {
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
