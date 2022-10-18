pub mod codegen;
pub mod options;
pub mod parsing;
pub mod value;

mod error;
mod format;

#[cfg(not(any(feature = "json", feature = "toml", feature = "yaml",)))]
compile_error!(
    "The edres crate requires at least one parsing feature to be enabled:\n {json, toml, yaml}"
);

pub use crate::{error::Error, format::Format, options::*};
