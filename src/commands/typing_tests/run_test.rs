use crate::errors::AppError;

pub fn run(id: &str) -> Result<(), AppError> {
    println!("Running typing test {}...", id);
    Ok(())
}
