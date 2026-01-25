use serde::{Deserialize};
use std::marker::PhantomData;

use crate::{errors::AppError, testsuite::loader::Loader};

#[derive(Deserialize, Debug)]
pub struct TypingTests(Vec<TypingTest>);

/// Represents a suite of typing tests loaded from a TOML file.
#[derive(Deserialize, Debug)]
pub struct TestSuite<L> {
    /// List of typing tests in the suite.
    pub tests: TypingTests,

    #[serde(skip)]
    _loader: PhantomData<L>
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

impl<'a> IntoIterator for &'a TypingTests {
    type Item = &'a TypingTest;
    type IntoIter = std::slice::Iter<'a, TypingTest>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.iter()
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
