use clap::Parser;

use super::cli::{Cli, Commands, TestSubcommands};

/// Builds command line arguments
pub fn build() -> Commands {
    let args = Cli::parse();
    args.command
}

/// Dispatches commands based on user input
pub fn dispatch(command: Commands) {
    match command {
        Commands::Test { commands } => match commands {
            TestSubcommands::Ls => {
                println!("Listing available typing tests...");
                // Here you would add the logic to list the typing tests
            }
        },
    }
}
