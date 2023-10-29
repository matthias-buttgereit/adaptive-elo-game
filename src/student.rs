use std::{
    f64::consts::E,
    sync::{Arc, Mutex},
};

use rand::{rngs::SmallRng, Rng, SeedableRng};
use rand_distr::{Distribution, Normal};

use crate::{
    adaptive::{change, probability},
    Question,
};

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

        let expected_probability: f64 = probability(self.estimated_elo, task.estimated_elo);
        let actual_probability: f64 = probability(self.real_elo, task.real_elo);

        let success: bool = rng.gen_range(0.0..1.0) < actual_probability;

        // Adjust student rating
        let change: f64 = change(success, 40.0, expected_probability);
        self.estimated_elo += change;

        // Adjust question rating
        let young_learning_bonus: f64 = 1.0 + E.powf(-0.1 * task.age as f64);
        let change = change * young_learning_bonus;
        task.estimated_elo -= change;

        success
    }
}
