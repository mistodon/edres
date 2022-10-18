use serde_yaml::{self, Value as YamlValue};

use crate::{
    error::Error,
    options::ParseOptions,
    parsing,
    value::{Struct, Value},
};

pub fn parse_source(source: &str, options: &ParseOptions) -> Result<Value, Error> {
    let raw_value: YamlValue = serde_yaml::from_str(source)?;
    parse_value(raw_value, options)
}

pub fn parse_value(raw_value: YamlValue, options: &ParseOptions) -> Result<Value, Error> {
    let mut result = parse_value_non_unified(raw_value, options)?;
    parsing::unify_value(&mut result)?;
    Ok(result)
}

pub fn parse_value_non_unified(
    raw_value: YamlValue,
    options: &ParseOptions,
) -> Result<Value, Error> {
    Ok(match raw_value {
        YamlValue::Null => Value::Option(None),
        YamlValue::Bool(value) => Value::Bool(value),
        YamlValue::Number(value) => match (value.as_i64(), value.as_u64(), value.as_f64()) {
            (Some(x), _, _) => parsing::preferred_int(x as i128, options.default_int_size),
            (None, Some(x), _) => parsing::preferred_int(x as i128, options.default_int_size),
            (None, None, Some(x)) => parsing::preferred_float(x, options.default_float_size),
            _ => return Err(Error::ErrorParsingNumber),
        },
        YamlValue::String(value) => Value::String(value),
        YamlValue::Sequence(values) => parsing::array_or_vec(
            values
                .into_iter()
                .map(|value| parse_value_non_unified(value, options))
                .collect::<Result<Vec<_>, _>>()?,
            options.max_array_size,
        ),
        YamlValue::Mapping(values) => Value::Struct(Struct(
            values
                .into_iter()
                .map(|(key, value)| {
                    let key = key.as_str().ok_or(Error::ExpectedStringKey)?.to_owned();
                    parse_value_non_unified(value, options).map(|value| (key, value))
                })
                .collect::<Result<_, Error>>()?,
        )),
        YamlValue::Tagged(tagged_value) => parse_value_non_unified(tagged_value.value, options)?,
    })
}

// TODO:
// - yaml! to make one big kitchen-sink value
// - test that it converts properly
