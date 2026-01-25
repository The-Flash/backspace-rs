use crate::{errors::AppError, testsuite::{TypingTests}};
pub mod toml;
pub use toml::TomlLoader;

/// Trait for loading typing tests from a file.
pub trait Loader {
    /// Loads typing tests from the specified file path.
    fn load(file_path: &str) -> Result<TypingTests, AppError>;
}

