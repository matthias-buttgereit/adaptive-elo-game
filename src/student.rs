use std::{
    f64::consts::E,
    sync::{Arc, Mutex},
};

use rand::{rngs::SmallRng, Rng};

use crate::{change, probability, Question};

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
