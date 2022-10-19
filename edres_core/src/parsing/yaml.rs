//! TODO

use serde_yaml::{self, Value as YamlValue};

use crate::{
    error::Error,
    options::ParseOptions,
    parsing,
    value::{Struct, Value},
};

/// TODO
pub fn parse_source(source: &str, options: &ParseOptions) -> Result<Value, Error> {
    let raw_value: YamlValue = serde_yaml::from_str(source)?;
    parse_value(raw_value, options)
}

/// TODO
pub fn parse_value(raw_value: YamlValue, options: &ParseOptions) -> Result<Value, Error> {
    let mut result = parse_value_non_unified(raw_value, options)?;
    parsing::unify_value(&mut result)?;
    Ok(result)
}

/// TODO
pub fn parse_value_non_unified(
    raw_value: YamlValue,
    options: &ParseOptions,
) -> Result<Value, Error> {
    Ok(match raw_value {
        YamlValue::Null => Value::Option(None),
        YamlValue::Bool(value) => Value::Bool(value),
        YamlValue::Number(value) => {
            if value.is_f64() {
                parsing::preferred_float(value.as_f64().unwrap(), options.default_float_size)
            } else if value.is_i64() {
                parsing::preferred_int(value.as_i64().unwrap() as i128, options.default_int_size)
            } else {
                parsing::preferred_int(value.as_u64().unwrap() as i128, options.default_int_size)
            }
        }
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

#[cfg(test)]
mod tests {
    use super::*;

    use pretty_assertions::assert_eq;

    #[test]
    fn value_conversion() {
        let yaml_source = r#"
            root_mapping:
                null_field: null
                bool_field: true
                signed_field: -100
                big_field: 18446744073709551615
                float_field: 2.5
                string_field: "hello"
                sequence:
                  - 1
                  - 2
                  - 3
                tagged: !Nice 69
            "#;

        let expected = Value::Struct(Struct(
            [(
                "root_mapping".into(),
                Value::Struct(Struct(
                    [
                        ("null_field".into(), Value::Option(None)),
                        ("bool_field".into(), Value::Bool(true)),
                        ("signed_field".into(), Value::I64(-100)),
                        ("big_field".into(), Value::I128(18446744073709551615)),
                        ("float_field".into(), Value::F64(2.5)),
                        ("string_field".into(), Value::String("hello".into())),
                        (
                            "sequence".into(),
                            Value::Vec(vec![Value::I64(1), Value::I64(2), Value::I64(3)]),
                        ),
                        ("tagged".into(), Value::I64(69)),
                    ]
                    .into_iter()
                    .collect(),
                )),
            )]
            .into_iter()
            .collect(),
        ));

        let value: YamlValue = serde_yaml::from_str(yaml_source).unwrap();
        let value = parse_value_non_unified(value, &Default::default()).unwrap();

        assert_eq!(value, expected);
    }
}
