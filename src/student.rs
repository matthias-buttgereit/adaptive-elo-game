use std::sync::{Arc, Mutex};

use rand::{rngs::SmallRng, Rng, SeedableRng};
use rand_distr::{Distribution, Normal};

use crate::{adaptive::probability, Question};

pub struct Student {
    pub real_elo: f64,
    pub estimated_elo: f64,
    pub age: usize,
}

impl Student {
    pub fn new(elo: f64) -> Self {
        Self {
            real_elo: elo,
            estimated_elo: 800.0,
            age: 0,
        }
    }

    pub fn get_n_random_students(n: usize, mean: f64, std_dev: f64) -> Vec<Self> {
        let mut students = Vec::with_capacity(n);
        let mut rng = SmallRng::from_entropy();
        let distr = Normal::new(mean, std_dev).unwrap();

        for _ in 0..n {
            let random_elo: f64 = distr.sample(&mut rng);

            students.push(Self::new(random_elo));
        }
        students
    }

    pub fn solve_task(&mut self, task: &Arc<Mutex<Question>>, rng: &mut SmallRng) -> bool {
        let task: &mut Question = &mut task.lock().unwrap();

        let actual_probability: f64 = probability(self.real_elo, task.real_elo);

        let success: bool = rng.gen_range(0.0..1.0) < actual_probability;

        success
    }
}
