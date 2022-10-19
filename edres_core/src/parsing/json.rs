//! TODO

use serde_json::{self, Value as JsonValue};

use crate::{
    error::Error,
    options::ParseOptions,
    parsing,
    value::{Struct, Value},
};

/// TODO
pub fn parse_source(source: &str, options: &ParseOptions) -> Result<Value, Error> {
    let raw_value: JsonValue = serde_json::from_str(source)?;
    parse_value(raw_value, options)
}

/// TODO
pub fn parse_value(raw_value: JsonValue, options: &ParseOptions) -> Result<Value, Error> {
    let mut result = parse_value_non_unified(raw_value, options)?;
    parsing::unify_value(&mut result)?;
    Ok(result)
}

/// TODO
pub fn parse_value_non_unified(
    raw_value: JsonValue,
    options: &ParseOptions,
) -> Result<Value, Error> {
    Ok(match raw_value {
        JsonValue::Null => Value::Option(None),
        JsonValue::Bool(value) => Value::Bool(value),
        JsonValue::Number(value) => match (value.as_i64(), value.as_u64(), value.as_f64()) {
            (Some(x), _, _) => parsing::preferred_int(x as i128, options.default_int_size),
            (None, Some(x), _) => parsing::preferred_int(x as i128, options.default_int_size),
            (None, None, Some(x)) => parsing::preferred_float(x, options.default_float_size),
            _ => return Err(Error::ErrorParsingNumber),
        },
        JsonValue::String(value) => Value::String(value),
        JsonValue::Array(values) => parsing::array_or_vec(
            values
                .into_iter()
                .map(|value| parse_value_non_unified(value, options))
                .collect::<Result<Vec<_>, _>>()?,
            options.max_array_size,
        ),
        JsonValue::Object(values) => Value::Struct(Struct(
            values
                .into_iter()
                .map(|(key, value)| {
                    parse_value_non_unified(value, options).map(|value| (key, value))
                })
                .collect::<Result<_, Error>>()?,
        )),
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    use pretty_assertions::assert_eq;

    #[test]
    fn value_conversion() {
        let json_source = r#"{
            "root_mapping": {
                "null_field": null,
                "bool_field": true,
                "signed_field": -100,
                "big_field": 18446744073709551615,
                "float_field": 2.5,
                "string_field": "hello",
                "sequence": [1, 2, 3]
            }
        }"#;

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
                    ]
                    .into_iter()
                    .collect(),
                )),
            )]
            .into_iter()
            .collect(),
        ));

        let value: JsonValue = serde_json::from_str(json_source).unwrap();
        let value = parse_value_non_unified(value, &Default::default()).unwrap();

        assert_eq!(value, expected);
    }
}
