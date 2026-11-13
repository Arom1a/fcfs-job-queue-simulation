mod queue_simulation;
mod utils;

use queue_simulation::*;

const MU: f64 = 1.0;
const LAMBDAS: [f64; 5] = [0.5, 0.6, 0.7, 0.8, 0.9];

fn main() {
    let n: u64 = std::env::args()
        .nth(1)
        .and_then(|s| s.parse().ok())
        .unwrap_or(100_000_000);

    println!("Running simulation with mu={MU} for {n} jobs");
    println!("lambda,E[T],E[N],rho");

    for lambda in LAMBDAS {
        let t = std::time::Instant::now();
        let s = simulate(lambda, MU, n);
        println!(
            "{},{},{},{}",
            lambda, s.mean_response_time, s.mean_jobs_in_system, s.server_utilization
        );

        println!("{:?}", t.elapsed());
    }
}
