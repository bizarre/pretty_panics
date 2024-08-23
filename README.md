# pretty_panics

`pretty_panics` is a crate that provides an easy way to customize panic messages and error messages.

<img width="509" alt="Screenshot 2024-08-23 at 2 00 37 AM" src="https://github.com/user-attachments/assets/6213d4b2-27c2-4ecc-b1a1-9033cdf68963">


If `use-default-features` is enabled, `pretty_panics` will provide default formatters for panics and errors.

```rust
use pretty_panics::pretty_panic;

#[pretty_panic]
fn main() -> std::result::Result<(), SomeDisplayError> {
    ...
}
```

```rust
use pretty_panics::pretty_panic;
use thiserror::Error;

#[derive(Error, Debug)]
enum SimpleError {
    #[error("an error")]
    AnError
}

fn error_formatter(error: &SimpleError) -> String {
    format!("uhhh something broke guys... {error}")
}

#[pretty_panic(formatter = error_formatter)]
fn main() -> anyhow::Result<(), SimpleError> {
    Result::<(), SimpleError>::Err(SimpleError::AnError)?;
    Ok(())
}
```

caveats:

- requires your main fn to return a result
- you will lose the ability to get panic backtraces unless you implement your own formatter and include it
- you probably shouldn't use this tbh LMAO
- doesn't work with async main fns yet xd
