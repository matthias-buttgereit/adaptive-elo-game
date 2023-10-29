use adaptive_elo_game::{adaptive::probability, Question, Student};

#[test]
fn test_probability_half_rating() {
    let student = Student::new(600.0);
    let question = Question::new(1200.0);

    let p = probability(student.real_elo, question.real_elo);

    assert_eq!(p, 0.5);
}

#[test]
fn test_probability_same_rating() {
    let student = Student::new(1200.0);
    let question = Question::new(1200.0);

    let p = probability(student.real_elo, question.real_elo);

    assert_eq!(p, 0.99);
}
