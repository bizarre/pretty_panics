# pretty_panic

`pretty_panic` is a crate that provides an easy way to customize panic messages and error messages.

If `use-default-features` is enabled, `pretty_panic` will provide default formatters for panics and errors.

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
caveats:

- requires your main fn to return a result
- you will lose the ability to get panic backtraces unless you implement your own formatter and include it
- you probably shouldn't use this tbh LMAO
- doesn't work with async main fns yet xd
