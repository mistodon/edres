use toml::{self, Value};

use crate::{
    error::GenerationError,
    options::StructOptions,
    parsing,
    value::{GenericStruct, GenericValue},
};

pub fn parse_toml(toml: &str, options: &StructOptions) -> Result<GenericStruct, GenerationError> {
    use parsing::ParsedFields;

    let toml_struct: ParsedFields<Value> = toml::from_str(toml)
        .map_err(|err| GenerationError::DeserializationFailed(err.to_string()))?;

    let generic_struct = parsing::parsed_to_generic_struct(toml_struct, options, toml_to_raw_value);

    Ok(generic_struct)
}

fn toml_to_raw_value(
    super_struct: &str,
    super_key: &str,
    value: Value,
    options: &StructOptions,
) -> GenericValue {
    match value {
        Value::Boolean(value) => GenericValue::Bool(value),
        Value::Integer(value) => parsing::preferred_int(value, options.default_int_size),
        Value::Float(value) => parsing::preferred_float(value, options.default_float_size),
        Value::String(value) => GenericValue::String(value),
        Value::Datetime(value) => GenericValue::String(value.to_string()),
        Value::Array(values) => GenericValue::Array(
            values
                .into_iter()
                .map(|value| toml_to_raw_value(super_struct, super_key, value, options))
                .collect(),
        ),
        Value::Table(values) => {
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

pub fn parse_map_keys(toml: &str) -> Result<Vec<String>, GenerationError> {
    use linear_map::LinearMap;

    let map: LinearMap<String, Value> = toml::from_str(toml)
        .map_err(|err| GenerationError::DeserializationFailed(err.to_string()))?;

    Ok(map.into_iter().map(|pair| pair.0).collect())
}
