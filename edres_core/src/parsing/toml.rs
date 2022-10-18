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

#[cfg(test)]
mod tests {
    use super::*;

    use pretty_assertions::assert_eq;

    #[test]
    fn value_conversion() {
        let toml_source = r#"
            [root_table]
            bool_field = true
            int_field = 1000
            float_field = 2.5
            string_field = "hello"
            datetime_field = 2022-12-22T22:22:22Z
            array_field = [1, 2, 3]
        "#;

        let expected = Value::Struct(Struct(
            [(
                "root_table".into(),
                Value::Struct(Struct(
                    [
                        ("bool_field".into(), Value::Bool(true)),
                        ("int_field".into(), Value::I64(1000)),
                        ("float_field".into(), Value::F64(2.5)),
                        ("string_field".into(), Value::String("hello".into())),
                        (
                            "datetime_field".into(),
                            Value::String("2022-12-22T22:22:22Z".into()),
                        ),
                        (
                            "array_field".into(),
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

        let value: TomlValue = toml::from_str(toml_source).unwrap();
        let value = parse_value_non_unified(value, &Default::default()).unwrap();

        assert_eq!(value, expected);
    }
}
