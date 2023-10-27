pub mod question;
pub mod student;

pub use crate::question::Question;
pub use crate::student::Student;

use std::f64::consts::E;

// use colored::Colorize;
use rand::{rngs::SmallRng, SeedableRng};
use rand_distr::{num_traits::Float, Distribution, Normal};

pub fn probability(elo_1: u32, elo_2: u32) -> f64 {
    let ra = elo_1 as f64;
    let rb = elo_2 as f64;

    1.0 / (1.0 + E.powf(2.0 * (1.0 / 99.0).ln() / rb * (ra - rb / 2.0)))
}

fn change(win: bool, k: f64, p: f64) -> f64 {
    match win {
        true => (1.0 - p) * k,
        false => (0.0 - p) * k,
    }
}

pub struct RandomNormalGen {
    pub rng: SmallRng,
    distribution: Normal<f64>,
}

impl RandomNormalGen {
    pub fn new(mean: f64, std_dev: f64) -> Self {
        Self {
            rng: SmallRng::from_entropy(),
            distribution: Normal::new(mean, std_dev).unwrap(),
        }
    }

    pub fn get_u32(&mut self) -> u32 {
        let random_f64: f64 = self.distribution.sample(&mut self.rng);
        let random_u32: u32 = random_f64 as u32;

        random_u32
    }
}
