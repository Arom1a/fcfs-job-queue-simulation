mod utils;

use utils::*;

const NUM_EXPERIMENTS: f64 = 1.0;

fn main() {
    let mut sum: f64 = 0.0;
    for _ in 0..NUM_EXPERIMENTS as _ {
        let k = gen_bounded_pareto(332.067, 1e10, 1.1);
        sum += k;
        print!("{}, ", k);
    }
    println!(
        "\n\nAverage over {} experiments: {}",
        NUM_EXPERIMENTS,
        (sum as f64) / NUM_EXPERIMENTS
    );
}
