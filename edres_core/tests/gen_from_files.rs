use pretty_assertions::assert_eq;
use proc_macro2::TokenStream;
use quote::quote;

use edres_core::{codegen, options::*};

fn assert_tokens(a: TokenStream, b: TokenStream) {
    assert_eq!(a.to_string(), b.to_string())
}

#[test]
fn enum_from_filenames() {
    let result = codegen::define_enum_from_filenames(
        "tests/yamls".as_ref(),
        "FileName",
        &Options::minimal(),
    )
    .unwrap();
    assert_tokens(
        result,
        quote! {
            pub enum FileName {
                FileA,
                FileB,
            }
        },
    );
}

#[test]
fn enum_from_filenames_with_values() {
    let result = codegen::define_enum_from_filenames(
        "tests/yamls".as_ref(),
        "FileName",
        &Options {
            enums: EnumOptions {
                all_values_const_name: Some("VALUES".into()),
                values_struct: Some(ValuesStructOptions::minimal()),
                ..EnumOptions::minimal()
            },
            ..Options::minimal()
        },
    )
    .unwrap();
    assert_tokens(
        result,
        quote! {
            pub enum FileName {
                FileA,
                FileB,
            }

            impl FileName {
                pub const VALUES: &'static [FileName__Value] = &[
                    FileName__Value {
                        number: 100i64,
                        text: std::borrow::Cow::Borrowed("one hundred"),
                        nested: FileName__Value__nested {
                            text_again: std::borrow::Cow::Borrowed("one hundred"),
                        },
                    },
                    FileName__Value {
                        number: 200i64,
                        text: std::borrow::Cow::Borrowed("two hundred"),
                        nested: FileName__Value__nested {
                            text_again: std::borrow::Cow::Borrowed("two hundred"),
                        },
                    },
                ];
            }

            #[allow(non_camel_case_types)]
            pub struct FileName__Value {
                pub number: i64,
                pub text: std::borrow::Cow<'static, str>,
                pub nested: FileName__Value__nested,
            }

            #[allow(non_camel_case_types)]
            pub struct FileName__Value__nested {
                pub text_again: std::borrow::Cow<'static, str>,
            }
        },
    );
}

#[test]
fn structs_from_file_contents() {
    let result = codegen::define_structs_from_file_contents(
        "tests/yamls".as_ref(),
        "FileContent",
        None,
        &Options {
            structs: StructOptions {
                struct_data_const_name: Some("DATA".into()),
                ..StructOptions::minimal()
            },
            ..Options::minimal()
        },
    )
    .unwrap();
    assert_tokens(
        result,
        quote! {
            #[allow(non_camel_case_types)]
            pub struct FileContent {
                pub number: i64,
                pub text: std::borrow::Cow<'static, str>,
                pub nested: FileContent__nested,
            }

            #[allow(non_camel_case_types)]
            pub struct FileContent__nested {
                pub text_again: std::borrow::Cow<'static, str>,
            }

            pub const DATA: &[FileContent] = &[
                FileContent {
                    number: 100i64,
                    text: std::borrow::Cow::Borrowed("one hundred"),
                    nested: FileContent__nested {
                        text_again: std::borrow::Cow::Borrowed("one hundred"),
                    },
                },
                FileContent {
                    number: 200i64,
                    text: std::borrow::Cow::Borrowed("two hundred"),
                    nested: FileContent__nested {
                        text_again: std::borrow::Cow::Borrowed("two hundred"),
                    },
                },
            ];
        },
    );
}

#[test]
fn enum_from_filenames_with_consts() {
    let result = codegen::define_enum_from_filenames(
        "tests/yamls".as_ref(),
        "FileName",
        &Options {
            files: FilesOptions {
                file_strings_const_name: Some("STRINGS".into()),
                get_string_fn_name: Some("string".into()),
                file_bytes_const_name: Some("BYTES".into()),
                get_bytes_fn_name: Some("bytes".into()),
                ..FilesOptions::default()
            },
            enums: EnumOptions {
                ..EnumOptions::minimal()
            },
            ..Options::new()
        },
    )
    .unwrap();
    assert_tokens(
        result,
        quote! {
            pub enum FileName {
                FileA,
                FileB,
            }

            impl FileName {
                pub const FILE_PATHS: &'static [&'static str] = &[
                    "tests/yamls/file_a.yaml",
                    "tests/yamls/file_b.yaml",
                ];
                pub const fn path(self) -> &'static str {
                    Self::FILE_PATHS[self as usize]
                }
                pub const BYTES: &'static [&'static [u8]] = &[
                    include_bytes!("tests/yamls/file_a.yaml"),
                    include_bytes!("tests/yamls/file_b.yaml"),
                ];
                pub const fn bytes(self) -> &'static [u8] {
                    Self::BYTES[self as usize]
                }
                pub const STRINGS: &'static [&'static str] = &[
                    include_str!("tests/yamls/file_a.yaml"),
                    include_str!("tests/yamls/file_b.yaml"),
                ];
                pub const fn string(self) -> &'static str {
                    Self::STRINGS[self as usize]
                }
                pub const SOURCE_PATH: &'static str = "tests/yamls";
            }
        },
    );
}
