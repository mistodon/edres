pub mod gen;
pub mod parsing;
pub mod value;

mod enums;
mod structs;

mod error;
mod files;
mod format;
mod generation;
mod load_fns;
mod options;
mod validation;

#[cfg(not(any(
    feature = "json-parsing",
    feature = "ron-parsing",
    feature = "toml-parsing",
    feature = "yaml-parsing",
)))]
compile_error!("The edres crate requires at least one parsing feature to be enabled:\n {json-parsing, ron-parsing, toml-parsing, yaml-parsing}");

pub use crate::{
    enums::*, // TODO: not this
    error::{Error, GenerationError, OptionsError},
    format::Format,
    options::{
        DynamicLoading, EnumOptions, FloatSize, IntSize, SerdeSupport, StructOptions, WipOptions,
    },
    structs::*, // TODO: not this
};
