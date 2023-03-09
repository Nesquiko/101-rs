//! Definitions for the quizzer app

use std::fmt::Display;

/// An enum that represents all errors that can
/// originate from this library.
#[derive(Debug)]
pub enum QuizError {
    InvalidQuizFile(String),
    Io(std::io::Error),
    Json(serde_json::Error),
}

impl Display for QuizError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        // Pretty-format the error
        match self {
            QuizError::InvalidQuizFile(fpath) => write!(f, "file {fpath} doen't exist"),
            QuizError::Io(e) => write!(f, "IO error: {e}"),
            QuizError::Json(e) => write!(f, "JSON error: {e}"),
        }
    }
}

/// Implementing std::error::Error makes the error
/// play nice with `anyhow`. Note that you must implement
/// [std::fmt::Debug] and [std::fmt::Display] in order to
/// implement [std::error::Error].
impl std::error::Error for QuizError {}

/// Convert [serde_json::Error] to [QuizError],
/// allowing us to use the try (`?`) operator
/// to easily convert [Result<_, serde_json::Error>]
/// to [Result<_, Error>].
impl From<serde_json::Error> for QuizError {
    fn from(e: serde_json::Error) -> Self {
        Self::Json(e)
    }
}

/// Convert [std::io::Error] to [QuizError]
/// allowing us to use the try (`?`) operator
/// to easily convert [Result<_, serde_json::Error>]
/// to [Result<_, Error>].
impl From<std::io::Error> for QuizError {
    fn from(e: std::io::Error) -> Self {
        Self::Io(e)
    }
}
