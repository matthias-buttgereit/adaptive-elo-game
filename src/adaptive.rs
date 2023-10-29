use std::f64::consts::E;

pub fn probability(ra: f64, rb: f64) -> f64 {
    1.0 / (1.0 + E.powf(2.0 * (1.0 / 99.0f64).ln() / rb * (ra - rb / 2.0)))
}

pub fn change(win: bool, k: f64, p: f64) -> f64 {
    match win {
        true => (1.0 - p) * k,
        false => (0.0 - p) * k,
    }
}
