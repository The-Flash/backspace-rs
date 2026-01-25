use crate::{errors::AppError, testsuite::{loader::Loader, model::{TestSuite}, TypingTests}};

/// Loader implementation for TOML files.
pub struct TomlLoader;

impl Loader for TomlLoader {
    /// Loads typing tests from a TOML file.
    fn load(file_path: &str) -> Result<TypingTests, AppError> {
        let content = std::fs::read_to_string(file_path)?;
        let suite: TestSuite<Self> = toml::from_str(&content)?;
        Ok(suite.tests)
    }
}

