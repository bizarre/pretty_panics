extern crate pretty_panics;

use pretty_panics::pretty_panic;
use thiserror::Error;

#[derive(Error, Debug)]
enum SimpleError {
    #[error("Broken")]
    AnError
}

fn error_formatter(error: &SimpleError) -> String {
    format!("uhhh something broke guys... {error}")
}

#[pretty_panic(formatter = error_formatter)]
fn main() -> anyhow::Result<(), SimpleError> {
    Result::<(), SimpleError>::Err(SimpleError::AnError).unwrap();
    Ok(())
}