use std::{
    f64::consts::E,
    sync::{Arc, Mutex},
};

// use colored::Colorize;
use rand::{rngs::SmallRng, Rng, SeedableRng};
use rand_distr::{num_traits::Float, Distribution, Normal};

pub struct Student {
    pub real_elo: u32,
    pub estimated_elo: u32,
    pub age: usize,
}

impl Student {
    pub fn new(elo: u32) -> Self {
        Self {
            real_elo: elo,
            estimated_elo: 800,
            age: 0,
        }
    }

    pub fn solve_task(&mut self, task: &Arc<Mutex<Question>>, rng: &mut SmallRng) -> bool {
        let task = &mut task.lock().unwrap();

        let expected_probability = probability(self.estimated_elo, task.estimated_elo);
        let actual_probability = probability(self.real_elo, task.real_elo);

        let success = rng.gen_range(0.0..1.0) < actual_probability;

        let young_learning_bonus = 1.0 + E.powf(-0.1 * task.age as f64);

        let change = change(success, 40.0, expected_probability) as i32;
        self.estimated_elo = (self.estimated_elo as i32 + change) as u32;

        let change = (change as f64 * young_learning_bonus) as i32;
        task.estimated_elo = (task.estimated_elo as i32 - change) as u32;

        success
    }
}

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

pub struct Question {
    pub real_elo: u32,
    pub estimated_elo: u32,
    pub age: usize,
}

impl Question {
    pub fn new(elo: u32) -> Self {
        Self {
            real_elo: elo,
            estimated_elo: 1600,
            age: 0,
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
