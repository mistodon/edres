use toml::{self, Value as TomlValue};

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
    let raw_value: TomlValue = toml::from_str(source).map_err(|err| WipError(err.to_string()))?;
    parse_value(raw_value, options)
}

pub fn parse_value(raw_value: TomlValue, options: &WipOptions) -> Result<Value, WipError> {
    let mut result = parse_value_non_unified(raw_value, options)?;
    parsing::unify_value(&mut result)?;
    Ok(result)
}

pub fn parse_value_non_unified(
    raw_value: TomlValue,
    options: &WipOptions,
) -> Result<Value, WipError> {
    Ok(match raw_value {
        TomlValue::Boolean(value) => Value::Bool(value),
        TomlValue::Integer(value) => {
            parsing::preferred_int2(value as i128, options.default_int_size)
        }
        TomlValue::Float(value) => parsing::preferred_float2(value, options.default_float_size),
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
                .collect::<Result<_, WipError>>()?,
        )),
    })
}

pub(crate) fn parse_toml(
    toml: &str,
    options: &StructOptions,
) -> Result<GenericStruct, GenerationError> {
    use parsing::ParsedFields;

    let toml_struct: ParsedFields<TomlValue> = toml::from_str(toml)
        .map_err(|err| GenerationError::DeserializationFailed(err.to_string()))?;

    let generic_struct = parsing::parsed_to_generic_struct(toml_struct, options, toml_to_raw_value);

    Ok(generic_struct)
}

fn toml_to_raw_value(
    super_struct: &str,
    super_key: &str,
    value: TomlValue,
    options: &StructOptions,
) -> GenericValue {
    match value {
        TomlValue::Boolean(value) => GenericValue::Bool(value),
        TomlValue::Integer(value) => parsing::preferred_int(value, options.default_int_size),
        TomlValue::Float(value) => parsing::preferred_float(value, options.default_float_size),
        TomlValue::String(value) => GenericValue::String(value),
        TomlValue::Datetime(value) => GenericValue::String(value.to_string()),
        TomlValue::Array(values) => GenericValue::Array(
            values
                .into_iter()
                .map(|value| toml_to_raw_value(super_struct, super_key, value, options))
                .collect(),
        ),
        TomlValue::Table(values) => {
            let sub_struct_name = format!("{}__{}", super_struct, super_key);
            let values = values
                .into_iter()
                .map(|(key, value)| {
                    let value = toml_to_raw_value(&sub_struct_name, &key, value, options);
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

pub(crate) fn parse_map_keys(toml: &str) -> Result<Vec<String>, GenerationError> {
    use linear_map::LinearMap;

    let map: LinearMap<String, TomlValue> = toml::from_str(toml)
        .map_err(|err| GenerationError::DeserializationFailed(err.to_string()))?;

    Ok(map.into_iter().map(|pair| pair.0).collect())
}
