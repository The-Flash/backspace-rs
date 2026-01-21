pub mod cli;
pub mod commands;
pub mod errors;

fn main() {
    let command = cli::build();
    let _ = cli::dispatch(command);
}
