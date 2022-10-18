pub mod gen;

#[cfg(test)]
mod de {
    pub fn json<T: for<'de> serde::Deserialize<'de>>(source: &str) -> T {
        serde_json::from_str(source).unwrap()
    }

    pub fn toml<T: for<'de> serde::Deserialize<'de>>(source: &str) -> T {
        toml::from_str(source).unwrap()
    }

    pub fn yaml<T: for<'de> serde::Deserialize<'de>>(source: &str) -> T {
        serde_yaml::from_str(source).unwrap()
    }
}

macro_rules! gen_tests {
    ($modname:ident, $ext:literal) => {
        #[cfg(test)]
        mod $modname {
            use super::de;

            #[test]
            fn deserialize_struct() {
                use crate::gen::$modname::Struct;

                let src =
                    std::fs::read_to_string(format!("data/{}/struct.{}", $ext, $ext)).unwrap();
                let data: Struct = de::$modname(&src);
                assert_eq!(data.number, 100_i64);
                assert_eq!(data.text, $ext);
                assert_eq!(data.nested.array.as_ref(), [1, 2, 3_i64]);
            }

            #[test]
            fn enum_keys() {
                use crate::gen::$modname::Enum;

                assert_eq!(Enum::ALL, &[Enum::Variant1, Enum::Variant2]);
                assert_eq!(Enum::Variant1.get().value, 1);
                assert_eq!(Enum::Variant2.get().value, 2);
            }

            #[test]
            fn deserialize_value_structs() {
                use crate::gen::$modname::{Enum, VStruct, DATA};
                use std::collections::HashMap;

                let src = std::fs::read_to_string(format!("data/{}/map.{}", $ext, $ext)).unwrap();

                if $ext == "toml" {
                    // NOTE: toml can't deserialize using an enum as
                    // a key, they're always treated as strings :(
                } else {
                    let map: HashMap<Enum, VStruct> = de::$modname(&src);
                    assert_eq!(map[&Enum::Variant1].value, 1);
                    assert_eq!(map[&Enum::Variant2].value, 2);
                }

                assert_eq!(DATA[0].value, 1);
                assert_eq!(DATA[1].value, 2);
            }

            #[test]
            fn file_enum() {
                use crate::gen::$modname::FileEnum;

                assert_eq!(FileEnum::ALL, &[FileEnum::FileA, FileEnum::FileB]);
                let path_a = format!("data/{}/files/file_a.{}", $ext, $ext);
                let path_b = format!("data/{}/files/file_b.{}", $ext, $ext);
                assert_eq!(FileEnum::FILE_PATHS, &[&path_a, &path_b]);
                assert_eq!(FileEnum::FileA.get().name, "file_a");
                assert_eq!(FileEnum::FileB.get().name, "file_b");
            }

            #[test]
            fn deserialize_file_structs() {
                use crate::gen::$modname::{FileEnum, FileStruct, FILE_VALUES};
                use std::collections::HashMap;

                // NOTE: We're cloning keys here because they would usually
                // be Copy, but can't be because I (lazily) wanted to
                // re-use the Options in build.rs for structs and enums.
                let map = FileEnum::ALL
                    .iter()
                    .map(|key| {
                        let path = key.clone().path();
                        let src = std::fs::read_to_string(&path).unwrap();
                        let value: FileStruct = de::$modname(&src);
                        (key.clone(), value)
                    })
                    .collect::<HashMap<_, _>>();

                assert_eq!(map[&FileEnum::FileA].name, "file_a");
                assert_eq!(map[&FileEnum::FileB].name, "file_b");
                assert_eq!(FILE_VALUES[0].name, "file_a");
                assert_eq!(FILE_VALUES[1].name, "file_b");
            }
        }
    };
}

gen_tests!(json, "json");
gen_tests!(toml, "toml");
gen_tests!(yaml, "yaml");
