//! Parsing utilities for RON config files. (Requires the `ron-parsing` feature.)
//!
//! Not all of the RON syntax is currently supported:
//!
//! 1.  Maps are not supported, for example: `{ "a": 1 }`, because `ron` cannot parse them as
//!     structs.
//! 2.  Named structs are not supported, for example: `Person(age: 20)`, because the struct name
//!     is not available at build time, and so cannot match the name in the config file.
//! 3.  Tuples are not supported, for example: `(1, 2, 3)`. It was attempted and did not work for
//!     some reason.
use ron::{
    self,
    value::{Number, Value as RonValue},
};

use crate::error::WipError;
use crate::options::WipOptions;
use crate::parsing;
use crate::value::{Struct, Value};

pub fn parse_source(source: &str, options: &WipOptions) -> Result<Value, WipError> {
    let raw_value: RonValue = ron::de::from_str(source).map_err(|err| WipError(err.to_string()))?;
    parse_value_non_unified(raw_value, options)
}

pub fn parse_value(raw_value: ron::value::Value, options: &WipOptions) -> Result<Value, WipError> {
    let mut result = parse_value_non_unified(raw_value, options)?;
    parsing::unify_value(&mut result)?;
    Ok(result)
}

pub fn parse_value_non_unified(
    raw_value: ron::value::Value,
    options: &WipOptions,
) -> Result<Value, WipError> {
    Ok(match raw_value {
        RonValue::Unit => Value::Unit,
        RonValue::Bool(value) => Value::Bool(value),
        RonValue::Char(value) => Value::Char(value),
        RonValue::Number(value) => match value {
            Number::Integer(integer) => {
                parsing::preferred_int(integer as i128, options.default_int_size)
            }
            Number::Float(float) => {
                parsing::preferred_float(float.get(), options.default_float_size)
            }
        },
        RonValue::String(value) => Value::String(value),
        RonValue::Option(value) => Value::Option(if let Some(value) = value {
            Some(Box::new(parse_value_non_unified(*value, options)?))
        } else {
            None
        }),
        RonValue::Seq(values) => parsing::array_or_vec(
            values
                .into_iter()
                .map(|value| parse_value_non_unified(value, options))
                .collect::<Result<Vec<_>, _>>()?,
            options.max_array_size,
        ),
        RonValue::Map(mapping) => Value::Struct(Struct(
            mapping
                .iter()
                .map(|(key, value)| {
                    let key = match key {
                        RonValue::String(key) => key.to_owned(),
                        _ => return Err(WipError("Keys in maps must be strings".into())),
                    };
                    parse_value_non_unified(value.clone(), options).map(|value| (key, value))
                })
                .collect::<Result<_, WipError>>()?,
        )),
    })
}
