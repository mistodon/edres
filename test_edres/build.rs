use edres::{codegen, parsing, value::Value, Options, StructOptions};

fn main() {
    build().unwrap();
}

fn build() -> Result<(), Box<dyn std::error::Error>> {
    panic!("{}", std::env::current_dir()?.display());
    let dirs = ["json", "toml", "yaml"];

    let options = Options::serde_default();

    for dir in dirs {
        use std::fmt::Write;

        let mut buffer = String::new();

        // define_structs
        {
            let path = format!("data/{}/struct.{}", dir, dir);
            let value = match parsing::parse_source_file(path.as_ref(), &options.parse)? {
                Value::Struct(s) => s,
                _ => panic!("Not a struct!"),
            };

            let source = codegen::define_structs(&value, "Struct", Some(path.as_ref()), &options)?;
            writeln!(&mut buffer, "{}", source)?;
        }

        // define_enum_from_keys
        {
            let path = format!("data/{}/map.{}", dir, dir);
            let value = match parsing::parse_source_file(path.as_ref(), &options.parse)? {
                Value::Struct(s) => s,
                _ => panic!("Not a struct!"),
            };

            let source =
                codegen::define_enum_from_keys(&value, "Enum", Some(path.as_ref()), &options)?;
            writeln!(&mut buffer, "{}", source)?;
        }

        // define_structs_from_values
        {
            let path = format!("data/{}/map.{}", dir, dir);
            let value = match parsing::parse_source_file(path.as_ref(), &options.parse)? {
                Value::Struct(s) => s,
                _ => panic!("Not a struct!"),
            };

            let source = codegen::define_structs_from_values(&value, "VStruct", &options)?;
            writeln!(&mut buffer, "{}", source)?;
        }

        // define_enum_from_filenames
        {
            let path = format!("data/{}/files", dir);
            let source = codegen::define_enum_from_filenames(path.as_ref(), "FileEnum", &options)?;
            writeln!(&mut buffer, "{}", source)?;
        }

        // define_structs_from_file_contents
        {
            let path = format!("data/{}/files", dir);
            let source = codegen::define_structs_from_file_contents(
                path.as_ref(),
                "FileStruct",
                None,
                &Options {
                    structs: StructOptions {
                        struct_data_const_name: Some("FILE_VALUES".into()),
                        ..Default::default()
                    },
                    ..options.clone()
                },
            )?;
            writeln!(&mut buffer, "{}", source)?;
        }

        std::fs::write(format!("src/gen/{}.rs", dir), buffer)?;
    }

    Ok(())
}
