use edres::{Format, Options};
use proc_macro2::TokenStream;
use quote::quote;

fn assert_str(s: String, q: TokenStream) {
    assert_eq!(s, q.to_string());
}

fn assert_file(p: &str, q: TokenStream) {
    let s = std::fs::read_to_string(p).unwrap();
    assert_str(s, q);
}

#[test]
pub fn generate_structs() {
    let s =
        edres::generate_structs("tests/data/struct.yaml", "Struct", &Options::minimal()).unwrap();
    assert_str(
        s,
        quote!(
            #[allow(non_camel_case_types)]
            pub struct Struct {
                pub name: std::borrow::Cow<'static, str>,
                pub number: i64,
            }
        ),
    );
}

#[test]
pub fn generate_structs_from_source() {
    let source = include_str!("data/struct.yaml");
    let s =
        edres::generate_structs_from_source(source, "Struct", Format::Yaml, &Options::minimal())
            .unwrap();
    assert_str(
        s,
        quote!(
            #[allow(non_camel_case_types)]
            pub struct Struct {
                pub name: std::borrow::Cow<'static, str>,
                pub number: i64,
            }
        ),
    );
}

#[test]
pub fn generate_structs_from_files() {
    let s = edres::generate_structs_from_files("tests/data/files", "Struct", &Options::minimal())
        .unwrap();
    assert_str(
        s,
        quote!(
            #[allow(non_camel_case_types)]
            pub struct Struct {
                pub name: std::borrow::Cow<'static, str>,
                pub letter: std::borrow::Cow<'static, str>,
            }
        ),
    );
}

#[test]
pub fn generate_enum() {
    let s = edres::generate_enum("tests/data/enum.yaml", "Enum", &Options::minimal()).unwrap();
    assert_str(
        s,
        quote!(
            pub enum Enum {
                First,
                Second,
            }
        ),
    );
}

#[test]
pub fn generate_enum_from_source() {
    let source = include_str!("data/enum.yaml");
    let s = edres::generate_enum_from_source(source, "Enum", Format::Yaml, &Options::minimal())
        .unwrap();
    assert_str(
        s,
        quote!(
            pub enum Enum {
                First,
                Second,
            }
        ),
    );
}

#[test]
pub fn generate_enum_from_filenames() {
    let s = edres::generate_enum_from_filenames("tests/data/files", "Enum", &Options::minimal())
        .unwrap();
    assert_str(
        s,
        quote!(
            pub enum Enum {
                Alpha,
                Beta,
            }
        ),
    );
}

#[test]
pub fn create_structs() {
    edres::create_structs(
        "tests/data/struct.yaml",
        "tests/output/test1.rs",
        "Struct",
        &Options::minimal(),
    )
    .unwrap();
    assert_file(
        "tests/output/test1.rs",
        quote!(
            #[allow(non_camel_case_types)]
            pub struct Struct {
                pub name: std::borrow::Cow<'static, str>,
                pub number: i64,
            }
        ),
    );
}

#[test]
pub fn create_structs_from_source() {
    let source = include_str!("data/struct.yaml");
    edres::create_structs_from_source(
        source,
        "tests/output/test2.rs",
        "Struct",
        Format::Yaml,
        &Options::minimal(),
    )
    .unwrap();
    assert_file(
        "tests/output/test2.rs",
        quote!(
            #[allow(non_camel_case_types)]
            pub struct Struct {
                pub name: std::borrow::Cow<'static, str>,
                pub number: i64,
            }
        ),
    );
}

#[test]
pub fn create_structs_from_files() {
    edres::create_structs_from_files(
        "tests/data/files",
        "tests/output/test3.rs",
        "Struct",
        &Options::minimal(),
    )
    .unwrap();
    assert_file(
        "tests/output/test3.rs",
        quote!(
            #[allow(non_camel_case_types)]
            pub struct Struct {
                pub name: std::borrow::Cow<'static, str>,
                pub letter: std::borrow::Cow<'static, str>,
            }
        ),
    );
}

#[test]
pub fn create_enum() {
    edres::create_enum(
        "tests/data/enum.yaml",
        "tests/output/test4.rs",
        "Enum",
        &Options::minimal(),
    )
    .unwrap();
    assert_file(
        "tests/output/test4.rs",
        quote!(
            pub enum Enum {
                First,
                Second,
            }
        ),
    );
}

#[test]
pub fn create_enum_from_source() {
    let source = include_str!("data/enum.yaml");
    edres::create_enum_from_source(
        source,
        "tests/output/test5.rs",
        "Enum",
        Format::Yaml,
        &Options::minimal(),
    )
    .unwrap();
    assert_file(
        "tests/output/test5.rs",
        quote!(
            pub enum Enum {
                First,
                Second,
            }
        ),
    );
}

#[test]
pub fn create_enum_from_filenames() {
    edres::create_enum_from_filenames(
        "tests/data/files",
        "tests/output/test6.rs",
        "Enum",
        &Options::minimal(),
    )
    .unwrap();
    assert_file(
        "tests/output/test6.rs",
        quote!(
            pub enum Enum {
                Alpha,
                Beta,
            }
        ),
    );
}
