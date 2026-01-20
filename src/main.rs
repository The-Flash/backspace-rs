pub mod cli;
pub mod commands;

fn main() {
    let command = cli::build();
    cli::dispatch(command);
}
