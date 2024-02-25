use std::fmt::{self, Display};
use std::time::*;

#[derive(Debug, Clone, Copy)]
pub struct BenchStats {
    sample_mean: f64,
    standart_deviation: f64,
    geometric_mean: f64,
}

impl BenchStats {
    pub fn from_sample(sample: Vec<f64>) -> Self {
        let n = sample.len() as f64;
        let sample_mean = sample.iter().copied().sum::<f64>() / n;
        let standart_deviation = sample
            .iter()
            .copied()
            .map(|x| (x - sample_mean).powi(2))
            .sum::<f64>()
            / n;
        let geometric_mean =
            (sample.iter().copied().map(f64::ln).sum::<f64>() / n).exp();
        BenchStats { sample_mean, standart_deviation, geometric_mean }
    }
}

impl Display for BenchStats {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{:>8.6}, {:.6}, {:>8.6}",
            self.sample_mean, self.standart_deviation, self.geometric_mean,
        )
    }
}

pub fn bench<F: Fn()>(proc: F, iterations: usize) -> BenchStats {
    let runs = (0..iterations)
        .map(|_| {
            let s = Instant::now();
            proc();
            s.elapsed().as_secs_f64()
        })
        .collect::<Vec<_>>();
    BenchStats::from_sample(runs)
}

pub struct BenchStand<T, const N: usize>(pub [Box<dyn Fn(T) + 'static>; N]);

impl<T: Clone, const N: usize> BenchStand<T, N> {
    pub fn run(&self, dataset: T, iterations: usize) -> [BenchStats; N] {
        self.0
            .iter()
            .map(|algo| bench(|| algo(dataset.clone()), iterations))
            .collect::<Vec<_>>()
            .try_into()
            .unwrap()
    }
}

#[macro_export]
macro_rules! bench_stand {
    [$($algo:expr),+] => (BenchStand([$(Box::new($algo)),+]));
    [$($algo:expr),+,] => (bench_stand![$($algo),+]);
}
