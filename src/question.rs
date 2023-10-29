use rand::{rngs::SmallRng, SeedableRng};
use rand_distr::{Distribution, Normal};

pub struct Question {
    pub real_elo: f64,
    pub estimated_elo: f64,
    pub age: usize,
}

impl Question {
    pub fn new(elo: f64) -> Self {
        Self {
            real_elo: elo,
            estimated_elo: 1600.0,
            age: 0,
        }
    }

    pub fn get_n_random_questions(n: usize, mean: f64, std_dev: f64) -> Vec<Self> {
        let mut questions = Vec::with_capacity(n);
        let mut rng = SmallRng::from_entropy();
        let distr = Normal::new(mean, std_dev).unwrap();

        for _ in 0..n {
            let random_elo: f64 = distr.sample(&mut rng);

            questions.push(Self::new(random_elo));
        }
        questions
    }
}
