use clap::Parser;

use super::cli::{Cli, Commands};
use crate::{commands::typing_tests, errors::AppError};

/// Builds command line arguments
pub fn build() -> Commands {
    let args = Cli::parse();
    args.command
}

/// Dispatches commands based on user input
pub fn dispatch(command: Commands) -> Result<(), AppError> {
    match command {
        Commands::Test { commands } => typing_tests::handle(commands),
    }
}
