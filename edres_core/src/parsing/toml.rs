use toml::{self, Value as TomlValue};

use crate::{
    error::Error,
    options::ParseOptions,
    parsing,
    value::{Struct, Value},
};

pub fn parse_source(source: &str, options: &ParseOptions) -> Result<Value, Error> {
    let raw_value: TomlValue = toml::from_str(source)?;
    parse_value(raw_value, options)
}

pub fn parse_value(raw_value: TomlValue, options: &ParseOptions) -> Result<Value, Error> {
    let mut result = parse_value_non_unified(raw_value, options)?;
    parsing::unify_value(&mut result)?;
    Ok(result)
}

pub fn parse_value_non_unified(
    raw_value: TomlValue,
    options: &ParseOptions,
) -> Result<Value, Error> {
    Ok(match raw_value {
        TomlValue::Boolean(value) => Value::Bool(value),
        TomlValue::Integer(value) => {
            parsing::preferred_int(value as i128, options.default_int_size)
        }
        TomlValue::Float(value) => parsing::preferred_float(value, options.default_float_size),
        TomlValue::String(value) => Value::String(value),
        TomlValue::Datetime(value) => Value::String(value.to_string()),
        TomlValue::Array(values) => parsing::array_or_vec(
            values
                .into_iter()
                .map(|value| parse_value_non_unified(value, options))
                .collect::<Result<Vec<_>, _>>()?,
            options.max_array_size,
        ),
        TomlValue::Table(values) => Value::Struct(Struct(
            values
                .into_iter()
                .map(|(key, value)| {
                    parse_value_non_unified(value, options).map(|value| (key, value))
                })
                .collect::<Result<_, Error>>()?,
        )),
    })
}
