#[cfg(feature = "json-parsing")]
pub mod json;

#[cfg(feature = "ron-parsing")]
pub mod ron;

#[cfg(feature = "toml-parsing")]
pub mod toml;

#[cfg(feature = "yaml-parsing")]
pub mod yaml;

use std::path::Path;

use crate::{
    error::WipError,
    format::Format,
    options::{FloatSize, IntSize, ParseOptions},
    value::Value,
};

pub fn parse_source_file(file: &Path, options: &ParseOptions) -> Result<Value, WipError> {
    let source = std::fs::read_to_string(file).map_err(|x| WipError(x.to_string()))?;
    let format = Format::from_filename(file).map_err(|x| WipError(x.to_string()))?;
    parse_source(&source, format, options)
}

pub(crate) fn parse_source_file_with_format(
    file: &Path,
    format: Option<Format>,
    options: &ParseOptions,
) -> Result<Value, WipError> {
    let source = std::fs::read_to_string(file).map_err(|x| WipError(x.to_string()))?;
    let format = match format {
        None => Format::from_filename(file).map_err(|x| WipError(x.to_string()))?,
        Some(x) => x,
    };
    parse_source(&source, format, options)
}

pub fn parse_source(
    source: &str,
    format: Format,
    options: &ParseOptions,
) -> Result<Value, WipError> {
    match format {
        #[cfg(feature = "json-parsing")]
        Format::Json => json::parse_source(source, options),

        #[cfg(feature = "ron-parsing")]
        Format::Ron => ron::parse_source(source, options),

        #[cfg(feature = "toml-parsing")]
        Format::Toml => toml::parse_source(source, options),

        #[cfg(feature = "yaml-parsing")]
        Format::Yaml => yaml::parse_source(source, options),
    }
}

pub fn unify_value(_value: &mut Value) -> Result<(), WipError> {
    // TODO: unify_values in all sequences
    Ok(())
}

pub fn unify_values(_values: &mut [Value]) -> Result<(), WipError> {
    // TODO: Unify values in this sequence
    Ok(())
}

pub(crate) fn preferred_float(value: f64, preferred: FloatSize) -> Value {
    match preferred {
        FloatSize::F32 => Value::F32(value as f32),
        FloatSize::F64 => Value::F64(value),
    }
}

pub(crate) fn preferred_int(value: i128, preferred: IntSize) -> Value {
    match preferred {
        IntSize::I8 => Value::I8(value as i8),
        IntSize::I16 => Value::I16(value as i16),
        IntSize::I32 => Value::I32(value as i32),
        IntSize::I64 => Value::I64(value as i64),
        IntSize::I128 => Value::I128(value),
        IntSize::ISize => Value::ISize(value as isize),
    }
}

pub(crate) fn array_or_vec(seq: Vec<Value>, max_array_size: Option<usize>) -> Value {
    if max_array_size.is_some() && seq.len() <= max_array_size.unwrap() {
        Value::Array(seq.len(), seq)
    } else {
        Value::Vec(seq)
    }
}
