mod utils;

use utils::*;

const NUM_EXPERIMENTS: u64 = 99999;

fn main() {
    let mut sum: u64 = 0;
    for _ in 0..NUM_EXPERIMENTS {
        let k = gen_geom(0.2);
        sum += k;
        // print!("{}, ", k);
    }
    println!();
    println!(
        "Average over {} experiments: {}",
        NUM_EXPERIMENTS,
        (sum as f64) / (NUM_EXPERIMENTS as f64)
    );
}
