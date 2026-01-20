pub mod cli;

fn main() {
    let command = cli::build();
    cli::dispatch(command);
}
