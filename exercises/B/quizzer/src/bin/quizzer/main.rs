use std::process::exit;

use quizzer::Quiz;

fn main() {
    todo!("implement CLI");

    let file = String::from("quiz.json");
    let quiz_name = String::from("name");
    let quiz = match Quiz::new(file, quiz_name) {
        Ok(q) => q,
        Err(err) => {
            println!("error: {err:#?}");
            exit(1)
        }
    };

    println!("quiz: {quiz:#?}");
}
