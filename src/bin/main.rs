use rayon::prelude::*;
use std::sync::{Arc, Mutex};

use adaptive_elo_game::{Question, RandomNormalGen, Student};
use rand::{rngs::SmallRng, Rng, SeedableRng};

fn main() {
    let student_amount: usize = 10;
    let question_amount: usize = 10000;
    let iterations: usize = 1000;

    let mut rng = RandomNormalGen::new(1600.0, 400.0);
    let mut students = get_n_random_students(student_amount, &mut rng);
    let questions = get_n_random_questions(question_amount, &mut rng);

    students.iter().for_each(|student| {
        println!(
            "Rating: {}, Actual Elo: {}",
            student.estimated_elo, student.real_elo
        )
    });

    println!();

    students.par_iter_mut().for_each(|student: &mut Student| {
        let mut random = SmallRng::from_entropy();

        (0..iterations).for_each(|_| {
            let question: &Arc<Mutex<Question>> = &questions[random.gen_range(0..question_amount)];
            student.solve_task(question, &mut random);
        })
    });

    students.iter().for_each(|student| {
        println!(
            "Rating: {}, Actual Elo: {}",
            student.estimated_elo, student.real_elo
        )
    });
}

fn get_n_random_students(n: usize, rng: &mut RandomNormalGen) -> Vec<Student> {
    (0..n).map(|_| Student::new(rng.get_u32())).collect()
}

fn get_n_random_questions(n: usize, rng: &mut RandomNormalGen) -> Vec<Arc<Mutex<Question>>> {
    (0..n)
        .map(|_| Arc::new(Mutex::new(Question::new(rng.get_u32()))))
        .collect()
}
