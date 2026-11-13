#![allow(dead_code)]

use rand::rngs::ThreadRng;

pub fn gen_uni01(mut rng: &mut ThreadRng) -> f64 {
    use rand::distr::{Distribution, Open01};
    let open01 = Open01;
    open01.sample(&mut rng)
}

pub fn gen_exp1(rng: &mut ThreadRng) -> f64 {
    -gen_uni01(rng).ln()
}

pub fn gen_poisson1(rng: &mut ThreadRng) -> u64 {
    let mut t = 0.0;
    let mut count = 0;
    loop {
        t += gen_exp1(rng);
        if t > 1.0 {
            return count;
        }
        count += 1;
    }
}

pub fn gen_geom(p: f64, rng: &mut ThreadRng) -> u64 {
    let u = gen_uni01(rng);
    let ret = (1.0 - u).log10() / (1.0 - p).log10();
    ret.ceil() as _
}

pub fn gen_bounded_pareto(lo: f64, hi: f64, alpha: f64, rng: &mut ThreadRng) -> f64 {
    let u = gen_uni01(rng);
    let c = 1.0 - (lo / hi).powf(alpha);
    lo / (1.0 - u * c).powf(1.0 / alpha)
}
