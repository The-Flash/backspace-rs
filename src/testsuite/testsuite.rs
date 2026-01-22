use std::marker::PhantomData;

use serde::{Deserialize};

use crate::errors::AppError;

/// Represents a suite of typing tests loaded from a TOML file.
#[derive(Deserialize, Debug)]
pub struct TestSuite<L> {
    /// List of typing tests in the suite.
    pub tests: Vec<TypingTest>,

    #[serde(skip)]
    _loader: PhantomData<L>
}

/// Trait for loading typing tests from a file.
pub trait Loader {
    /// Loads typing tests from the specified file path.
    fn load(file_path: &str) -> Result<Vec<TypingTest>, AppError>;
}

/// Loader implementation for TOML files.
pub struct TomlLoader;

impl Loader for TomlLoader {
    /// Loads typing tests from a TOML file.
    fn load(file_path: &str) -> Result<Vec<TypingTest>, AppError> {
        let content = std::fs::read_to_string(file_path)?;
        let suite: TestSuite<Self> = toml::from_str(&content)?;
        Ok(suite.tests)
    }
}

impl<L> TestSuite<L> where L: Loader {
    pub fn load(file_path: &str) -> Result<Self, AppError> {
        let tests = L::load(file_path)?;
        Ok(Self{
            tests,
            _loader: PhantomData
        })
    }
}

/// Difficulty levels for typing tests.
#[derive(Deserialize, Debug, Clone)]
#[serde(rename_all = "lowercase")]
pub enum Difficulty {
    Easy,
    Medium,
    Hard
}

// Represents a single typing test.
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
