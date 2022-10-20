//! If `serde` turns your structs into markup files,
//!
//! then `edres` turns your markup files into structs.
//!
//! This crate has three main related functionalities:
//!
//! 1.  Generate a set of structs representing the contents of
//!     a single markup file.
//! 2.  Generate an enum to represent the keys of a map in a
//!     single markup file (and optionally structs to represent
//!     the values).
//! 3.  Generate an enum to represent the files of a directory
//!     (and optionally structs to represent the contents of those
//!     files).
//!
//! The crate is mainly intended to be used in `build.rs` build
//! scripts, but could also be used inside a proc-macro. (See the
//! [`codegen`] module.)
//!
//! To use this crate, make sure you enable at least one of the
//! serde features to support the kinds of files you're using:
//!
//! 1. `json`
//! 2. `toml`
//! 3. `yaml`
//!
//! There are two sets of functions provided at the top level:
//! the `create_` functions which will write a Rust source file,
//! and the `generate_` functions which simply return Rust code as
//! a string.
//!
//! # Examples
//!
//! ## Generating structs
//!
//! This example takes a single config file and generates a struct
//! that is compatible with it:
//!
//! ```
//! # use edres::*;
//! # use quote::quote;
//! let my_file_content = "
//! title = \"My TOML file\"
//!
//! [my_table]
//! value = 10
//! ";
//!
//! let generated_code = generate_structs_from_source(
//!     my_file_content,
//!     "MyStruct",
//!     Format::Toml,
//!     &Options::minimal(),
//! ).unwrap();
//!
//! assert_eq!(generated_code, quote!(
//!     #[allow(non_camel_case_types)]
//!     pub struct MyStruct {
//!         pub title: std::borrow::Cow<'static, str>,
//!         pub my_table: MyStruct__my_table,
//!     }
//!
//!     #[allow(non_camel_case_types)]
//!     pub struct MyStruct__my_table {
//!         pub value: i64,
//!     }
//! ).to_string());
//! ```
//!
//! ## Generating enums
//!
//! This example takes a single config file and generates an enum
//! for the keys of the file, and structs for the values:
//!
//! ```
//! # use edres::*;
//! # use quote::quote;
//! let my_file_content = "
//!     Key1:
//!         name: First key
//!     Key2:
//!         name: Second key
//! ";
//!
//! let generated_code = generate_enum_from_source(
//!     my_file_content,
//!     "MyEnum",
//!     Format::Yaml,
//!     &Options {
//!         enums: EnumOptions {
//!             values_struct: Some(ValuesStructOptions::minimal()),
//!             ..EnumOptions::minimal()
//!         },
//!         ..Options::minimal()
//!     },
//! ).unwrap();
//!
//! assert_eq!(generated_code, quote!(
//!     pub enum MyEnum {
//!         Key1,
//!         Key2,
//!     }
//!
//!     #[allow(non_camel_case_types)]
//!     pub struct MyEnum__Value {
//!         pub name: std::borrow::Cow<'static, str>,
//!     }
//! ).to_string());
//! ```
//!
//! ## Generating file enums
//!
//! This example a directory full of JSON files and generates an
//! enum variant for each file name, and a struct to represent the
//! contents of them all.
//!
//! ```no_run
//! # use edres::*;
//! # use quote::quote;
//! /* Example JSON file:
//! {
//!     "name": "file1.json",
//!     "number": 1
//! }
//! */
//!
//! let generated_code = generate_enum_from_filenames(
//!     "./dir_full_of_json_files",
//!     "MyFileEnum",
//!     &Options {
//!         enums: EnumOptions {
//!             values_struct: Some(ValuesStructOptions::minimal()),
//!             ..EnumOptions::minimal()
//!         },
//!         ..Options::minimal()
//!     },
//! ).unwrap();
//!
//! assert_eq!(generated_code, quote!(
//!     pub enum MyFileEnum {
//!         File1,
//!         File2,
//!         File3,
//!     }
//!
//!     #[allow(non_camel_case_types)]
//!     pub struct MyFileEnum__Value {
//!         pub name: std::borrow::Cow<'static, str>,
//!         pub number: i32,
//!     }
//! ).to_string());
//! ```

mod files;

#[cfg(not(any(feature = "json", feature = "toml", feature = "yaml",)))]
compile_error!(
    "The edres crate requires at least one parsing feature to be enabled:\n {json, toml, yaml}"
);

use std::path::Path;

#[cfg(feature = "proc-macros")]
pub use edres_macros::{
    define_enums, define_enums_from_dirs, define_structs, define_structs_from_dirs,
};

pub use edres_core::*;

/// Generate Rust code that defines a set of structs based on a
/// given markup file.
pub fn generate_structs<SrcPath: AsRef<Path>, Name: AsRef<str>>(
    src_path: SrcPath,
    struct_name: Name,
    options: &Options,
) -> Result<String, Error> {
    let path = src_path.as_ref();
    let value = parsing::parse_source_file(path, &options.parse)?.assume_struct()?;
    let tokens = codegen::define_structs(&value, struct_name.as_ref(), Some(path), options)?;
    Ok(tokens.to_string())
}

/// Generate Rust code that defines a set of structs based on the
/// given markup source.
pub fn generate_structs_from_source<Source: AsRef<str>, Name: AsRef<str>>(
    source: Source,
    struct_name: Name,
    format: Format,
    options: &Options,
) -> Result<String, Error> {
    let value = parsing::parse_source(source.as_ref(), format, &options.parse)?.assume_struct()?;
    let tokens = codegen::define_structs(&value, struct_name.as_ref(), None, options)?;
    Ok(tokens.to_string())
}

