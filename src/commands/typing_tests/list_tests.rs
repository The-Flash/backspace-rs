use tabled::{
    Tabled, 
    Table,
    settings::{Style},
};

use crate::{errors::AppError, testsuite::testsuite::{Difficulty, TestSuite, TomlLoader, TypingTest}};

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
    let suite = TestSuite::<TomlLoader>::load(typing_tests_file)?;
    let table_rows: Vec<TableRow> = suite.tests.iter().map(|test| TableRow::from(test)).collect();
    let mut table = Table::new(table_rows);
    println!("{}", table.with(Style::modern()).to_string());
    Ok(())
}
