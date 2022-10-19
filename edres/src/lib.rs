//! TODO

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

/// TODO
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

/// TODO
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

/// TODO
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

/// TODO
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

/// TODO
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

/// TODO
pub fn generate_enum_from_filenames<DirPath: AsRef<Path>, Name: AsRef<str>>(
    dir_path: DirPath,
    enum_name: Name,
    options: &Options,
) -> Result<String, Error> {
    let tokens =
        codegen::define_enum_from_filenames(dir_path.as_ref(), enum_name.as_ref(), options)?;
    Ok(tokens.to_string())
}

/// TODO
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

/// TODO
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

/// TODO
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

/// TODO
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

/// TODO
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

/// TODO
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
