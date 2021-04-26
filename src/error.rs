use std::io::Error as IOError;

use thiserror::Error as ErrorTrait;

/// An error type for errors while generating config struct modules.
///
/// Errors can either occur during IO (when reading or creating files) or during
/// the generation itself.
#[derive(Debug, ErrorTrait)]
pub enum Error {
    #[error("Generation error")]
    Generation(#[from] GenerationError),

    #[error("IO error")]
    Io(#[from] IOError),
}

/// An error occurring during code generation.
#[derive(Debug, ErrorTrait)]
pub enum GenerationError {
    /// Occurs when the config format can't be determined from the
    /// filename extension of the input file.
    #[error("Unknown input format: `{0}`. (Maybe you need to enable the right feature?)")]
    UnknownInputFormat(String),

    /// Occurs when encountering a field in the config which is not a
    /// valid name for a struct field.
    #[error("Invalid field name: `{0}`.")]
    InvalidFieldName(String),

    /// Occurs when encountering a field in the config which is not a
    /// valid name for an enum variant.
    #[error("Invalid variant name: `{0}`.")]
    InvalidVariantName(String),

    /// Occurs when an array in the config file contains multiple different types
    /// of data, which cannot be represented in a Rust struct.
    #[error("Array under key `{0}` has elements of different types. Arrays must be homogenous.")]
    HeterogenousArray(String),

    /// Occurs when generating from source and not a file, if attempting to also
    /// generate dynamic loading functions.
    ///
    /// Because no input filepath was given, it's impossible to generate a function
    /// which loads from that file.
    #[error("Cannot generate dynamic loading functions without a filename. (Generate struct from a file, set generate_load_fns: false, or set dynamic_loading: DynamicLoading::Never to fix.)")]
    MissingFilePath,

    /// Occurs when the config file could not be correctly parsed.
    #[error("Deserialization failed: {0}")]
    DeserializationFailed(String),

    /// Occurs when invalid options were provided.
    #[error("Invalid options error")]
    StructOptions(#[from] OptionsError),
}

/// An error type for when a [`StructOptions`](struct.StructOptions.html) value
/// failed validation.
#[derive(Debug, ErrorTrait)]
pub enum OptionsError {
    /// Occurs when the provided `struct_name` is not a valid Rust identifier.
    #[error("Invalid name for a struct: `{0}`.")]
    InvalidStructName(String),

    /// Occurs when the provided `const_name` is not a valid Rust identifier.
    #[error("Invalid name for a const: `{0}`.")]
    InvalidConstName(String),
}
