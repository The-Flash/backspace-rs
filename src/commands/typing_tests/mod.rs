use crate::{cli::cli::TestSubcommands, errors::AppError};

pub mod list_tests;
pub mod run_test;

/// Handles typing test related commands
pub fn handle(commands: TestSubcommands) -> Result<(), AppError> {
    match commands {
        TestSubcommands::Ls => list_tests::run(),
        TestSubcommands::Run { id } => run_test::run(id.as_str())
    }
}
