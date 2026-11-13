use crate::utils::gen_exp1;
use std::{cmp::Ordering::Less, collections::VecDeque};

pub struct Stats {
    pub mean_response_time: f64,
    pub mean_jobs_in_system: f64,
    pub server_utilization: f64,
}

pub fn simulate(lambda: f64, mu: f64, num_jobs: u64) -> Stats {
    let mut clock = 0.0;
    let mut job_q = VecDeque::new();
    let mut next_arrival = gen_exp1() / lambda;
    let mut next_complete = f64::MAX;

    let mut completed_cnt = 0;
    #[allow(non_snake_case)]
    let mut running_mean_T = 0.0;
    #[allow(non_snake_case)]
    let mut running_mean_N = 0.0;
    let mut server_busy_cnt = 0;

    while completed_cnt < num_jobs {
        match next_arrival.total_cmp(&next_complete) {
            Less => {
                running_mean_N +=
                    (job_q.len() as f64 - running_mean_N) / next_arrival * (next_arrival - clock);

                clock = next_arrival;
                if job_q.is_empty() {
                    next_complete = clock + gen_exp1() / mu;
                } else {
                    server_busy_cnt += 1;
                }
                job_q.push_back(clock);

                next_arrival = clock + gen_exp1() / lambda;
            }
            _ => {
                running_mean_N +=
                    (job_q.len() as f64 - running_mean_N) / next_complete * (next_complete - clock);

                clock = next_complete;
                let arrived = job_q
                    .pop_front()
                    .expect("invariant ensures there must be a job in the queue");
                completed_cnt += 1;
                running_mean_T += ((clock - arrived) - running_mean_T) / (completed_cnt as f64);

                next_complete = if job_q.is_empty() {
                    f64::MAX
                } else {
                    clock + gen_exp1() / mu
                };
            }
        }
    }

    Stats {
        mean_response_time: running_mean_T,
        mean_jobs_in_system: running_mean_N,
        server_utilization: server_busy_cnt as f64 / (num_jobs as f64 + job_q.len() as f64),
    }
}
