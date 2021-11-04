use serde_yaml::{self, Value as YamlValue};

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
    let raw_value: YamlValue =
        serde_yaml::from_str(source).map_err(|err| WipError(err.to_string()))?;
    parse_value(raw_value, options)
}

pub fn parse_value(raw_value: YamlValue, options: &WipOptions) -> Result<Value, WipError> {
    let mut result = parse_value_non_unified(raw_value, options)?;
    parsing::unify_value(&mut result)?;
    Ok(result)
}

pub fn parse_value_non_unified(
    raw_value: YamlValue,
    options: &WipOptions,
) -> Result<Value, WipError> {
    Ok(match raw_value {
        YamlValue::Null => Value::Option(None),
        YamlValue::Bool(value) => Value::Bool(value),
        YamlValue::Number(value) => match (value.as_i64(), value.as_u64(), value.as_f64()) {
            (Some(x), _, _) => parsing::preferred_int2(x as i128, options.default_int_size),
            (None, Some(x), _) => parsing::preferred_int2(x as i128, options.default_int_size),
            (None, None, Some(x)) => parsing::preferred_float2(x, options.default_float_size),
            _ => return Err(WipError("Failed to parse number".into())),
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
                    let key = key
                        .as_str()
                        .ok_or(WipError("Could not read key".into()))?
                        .to_owned();
                    parse_value_non_unified(value, options).map(|value| (key, value))
                })
                .collect::<Result<_, WipError>>()?,
        )),
    })
}

pub(crate) fn parse_yaml(
    yaml: &str,
    options: &StructOptions,
) -> Result<GenericStruct, GenerationError> {
    use parsing::ParsedFields;

    let yaml_struct: ParsedFields<YamlValue> = serde_yaml::from_str(yaml)
        .map_err(|err| GenerationError::DeserializationFailed(err.to_string()))?;

    let generic_struct = parsing::parsed_to_generic_struct(yaml_struct, options, yaml_to_raw_value);

    Ok(generic_struct)
}

fn yaml_to_raw_value(
    super_struct: &str,
    super_key: &str,
    value: YamlValue,
    options: &StructOptions,
) -> GenericValue {
    match value {
        YamlValue::Null => GenericValue::Option(None),
        YamlValue::Bool(value) => GenericValue::Bool(value),
        YamlValue::Number(value) => match (value.as_i64(), value.as_u64(), value.as_f64()) {
            // TODO: Add some unit tests for this
            (Some(x), _, _) => parsing::preferred_int(x, options.default_int_size),
            (None, Some(x), _) => GenericValue::U64(x),
            (None, None, Some(x)) => parsing::preferred_float(x, options.default_float_size),
            _ => unimplemented!("Should handle error here"), // TODO
        },
        YamlValue::String(value) => GenericValue::String(value),
        YamlValue::Sequence(values) => GenericValue::Array(
            values
                .into_iter()
                .map(|value| yaml_to_raw_value(super_struct, super_key, value, options))
                .collect(),
        ),
        YamlValue::Mapping(values) => {
            let sub_struct_name = format!("{}__{}", super_struct, super_key);
            let values = values
                .into_iter()
                .map(|(key, value)| {
                    let key = key.as_str().unwrap().to_owned();
                    let value = yaml_to_raw_value(&sub_struct_name, &key, value, options);
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

pub(crate) fn parse_map_keys(yaml: &str) -> Result<Vec<String>, GenerationError> {
    use linear_map::LinearMap;

    let map: LinearMap<String, YamlValue> = serde_yaml::from_str(yaml)
        .map_err(|err| GenerationError::DeserializationFailed(err.to_string()))?;

    Ok(map.into_iter().map(|pair| pair.0).collect())
}
