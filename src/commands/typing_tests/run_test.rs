use crate::{errors::AppError, testsuite::{TestSuite, TomlLoader}};

pub fn run(id: &str) -> Result<(), AppError> {
    println!("Running typing test {}...", id);
    let typing_tests_file = "./data/typing-tests/data.toml";
    let suite = TestSuite::<TomlLoader>::load(typing_tests_file)?;
    let test = suite.tests.into_iter()
        .find(|test| test.id == id);
    println!("Test found: {:?}", test);
    Ok(())
}
