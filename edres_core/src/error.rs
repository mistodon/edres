use std::io::Error as IOError;

use thiserror::Error as ErrorTrait;

/// An error type for errors while generating config struct modules.
///
/// Errors can either occur during IO (when reading or creating files) or during
/// the generation itself.
#[derive(Debug, ErrorTrait)]
pub enum Error {
    #[error("Error parsing number")]
    ErrorParsingNumber,

    #[error("Expected a string key in mapping")]
    ExpectedStringKey,

    #[error("Expected value to be a struct but found `{0}` instead")]
    ExpectedStruct(&'static str),

    #[error("Expected values in map, but it was empty")]
    ExpectedValuesInMap,

    #[error("Provided file extension {0:?} not recognized")]
    UnknownInputFormat(Option<String>),

    #[error("Unsupported file path `{0}`")]
    UnsupportedFilePath(String),

    #[cfg(feature = "json")]
    #[error("JSON error")]
    Json(#[from] serde_json::Error),

    #[cfg(feature = "toml")]
    #[error("TOML error")]
    Toml(#[from] toml::de::Error),

    #[cfg(feature = "yaml")]
    #[error("YAML error")]
    Yaml(#[from] serde_yaml::Error),

    #[error("Error walking directories")]
    Ignore(#[from] ignore::Error),

    #[error("IO error")]
    Io(#[from] IOError),
}
