use std::{
    f64::consts::E,
    sync::{Arc, Mutex},
};

use crate::{Question, Student};

pub fn probability(ra: f64, rb: f64) -> f64 {
    1.0 / (1.0 + E.powf(2.0 * (1.0 / 99.0f64).ln() / rb * (ra - rb / 2.0)))
}

pub fn change(win: bool, k: f64, p: f64) -> f64 {
    if win {
        (1.0 - p) * k
    } else {
        (0.0 - p) * k
    }
}

pub struct AdaptiveSystem {
    initial_student_rating: f64,
    initial_question_rating: f64,
}

pub fn adjust_rating(
    success: bool,
    student: &mut Student,
    question: &Arc<Mutex<Question>>,
) -> (f64, f64) {
    let mut question = question.lock().unwrap();
    let question_elo = question.estimated_elo;
    let student_elo = student.estimated_elo;

    let expected_probability: f64 = probability(student_elo, question_elo);

    // Adjust student rating
    let change: f64 = change(success, 40.0, expected_probability);
    let new_student_rating = student_elo + change;

    // Adjust question rating
    let young_learning_bonus: f64 = 1.0 + E.powf(-0.1 * 1.0);
    let change = change * young_learning_bonus;
    let new_question_rating = question_elo - change;

    student.estimated_elo = new_student_rating;
    question.estimated_elo = new_question_rating;

    (new_student_rating, new_question_rating)
}
