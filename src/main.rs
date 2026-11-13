mod utils;

use utils::*;

fn main() {
    let n: u64 = std::env::args()
        .nth(1)
        .and_then(|s| s.parse().ok())
        .unwrap_or(100_000);

    let mut sum: f64 = 0.0;
    for _ in 0..n {
        sum += gen_bounded_pareto(332.067, 1e10, 1.1);
    }
    let avg = sum / n as f64;
    println!("Average over {} experiments: {}", n, avg);
}
