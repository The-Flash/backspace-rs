use std::fs;
use serde::{Deserialize};

use crate::errors::AppError;

/// Represents a suite of typing tests loaded from a TOML file.
#[derive(Deserialize, Debug)]
pub struct TestSuite {
    pub tests: Vec<TypingTest>,
}

/// Represents a single typing test.
#[derive(Deserialize, Debug)]
pub struct TypingTest {
    /// Unique identifier for the typing test.
    pub id: String,
    /// Title of the typing test.
    pub title: String,
    /// Difficulty level of the typing test.
    pub difficulty: String,
    /// Content of the typing test.
    pub content: String,
    // TODO: Add tags
}

pub fn list_tests() -> Result<(), AppError> {
    let typing_tests_file = "./data/typing-tests/data.toml";
    let content = fs::read_to_string(std::path::Path::new(typing_tests_file))?;
    let suite: TestSuite = toml::from_str(&content)?;
    for test in suite.tests {
        println!("ID: {}, Title: {}, Difficulty: {}", test.id, test.title, test.difficulty);
    }
    Ok(())
}
