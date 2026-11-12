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
