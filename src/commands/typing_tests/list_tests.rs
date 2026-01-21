use std::fs;

use crate::errors::AppError;

pub fn list_tests() -> Result<(), AppError> {
    let typing_tests_file = "./data/typing-tests/data.toml";
    let content = fs::read_to_string(std::path::Path::new(typing_tests_file))?;
    println!("{}", content);
    Ok(())
}
