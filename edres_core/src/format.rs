use std::path::Path;

use crate::error::*;

/// Represents an input markup format for a config file.
///
/// The variants that exist correspond to the features that have been enabled.
/// For example, if the `json` feature is not enabled, then the
/// `Format::Json` variant will not exist.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Format {
    #[cfg(feature = "json")]
    Json,
    #[cfg(feature = "toml")]
    Toml,
    #[cfg(feature = "yaml")]
    Yaml,
}

impl Format {
    /// Matches the extension of the given filename to identify
    /// the format of the file.
    ///
    /// # Examples
    /// ```
    /// # use edres_core::Format;
    /// assert_eq!(
    ///     Format::from_filename("file.yml".as_ref()).unwrap(),
    ///     Format::Yaml,
    /// );
    /// ```
    pub fn from_filename(filename: &Path) -> Result<Self, Error> {
        match filename.extension() {
            Some(ext) => match ext.to_string_lossy().as_ref() {
                #[cfg(feature = "json")]
                "json" => Ok(Format::Json),

                #[cfg(feature = "toml")]
                "toml" => Ok(Format::Toml),

                #[cfg(feature = "yaml")]
                "yaml" | "yml" => Ok(Format::Yaml),

                other => Err(Error::UnknownInputFormat(Some(other.into()))),
            },
            None => Err(Error::UnknownInputFormat(None)),
        }
    }
}
