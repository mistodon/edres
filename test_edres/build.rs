use edres::{gen, parsing, value::Value, SerdeSupport, WipOptions};

fn main() {
    build().unwrap();
}

fn build() -> Result<(), Box<dyn std::error::Error>> {
    let dirs = ["json", "toml", "yaml"];

    let options = WipOptions {
        derived_traits: vec![
            "Debug".into(),
            "Clone".into(),
            "PartialEq".into(),
            "Hash".into(),
            "Eq".into(),
        ]
        .into(),
        serde_support: SerdeSupport::Yes,
        ..WipOptions::default()
    };

    for dir in dirs {
        use std::fmt::Write;

        let mut buffer = String::new();

        // Methods to test:
        // - define_structs
        // - define_enum_from_keys
        // - define_structs_from_values
        // - define_enum_from_filenames
        // - define_structs_from_file_contents

        // define_structs
        {
            let path = format!("data/{}/struct.{}", dir, dir);
            let value = match parsing::parse_source_file(path.as_ref(), &options)? {
                Value::Struct(s) => s,
                _ => panic!("Not a struct!"),
            };

            let source = gen::define_structs(&value, "Struct", Some(path.as_ref()), &options)?;
            writeln!(&mut buffer, "{}", source.to_string())?;
        }

        // define_enum_from_keys
        {
            let path = format!("data/{}/map.{}", dir, dir);
            let value = match parsing::parse_source_file(path.as_ref(), &options)? {
                Value::Struct(s) => s,
                _ => panic!("Not a struct!"),
            };

            let source = gen::define_enum_from_keys(&value, "Enum", Some(path.as_ref()), &options)?;
            writeln!(&mut buffer, "{}", source.to_string())?;
        }

        // define_structs_from_values
        {
            let path = format!("data/{}/map.{}", dir, dir);
            let value = match parsing::parse_source_file(path.as_ref(), &options)? {
                Value::Struct(s) => s,
                _ => panic!("Not a struct!"),
            };

            let source = gen::define_structs_from_values(&value, "VStruct", &options)?;
            writeln!(&mut buffer, "{}", source.to_string())?;
        }

        // define_enum_from_filenames
        {
            let path = format!("data/{}/files", dir);
            let source = gen::define_enum_from_filenames(path.as_ref(), "FileEnum", &options)?;
            writeln!(&mut buffer, "{}", source.to_string())?;
        }

        // define_structs_from_file_contents
        {
            let path = format!("data/{}/files", dir);
            let source = gen::define_structs_from_file_contents(
                path.as_ref(),
                "FileStruct",
                None,
                &WipOptions {
                    all_values_const_name: Some("FILE_VALUES".into()),
                    ..options.clone()
                },
            )?;
            writeln!(&mut buffer, "{}", source.to_string())?;
        }

        std::fs::write(format!("src/gen/{}.rs", dir), buffer)?;
    }

    Ok(())
}
