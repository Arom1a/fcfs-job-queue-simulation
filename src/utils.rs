pub fn gen_uni01() -> f64 {
    use rand::distr::{Distribution, Open01};
    let mut rng = rand::rng();
    let open01 = Open01;
    open01.sample(&mut rng)
}

pub fn gen_exp1() -> f64 {
    todo!()
}

pub fn gen_geom(p: f64) -> u64 {
    let u = gen_uni01();
    let ret = (1.0 - u).log10() / (1.0 - p).log10();
    ret.ceil() as _
}

pub fn gen_bounded_pareto(lo: f64, hi: f64, alpha: f64) -> f64 {
    let u = gen_uni01();
    let c = 1.0 - (lo / hi).powf(alpha);
    lo / (1.0 - u * c).powf(1.0 / alpha)
}
