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

use crate::{
    error::GenerationError,
    options::StructOptions,
    parsing,
    value::{GenericStruct, GenericValue},
};

use crate::error::WipError;
use crate::options::WipOptions;
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
                parsing::preferred_int2(integer as i128, options.default_int_size)
            }
            Number::Float(float) => {
                parsing::preferred_float2(float.get(), options.default_float_size)
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

fn ron_to_raw_value(
    super_struct: &str,
    super_key: &str,
    value: RonValue,
    options: &StructOptions,
) -> GenericValue {
    match value {
        RonValue::Unit => GenericValue::Unit,
        RonValue::Bool(value) => GenericValue::Bool(value),
        RonValue::Char(value) => GenericValue::Char(value),
        RonValue::Number(value) => match value {
            Number::Integer(integer) => parsing::preferred_int(integer, options.default_int_size),
            Number::Float(float) => {
                parsing::preferred_float(float.get(), options.default_float_size)
            }
        },
        RonValue::String(value) => GenericValue::String(value),
        RonValue::Option(option) => GenericValue::Option(
            option
                .map(|value| Box::new(ron_to_raw_value(super_struct, super_key, *value, options))),
        ),
        RonValue::Seq(values) => GenericValue::Array(
            values
                .into_iter()
                .map(|value| ron_to_raw_value(super_struct, super_key, value, options))
                .collect(),
        ),
        RonValue::Map(values) => {
            let sub_struct_name = format!("{}__{}", super_struct, super_key);
            let values = values
                .iter()
                .map(|(key, value)| {
                    let key = {
                        if let RonValue::String(key) = key {
                            key.to_owned()
                        } else {
                            unimplemented!("We should handle an error here");
                        }
                    };
                    let value = ron_to_raw_value(&sub_struct_name, &key, value.clone(), options);
                    (key, value)
                })
                .collect();
            GenericValue::Struct(GenericStruct {
                struct_name: sub_struct_name,
                fields: values,
            })
        }
    }
}

pub(crate) fn parse_ron(
    ron: &str,
    options: &StructOptions,
) -> Result<GenericStruct, GenerationError> {
    use parsing::ParsedFields;

    let ron_struct = {
        let ron_object: RonValue = ron::de::from_str(ron)
            .map_err(|err| GenerationError::DeserializationFailed(err.to_string()))?;

        if let RonValue::Map(mapping) = ron_object {
            mapping
                .iter()
                .map(|(key, value)| {
                    let key = {
                        if let RonValue::String(key) = key {
                            key.to_owned()
                        } else {
                            let m = "Top-level keys in RON map must be strings.".to_owned();
                            return Err(GenerationError::DeserializationFailed(m));
                        }
                    };
                    Ok((key, value.clone()))
                })
                .collect::<Result<ParsedFields<RonValue>, GenerationError>>()?
        } else {
            let m = "Root RON object must be a struct or map.".to_owned();
            return Err(GenerationError::DeserializationFailed(m));
        }
    };

    let generic_struct = parsing::parsed_to_generic_struct(ron_struct, options, ron_to_raw_value);

    Ok(generic_struct)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_non_string_keys() {
        let ron_code = r#"(100: "One hundred")"#;
        assert!(parse_ron(ron_code, &StructOptions::default()).is_err());
    }

    #[test]
    fn test_non_struct_root_object() {
        let ron_code = r#"["key", "value"]"#;
        assert!(parse_ron(ron_code, &StructOptions::default()).is_err());
    }
}

pub(crate) fn parse_map_keys(ron: &str) -> Result<Vec<String>, GenerationError> {
    use linear_map::LinearMap;

    let map: LinearMap<String, RonValue> = ron::de::from_str(ron)
        .map_err(|err| GenerationError::DeserializationFailed(err.to_string()))?;

    Ok(map.into_iter().map(|pair| pair.0).collect())
}
