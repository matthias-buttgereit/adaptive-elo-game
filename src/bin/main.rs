use colored::Colorize;
use rayon::prelude::*;
use std::sync::{Arc, Mutex};

use adaptive_elo_game::{adaptive::adjust_rating, Question, Student};
use rand::{rngs::SmallRng, Rng, SeedableRng};

const STUDENT_AMOUNT: usize = 10;
const QUESTION_AMOUNT: usize = 30;
const ITERATIONS: usize = 3;

fn main() {
    let (mut students, questions) = get_students_and_questions(STUDENT_AMOUNT, QUESTION_AMOUNT);

    let mut errors: Vec<[(f64, f64); STUDENT_AMOUNT]> = Vec::with_capacity(STUDENT_AMOUNT);

    push_elo_errors(&mut errors, &students);

    students.par_iter_mut().for_each(|student: &mut Student| {
        let mut random = SmallRng::from_entropy();

        (0..ITERATIONS).for_each(|_| {
            let question: &Arc<Mutex<Question>> = &questions[random.gen_range(0..QUESTION_AMOUNT)];
            let success = student.solve_task(question, &mut random);

            let _ = adjust_rating(success, student, question);
        });
    });
    push_elo_errors(&mut errors, &students);
    print_errors(&errors);
}

fn print_errors(errors: &[[(f64, f64); STUDENT_AMOUNT]]) {
    (0..STUDENT_AMOUNT).for_each(|i| {
        let mut error_before: f64 = 0.0;

        errors.iter().for_each(|array| {
            let (elo, error) = array[i];

            if error_before != 0.0 {
                if error_before > error {
                    let error = format!("{:>6.2}%", error);
                    print!("{:4}{} | ", elo as u32, error.green());
                } else if error_before < error {
                    let error = format!("{:>6.2}%", error);
                    print!("{:4}{} | ", elo as u32, error.red());
                } else {
                    let error = format!("{:>6.2}%", error);
                    print!("{:4}{} | ", elo as u32, error);
                }
            } else {
                let error = format!("{:>6.2}%", error);
                print!("{:4}{} | ", elo as u32, error);
            }

            error_before = error;
        });
        println!();
    });
}

fn push_elo_errors(errors: &mut Vec<[(f64, f64); STUDENT_AMOUNT]>, students: &[Student]) {
    errors.push(
        students
            .iter()
            .map(|s| {
                (
                    s.estimated_elo,
                    (s.estimated_elo - s.real_elo).abs() * 100.0 / s.real_elo,
                )
            })
            .collect::<Vec<_>>()
            .try_into()
            .unwrap(),
    );
}

fn get_students_and_questions(
    student_amount: usize,
    question_amount: usize,
) -> (Vec<Student>, Vec<Arc<Mutex<Question>>>) {
    let students = Student::get_n_random_students(student_amount, 800.0, 200.0);
    let questions = Question::get_n_random_questions(question_amount, 1600.0, 300.0);
    let questions: Vec<_> = questions
        .into_iter()
        .map(|q| Arc::new(Mutex::new(q)))
        .collect();
    (students, questions)
}
