use std::{
    f64::consts::E,
    sync::{Arc, Mutex},
};

//use colored::*;
use rand::{rngs::SmallRng, Rng, SeedableRng};
use rand_distr::{num_traits::Float, Distribution, Normal};

pub struct Student {
    pub real_elo: u32,
    pub estimated_elo: u32,
}

impl Student {
    pub fn new(elo: u32) -> Self {
        Self {
            real_elo: elo,
            estimated_elo: 1600,
        }
    }

    pub fn solve_task(&mut self, task: &Arc<Mutex<Question>>, rng: &mut SmallRng) -> bool {
        let task = &mut task.lock().unwrap();

        let success_probability = expected_probability(self.real_elo as f64, task.real_elo as f64);

        let success = rng.gen_range(0.0..1.0) < success_probability;

        todo!("Adjust ratings accordingly.");

        success
    }
}

pub fn expected_probability(ra: f64, rb: f64) -> f64 {
    1.0 / (1.0 + E.powf(2.0 * (1.0 / 99.0).ln() / rb * (ra - rb / 2.0)))
}

pub struct Question {
    real_elo: u32,
    pub estimated_elo: u32,
}

impl Question {
    pub fn new(elo: u32) -> Self {
        Self {
            real_elo: elo,
            estimated_elo: 1600,
        }
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
