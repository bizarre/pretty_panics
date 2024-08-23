extern crate pretty_panic;

use pretty_panic::pretty_panic;
use thiserror::Error;

#[derive(Debug, Error)]
#[error("Whoops, something broke: {msg}")]
struct SimpleError {
    msg: String,
}

fn error_formatter(error: &SimpleError) -> String {
    format!("{error}")
}

#[pretty_panic(formatter = error_formatter)]
fn main() -> anyhow::Result<(), SimpleError> {
    Err(SimpleError {
        msg: "An error occurred!".to_string(),
    })?;
    Ok(())
}
