pub mod codegen;
pub mod options;
pub mod parsing;
pub mod value;

mod error;
mod format;

#[cfg(not(any(
    feature = "json-parsing",
    feature = "ron-parsing",
    feature = "toml-parsing",
    feature = "yaml-parsing",
)))]
compile_error!("The edres crate requires at least one parsing feature to be enabled:\n {json-parsing, ron-parsing, toml-parsing, yaml-parsing}");

pub use crate::{
    error::{Error, GenerationError, OptionsError, WipError},
    format::Format,
    options::*,
};
