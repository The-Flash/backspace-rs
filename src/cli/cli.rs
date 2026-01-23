use clap::{Parser, Subcommand};

/// Command-Line Interface definition for the Backspace
#[derive(Debug, Parser)]
#[command(name = "backspace")]
#[command(about = "Command-Line Typing Test tool", long_about = None)]
pub struct Cli {
    /// The primary command to execute
    #[command(subcommand)]
    pub command: Commands,
}

/// Primary commands for the Backspace CLI
#[derive(Debug, Subcommand)]
pub enum Commands {
    /// Type test related commands
    #[command(arg_required_else_help = true)]
    Test {
        /// Subcommands for the 'test' command
        #[command(subcommand)]
        commands: TestSubcommands,
    },
}

/// Subcommands under the 'test' command
#[derive(Debug, Subcommand)]
pub enum TestSubcommands {
    /// List available typing tests
    #[command(about = "List available typing tests")]
    Ls,
    #[command(about = "Run a typing test")]
    Run {
        /// ID of the typing test to run
        #[arg(short, long)]
        id: String,
    }
}
