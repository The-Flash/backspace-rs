pub mod cli;
pub mod commands;
pub mod errors;
pub mod testsuite;

fn main() {
    let command = cli::build();
    let _ = cli::dispatch(command);
}
