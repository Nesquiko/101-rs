mod error;

use std::{
    fs::{self, OpenOptions},
    io::Write,
    ops::Index,
    path::Path,
};

use serde::{Deserialize, Serialize};

use crate::error::QuizError;

#[derive(Debug, Serialize, Deserialize)]
pub struct Quiz {
    name: String,
    file: String,
    questions: Vec<Question>,
}

impl Quiz {
    pub fn new(file: String, name: String) -> anyhow::Result<Quiz, QuizError> {
        let quiz = Quiz {
            name,
            file,
            questions: vec![],
        };

        if Path::new(&quiz.file).exists() {
            fs::remove_file(Path::new(&quiz.file))?;
        };
        quiz.save()?;

        Ok(quiz)
    }

    fn save(&self) -> anyhow::Result<(), QuizError> {
        let mut f = OpenOptions::new()
            .create_new(true)
            .write(true)
            .open(&self.file)?;

        let json = serde_json::to_string(&self)?;
        f.write_all(json.as_bytes())?;
        Ok(())
    }

    pub fn add_question(&mut self, q: Question) -> anyhow::Result<(), QuizError> {
        self.load_quiz()?;
        self.questions.push(q);
        self.save()?;

        Ok(())
    }

    pub fn load_quiz(&mut self) -> anyhow::Result<(), QuizError> {
        let config = std::fs::read_to_string(&self.file)?;
        let quiz: Quiz = serde_json::from_str(&config)?;
        self.name = quiz.name;
        self.file = quiz.file;
        self.questions = quiz.questions;
        Ok(())
    }

    pub fn iter(&self) -> QuizIter {
        QuizIter {
            quiz: self,
            index: 0,
        }
    }
}

pub struct QuizIter<'a> {
    quiz: &'a Quiz,
    index: usize,
}

impl<'a> Iterator for QuizIter<'a> {
    type Item = &'a Question;

    fn next(&mut self) -> Option<Self::Item> {
        let q = if self.index >= self.quiz.questions.len() {
            None
        } else {
            Some(self.quiz.questions.index(self.index))
        };
        self.index += 1;
        q
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Question {
    question: String,
    correct_answer: String,
    other_answers: [String; 3],
}

impl Question {
    pub fn question(&self) -> &str {
        self.question.as_ref()
    }

    pub fn correct_answer(&self) -> &str {
        self.correct_answer.as_ref()
    }

    pub fn other_answers(&self) -> &[String; 3] {
        &self.other_answers
    }
}