/// Generate Rust code that defines a set of structs based on the
/// contents of the files in the given directory.
pub fn generate_structs_from_files<DirPath: AsRef<Path>, Name: AsRef<str>>(
    dir_path: DirPath,
    struct_name: Name,
    options: &Options,
) -> Result<String, Error> {
    let tokens = codegen::define_structs_from_file_contents(
        dir_path.as_ref(),
        struct_name.as_ref(),
        None,
        options,
    )?;
    Ok(tokens.to_string())
}

/// Generate Rust code that defines an enum based on the map keys
/// of the given markup file.
pub fn generate_enum<SrcPath: AsRef<Path>, Name: AsRef<str>>(
    src_path: SrcPath,
    enum_name: Name,
    options: &Options,
) -> Result<String, Error> {
    let path = src_path.as_ref();
    let value = parsing::parse_source_file(path, &options.parse)?.assume_struct()?;
    let tokens = codegen::define_enum_from_keys(&value, enum_name.as_ref(), Some(path), options)?;
    Ok(tokens.to_string())
}

/// Generate Rust code that defines an enum based on the map keys
/// of the given markup source.
pub fn generate_enum_from_source<Source: AsRef<str>, Name: AsRef<str>>(
    source: Source,
    enum_name: Name,
    format: Format,
    options: &Options,
) -> Result<String, Error> {
    let value = parsing::parse_source(source.as_ref(), format, &options.parse)?.assume_struct()?;
    let tokens = codegen::define_enum_from_keys(&value, enum_name.as_ref(), None, options)?;
    Ok(tokens.to_string())
}

/// Generate Rust code that defines an enum based on the file names
/// within the given directory.
pub fn generate_enum_from_filenames<DirPath: AsRef<Path>, Name: AsRef<str>>(
    dir_path: DirPath,
    enum_name: Name,
    options: &Options,
) -> Result<String, Error> {
    let tokens =
        codegen::define_enum_from_filenames(dir_path.as_ref(), enum_name.as_ref(), options)?;
    Ok(tokens.to_string())
}

/// Create a Rust source file that defines a set of structs
/// based on a given markup file.
pub fn create_structs<SrcPath: AsRef<Path>, DestPath: AsRef<Path>, Name: AsRef<str>>(
    src_path: SrcPath,
    dest_path: DestPath,
    struct_name: Name,
    options: &Options,
) -> Result<(), Error> {
    let output = generate_structs(src_path, struct_name, options)?;
    files::ensure_destination(dest_path.as_ref(), options.output.create_dirs)?;
    files::write_destination(
        dest_path.as_ref(),
        output,
        options.output.write_only_if_changed,
    )?;

    Ok(())
}

/// Create a Rust source file that defines a set of structs based
/// on the given markup source.
pub fn create_structs_from_source<Source: AsRef<str>, DestPath: AsRef<Path>, Name: AsRef<str>>(
    source: Source,
    dest_path: DestPath,
    struct_name: Name,
    format: Format,
    options: &Options,
) -> Result<(), Error> {
    let output = generate_structs_from_source(source, struct_name, format, options)?;
    files::ensure_destination(dest_path.as_ref(), options.output.create_dirs)?;
    files::write_destination(
        dest_path.as_ref(),
        output,
        options.output.write_only_if_changed,
    )?;

    Ok(())
}

/// Create a Rust source file that defines a set of structs based
/// on the contents of the files in the given directory.
pub fn create_structs_from_files<DirPath: AsRef<Path>, DestPath: AsRef<Path>, Name: AsRef<str>>(
    dir_path: DirPath,
    dest_path: DestPath,
    struct_name: Name,
    options: &Options,
) -> Result<(), Error> {
    let output = generate_structs_from_files(dir_path, struct_name, options)?;
    files::ensure_destination(dest_path.as_ref(), options.output.create_dirs)?;
    files::write_destination(
        dest_path.as_ref(),
        output,
        options.output.write_only_if_changed,
    )?;

    Ok(())
}

/// Create a Rust source file that defines an enum based on the
/// map keys of the given markup file.
pub fn create_enum<SrcPath: AsRef<Path>, DestPath: AsRef<Path>, Name: AsRef<str>>(
    src_path: SrcPath,
    dest_path: DestPath,
    enum_name: Name,
    options: &Options,
) -> Result<(), Error> {
    let output = generate_enum(src_path, enum_name, options)?;
    files::ensure_destination(dest_path.as_ref(), options.output.create_dirs)?;
    files::write_destination(
        dest_path.as_ref(),
        output,
        options.output.write_only_if_changed,
    )?;

    Ok(())
}

/// Create a Rust source file that defines an enum based on the
/// map keys of the given markup source.
pub fn create_enum_from_source<Source: AsRef<str>, DestPath: AsRef<Path>, Name: AsRef<str>>(
    source: Source,
    dest_path: DestPath,
    enum_name: Name,
    format: Format,
    options: &Options,
) -> Result<(), Error> {
    let output = generate_enum_from_source(source, enum_name, format, options)?;
    files::ensure_destination(dest_path.as_ref(), options.output.create_dirs)?;
    files::write_destination(
        dest_path.as_ref(),
        output,
        options.output.write_only_if_changed,
    )?;

    Ok(())
}

/// Create a Rust source file that defines an enum based on the
/// file names within the given directory.
pub fn create_enum_from_filenames<DirPath: AsRef<Path>, DestPath: AsRef<Path>, Name: AsRef<str>>(
    dir_path: DirPath,
    dest_path: DestPath,
    enum_name: Name,
    options: &Options,
) -> Result<(), Error> {
    let output = generate_enum_from_filenames(dir_path, enum_name, options)?;
    files::ensure_destination(dest_path.as_ref(), options.output.create_dirs)?;
    files::write_destination(
        dest_path.as_ref(),
        output,
        options.output.write_only_if_changed,
    )?;

    Ok(())
}
