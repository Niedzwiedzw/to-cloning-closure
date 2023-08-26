use std::{error::Error, path::Path};
use thiserror::Error;

#[derive(Debug, Error)]
pub enum BuildError {
    #[error("IO error occurred: {source}")]
    IO {
        #[from]
        source: std::io::Error,
    },
}

type Result<T> = std::result::Result<T, BuildError>;

fn generate_impls() -> Result<String> {
    Ok("".to_owned())
}

fn main() -> Result<()> {
    let generated = Path::new("./src/generated.rs");
    generate_impls().and_then(|code| std::fs::write(generated, code).map_err(Into::into))
}
