extern crate pretty_panics;

use pretty_panics::pretty_panic;
use thiserror::Error;

#[derive(Error, Debug)]
enum SimpleError {
    #[error("Broken")]
    AnError,
}

#[pretty_panic]
fn main() -> anyhow::Result<(), SimpleError> {
    Result::<(), SimpleError>::Err(SimpleError::AnError).unwrap();
    Ok(())
}
