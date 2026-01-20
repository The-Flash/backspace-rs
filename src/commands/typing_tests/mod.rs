use crate::cli::cli::TestSubcommands;

pub mod list_tests;

/// Handles typing test related commands
pub fn handle(commands: TestSubcommands) {
    match commands {
        TestSubcommands::Ls => {
            println!("Listing all typing tests:");
        }
    }
}
