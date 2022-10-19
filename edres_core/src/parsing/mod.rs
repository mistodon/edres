//! This module contains all of the utilities for parsing markup
//! files.
//!
//! The submodules are controlled by the `edres` features set.
//! So for example, the `json` module will only be present if
//! the `edres/json` feature is enabled.

#[cfg(feature = "json")]
pub mod json;

#[cfg(feature = "toml")]
pub mod toml;

#[cfg(feature = "yaml")]
pub mod yaml;

use std::path::Path;

use crate::{
    error::Error,
    format::Format,
    options::{FloatSize, IntSize, ParseOptions},
    value::Value,
};

/// Parses a source file into a generic `Value`, inferring its
/// format.
pub fn parse_source_file(file: &Path, options: &ParseOptions) -> Result<Value, Error> {
    let source = std::fs::read_to_string(file)?;
    let format = Format::from_filename(file)?;
    parse_source(&source, format, options)
}

pub(crate) fn parse_source_file_with_format(
    file: &Path,
    format: Option<Format>,
    options: &ParseOptions,
) -> Result<Value, Error> {
    let source = std::fs::read_to_string(file)?;
    let format = match format {
        None => Format::from_filename(file)?,
        Some(x) => x,
    };
    parse_source(&source, format, options)
}

/// Parse source of a given format, producing a generic `Value`.
pub fn parse_source(source: &str, format: Format, options: &ParseOptions) -> Result<Value, Error> {
    match format {
        #[cfg(feature = "json")]
        Format::Json => json::parse_source(source, options),

        #[cfg(feature = "toml")]
        Format::Toml => toml::parse_source(source, options),

        #[cfg(feature = "yaml")]
        Format::Yaml => yaml::parse_source(source, options),
    }
}

/// Attempts to unify values internal to the given one so that
/// their types are compatible.
///
/// **NOTE:** This is not currently implemented.
pub fn unify_value(_value: &mut Value) -> Result<(), Error> {
    // TODO: unify_values in all sequences
    Ok(())
}

/// Attempts to unify all provided values to compatible types.
///
/// **NOTE:** This is not currently implemented.
pub fn unify_values(_values: &mut [Value]) -> Result<(), Error> {
    // TODO: Unify values in this sequence
    Ok(())
}

pub(crate) fn preferred_float(value: f64, preferred: FloatSize) -> Value {
    use FloatSize::*;
    match preferred {
        F32 if value >= (f32::MIN as f64) && value <= (f32::MAX as f64) => Value::F32(value as f32),
        _ => Value::F64(value),
    }
}

pub(crate) const fn preferred_int(value: i128, preferred: IntSize) -> Value {
    use IntSize::*;
    match preferred {
        ISize => Value::ISize(value as isize),
        I8 if value >= (i8::MIN as i128) && value <= (i8::MAX as i128) => Value::I8(value as i8),
        I8 | I16 if value >= (i16::MIN as i128) && value <= (i16::MAX as i128) => {
            Value::I16(value as i16)
        }
        I8 | I16 | I32 if value >= (i32::MIN as i128) && value <= (i32::MAX as i128) => {
            Value::I32(value as i32)
        }
        I8 | I16 | I32 | I64 if value >= (i64::MIN as i128) && value <= (i64::MAX as i128) => {
            Value::I64(value as i64)
        }
        _ => Value::I128(value),
    }
}

pub(crate) fn array_or_vec(seq: Vec<Value>, max_array_size: Option<usize>) -> Value {
    if max_array_size.is_some() && seq.len() <= max_array_size.unwrap() {
        Value::Array(seq.len(), seq)
    } else {
        Value::Vec(seq)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn max_array_size() {
        let u = Value::Unit;
        assert_eq!(
            array_or_vec(vec![u.clone(); 3], None),
            Value::Vec(vec![u.clone(); 3])
        );
        assert_eq!(
            array_or_vec(vec![u.clone(); 3], Some(3)),
            Value::Array(3, vec![u.clone(); 3])
        );
        assert_eq!(
            array_or_vec(vec![u.clone(); 4], Some(3)),
            Value::Vec(vec![u.clone(); 4])
        );
    }

    #[test]
    fn correct_preferred_int() {
        assert_eq!(preferred_int(-1, IntSize::I32), Value::I32(-1));
        assert_eq!(preferred_int(1, IntSize::I64), Value::I64(1));
        assert_eq!(preferred_int(129, IntSize::I8), Value::I16(129));
        assert_eq!(preferred_int(65537, IntSize::I16), Value::I32(65537));
    }

    #[test]
    fn correct_preferred_float() {
        assert_eq!(preferred_float(2.5, FloatSize::F32), Value::F32(2.5));
        assert_eq!(preferred_float(2.5, FloatSize::F64), Value::F64(2.5));
        assert_eq!(
            preferred_float((f32::MAX as f64) * 2.0, FloatSize::F32),
            Value::F64((f32::MAX as f64) * 2.0)
        );
    }
}
