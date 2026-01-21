use std::fs;
use serde::{Deserialize};
use tabled::{
    Tabled, 
    Table,
    settings::{Style},
};

use crate::errors::AppError;

/// Represents a suite of typing tests loaded from a TOML file.
#[derive(Deserialize, Debug)]
pub struct TestSuite {
    pub tests: Vec<TypingTest>,
}

#[derive(Deserialize, Debug, Clone)]
#[serde(rename_all = "lowercase")]
pub enum Difficulty {
    Easy,
    Medium,
    Hard
}

/// Represents a single typing test.
#[derive(Deserialize, Debug)]
pub struct TypingTest {
    /// Unique identifier for the typing test.
    pub id: String,
    /// Title of the typing test.
    pub title: String,
    /// Difficulty level of the typing test.
    pub difficulty: Difficulty,
    /// Content of the typing test.
    pub content: String,
    // TODO: Add tags
}

#[derive(Debug, Tabled)]
struct TableRow<'a> {
    id: &'a str,
    title: &'a str,
    difficulty: &'a str,
}

impl<'a> From<&'a TypingTest> for TableRow<'a> {
    fn from(test: &'a TypingTest) -> Self {
        TableRow {
            id: &test.id,
            title: &test.title,
            difficulty: match &test.difficulty {
                Difficulty::Easy => "easy",
                Difficulty::Medium => "medium",
                Difficulty::Hard => "hard",
            },
        }
    }
}

pub fn run() -> Result<(), AppError> {
    let typing_tests_file = "./data/typing-tests/data.toml";
    let content = fs::read_to_string(typing_tests_file)?;
    let suite: TestSuite = toml::from_str(&content)?;
    let table_rows: Vec<TableRow> = suite.tests.iter().map(|test| TableRow::from(test)).collect();
    let mut table = Table::new(table_rows);
    println!("{}", table.with(Style::modern()).to_string());
    Ok(())
}
