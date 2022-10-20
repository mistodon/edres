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
/// See [`unify_values`] for more details.
pub fn unify_value(value: &mut Value) -> Result<(), Error> {
    match value {
        Value::Option(Some(inner)) => unify_value(inner)?,
        Value::Tuple(items) => {
            for value in items.iter_mut() {
                unify_value(value)?;
            }
        }
        Value::Array(_, items) => unify_values(items)?,
        Value::Vec(items) => unify_values(items)?,
        Value::Struct(inner) => {
            for value in inner.0.values_mut() {
                unify_value(value)?;
            }
        }
        _ => (),
    }

    Ok(())
}

/// Attempts to unify all provided values to compatible types.
///
/// Currently this only ensures the following:
///
/// 1.  If any of the values are null, all the values are
///     converted to Option types.
/// 2.  This function is applied recursively to sequences within
///     the given values.
pub fn unify_values(values: &mut [Value]) -> Result<(), Error> {
    for v in values.iter_mut() {
        unify_value(v)?;
    }

    // Unify Options
    {
        if values.iter().any(|v| matches!(v, Value::Option(_))) {
            values.iter_mut().for_each(Value::wrap_in_option);
        }
    }

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

#[cfg(test)]
mod unify_tests {
    use super::*;
    use crate::value::Struct;

    #[test]
    fn unify_scalar_value_is_neutral() {
        let vals = [
            Value::Unit,
            Value::Bool(true),
            Value::Char('a'),
            Value::I8(1),
            Value::I16(1),
            Value::I32(1),
            Value::I64(1),
            Value::I128(1),
            Value::ISize(1),
            Value::U8(2),
            Value::U16(2),
            Value::U32(2),
            Value::U64(2),
            Value::U128(2),
            Value::USize(2),
            Value::F32(3.0),
            Value::F64(4.0),
        ];

        for mut val in vals {
            let expected = val.clone();
            unify_value(&mut val).unwrap();
            assert_eq!(val, expected);
        }
    }

    #[test]
    fn unify_recursively_in_struct_fields() {
        let some = |v| Value::Option(Some(Box::new(v)));

        let mut value = Value::Struct(Struct::from_pairs([
            ("vec", Value::Vec(vec![Value::Unit, some(Value::Unit)])),
            (
                "array",
                Value::Array(2, vec![Value::Unit, some(Value::Unit)]),
            ),
        ]));
        unify_value(&mut value).unwrap();

        let expected = Value::Struct(Struct::from_pairs([
            (
                "vec",
                Value::Vec(vec![some(Value::Unit), some(Value::Unit)]),
            ),
            (
                "array",
                Value::Array(2, vec![some(Value::Unit), some(Value::Unit)]),
            ),
        ]));
        assert_eq!(value, expected);
    }

    #[test]
    fn unify_option_types() {
        let some = |v| Value::Option(Some(Box::new(v)));

        let cases = [
            (
                vec![Value::Unit, some(Value::Unit)],
                vec![some(Value::Unit), some(Value::Unit)],
            ),
            (
                vec![Value::Option(Some(Box::new(Value::Vec(vec![
                    Value::Unit,
                    some(Value::Unit),
                ]))))],
                vec![Value::Option(Some(Box::new(Value::Vec(vec![
                    some(Value::Unit),
                    some(Value::Unit),
                ]))))],
            ),
            (
                vec![Value::Vec(vec![Value::Unit, some(Value::Unit)])],
                vec![Value::Vec(vec![some(Value::Unit), some(Value::Unit)])],
            ),
            (
                vec![Value::Array(2, vec![Value::Unit, some(Value::Unit)])],
                vec![Value::Array(2, vec![some(Value::Unit), some(Value::Unit)])],
            ),
            (
                vec![Value::Tuple(vec![
                    Value::Vec(vec![Value::Unit, some(Value::Unit)]),
                    Value::Unit,
                ])],
                vec![Value::Tuple(vec![
                    Value::Vec(vec![some(Value::Unit), some(Value::Unit)]),
                    Value::Unit,
                ])],
            ),
        ];

        for (mut inputs, expected) in cases {
            unify_values(&mut inputs).unwrap();
            assert_eq!(inputs, expected);
        }
    }
}
