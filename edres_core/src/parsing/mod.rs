#[cfg(feature = "json-parsing")]
pub mod json;

#[cfg(feature = "ron-parsing")]
pub mod ron;

#[cfg(feature = "toml-parsing")]
pub mod toml;

#[cfg(feature = "yaml-parsing")]
pub mod yaml;

use std::collections::BTreeMap;
use std::path::Path;

use crate::{
    options::{FloatSize, IntSize, StructOptions},
    value::{GenericStruct, GenericValue},
};

use crate::error::WipError;
use crate::format::Format;
use crate::options::WipOptions;
use crate::value::Value;

pub fn parse_source_file(file: &Path, options: &WipOptions) -> Result<Value, WipError> {
    let source = std::fs::read_to_string(file).map_err(|x| WipError(x.to_string()))?;
    let format = Format::from_filename(file).map_err(|x| WipError(x.to_string()))?;
    parse_source(&source, format, options)
}

pub(crate) fn parse_source_file_with_format(
    file: &Path,
    format: Option<Format>,
    options: &WipOptions,
) -> Result<Value, WipError> {
    let source = std::fs::read_to_string(file).map_err(|x| WipError(x.to_string()))?;
    let format = match format {
        None => Format::from_filename(file).map_err(|x| WipError(x.to_string()))?,
        Some(x) => x,
    };
    parse_source(&source, format, options)
}

pub fn parse_source(source: &str, format: Format, options: &WipOptions) -> Result<Value, WipError> {
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

pub(crate) type ParsedFields<T> = BTreeMap<String, T>;

pub(crate) fn parsed_to_generic_struct<T, F>(
    parsed_config: ParsedFields<T>,
    options: &StructOptions,
    convert_fn: F,
) -> GenericStruct
where
    F: Fn(&str, &str, T, &StructOptions) -> GenericValue,
{
    let struct_name = "Config".to_owned();

    let fields = parsed_config
        .into_iter()
        .map(|(key, value)| {
            let value = convert_fn("_Config", &key, value, options);
            (key, value)
        })
        .collect();

    GenericStruct {
        struct_name,
        fields,
    }
}

pub(crate) fn preferred_float(value: f64, preferred: FloatSize) -> GenericValue {
    match preferred {
        FloatSize::F32 => GenericValue::F32(value as f32),
        FloatSize::F64 => GenericValue::F64(value),
    }
}

pub(crate) fn preferred_int(value: i64, preferred: IntSize) -> GenericValue {
    match preferred {
        IntSize::I8 => GenericValue::I8(value as i8),
        IntSize::I16 => GenericValue::I16(value as i16),
        IntSize::I32 => GenericValue::I32(value as i32),
        IntSize::I64 => GenericValue::I64(value),
        IntSize::I128 => unimplemented!(),
        IntSize::ISize => GenericValue::ISize(value as isize),
    }
}

pub(crate) fn preferred_float2(value: f64, preferred: FloatSize) -> Value {
    match preferred {
        FloatSize::F32 => Value::F32(value as f32),
        FloatSize::F64 => Value::F64(value),
    }
}

pub(crate) fn preferred_int2(value: i128, preferred: IntSize) -> Value {
    match preferred {
        IntSize::I8 => Value::I8(value as i8),
        IntSize::I16 => Value::I16(value as i16),
        IntSize::I32 => Value::I32(value as i32),
        IntSize::I64 => Value::I64(value as i64),
        IntSize::I128 => Value::I128(value),
        IntSize::ISize => Value::ISize(value as isize),
    }
}

pub(crate) fn array_or_vec(seq: Vec<Value>, max_array_size: usize) -> Value {
    if seq.len() <= max_array_size {
        Value::Array(seq.len(), seq)
    } else {
        Value::Vec(seq)
    }
}
