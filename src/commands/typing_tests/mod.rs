use crate::{cli::cli::TestSubcommands, errors::AppError};

pub mod list_tests;

/// Handles typing test related commands
pub fn handle(commands: TestSubcommands) -> Result<(), AppError> {
    match commands {
        TestSubcommands::Ls => list_tests::run()
    }
}
