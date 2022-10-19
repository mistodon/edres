//! This module contains the structs for configuring the behaviour of the public APIs.
//!
//! The main struct of this module is `Options` which contains
//! all of the different configuration options.
//!
//! Most structs have a `new` constructor which contains sensible
//! defaults, as well as a `minimal` constructor which generates
//! as little code as possible.

use std::borrow::Cow;

/// Contains the full set of options for all public APIs
/// in this crate.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Options {
    /// TODO
    pub source_path_const_name: Option<Cow<'static, str>>,

    /// Controls whether generated items should derive `serde` traits.
    pub serde_support: SerdeSupport,

    /// TODO
    pub parse: ParseOptions,

    /// TODO
    pub structs: StructOptions,

    /// TODO
    pub enums: EnumOptions,

    /// TODO
    pub files: FilesOptions,

    /// TODO
    pub output: OutputOptions,
}

impl Options {
    /// # Examples
    /// ```
    /// # use edres_core::options::*;
    /// assert_eq!(Options::new(), Options {
    ///     source_path_const_name: Some("SOURCE_PATH".into()),
    ///     serde_support: SerdeSupport::No,
    ///     parse: ParseOptions::new(),
    ///     structs: StructOptions::new(),
    ///     enums: EnumOptions::new(),
    ///     files: FilesOptions::new(),
    ///     output: OutputOptions::new(),
    /// });
    /// ```
    pub const fn new() -> Options {
        Options {
            source_path_const_name: Some(Cow::Borrowed("SOURCE_PATH")),
            serde_support: SerdeSupport::No,

            parse: ParseOptions::new(),
            structs: StructOptions::new(),
            enums: EnumOptions::new(),
            files: FilesOptions::new(),
            output: OutputOptions::new(),
        }
    }

    /// # Examples
    /// ```
    /// # use edres_core::options::*;
    /// assert_eq!(Options::serde_default(), Options {
    ///     source_path_const_name: Some("SOURCE_PATH".into()),
    ///     serde_support: SerdeSupport::Yes,
    ///     parse: ParseOptions::new(),
    ///     structs: StructOptions::new(),
    ///     enums: EnumOptions::new(),
    ///     files: FilesOptions::new(),
    ///     output: OutputOptions::new(),
    /// });
    /// ```
    pub const fn serde_default() -> Options {
        Options {
            source_path_const_name: Some(Cow::Borrowed("SOURCE_PATH")),
            serde_support: SerdeSupport::Yes,

            parse: ParseOptions::new(),
            structs: StructOptions::new(),
            enums: EnumOptions::new(),
            files: FilesOptions::new(),
            output: OutputOptions::new(),
        }
    }

    /// ```
    /// # use edres_core::options::*;
    /// assert_eq!(Options::minimal(), Options {
    ///     source_path_const_name: None,
    ///     serde_support: SerdeSupport::No,
    ///     parse: ParseOptions::new(),
    ///     structs: StructOptions::minimal(),
    ///     enums: EnumOptions::minimal(),
    ///     files: FilesOptions::minimal(),
    ///     output: OutputOptions::new(),
    /// });
    /// ```
    pub const fn minimal() -> Options {
        Options {
            source_path_const_name: None,
            serde_support: SerdeSupport::No,

            parse: ParseOptions::new(),
            structs: StructOptions::minimal(),
            enums: EnumOptions::minimal(),
            files: FilesOptions::minimal(),
            output: OutputOptions::new(),
        }
    }
}

impl Default for Options {
    /// # Examples
    /// ```
    /// # use edres_core::options::*;
    /// assert_eq!(Options::default(), Options::new());
    /// ```
    fn default() -> Self {
        Self::new()
    }
}

/// Options specific to how `edres` should parse markup.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ParseOptions {
    /// The floating point type to infer from input.
    ///
    /// This can be either `f32` or `f64`. Values that are too
    /// large to fit the chosen default will instead be inferred
    /// as a larger type.
    pub default_float_size: FloatSize,

    /// The integer type to infer from input.
    ///
    /// This can be anything from `i8` to `i128`, including
    /// `isize`. Values that are too large to fit the chosen
    /// default will instead be inferred as a larger type.
    pub default_int_size: IntSize,

    /// What size of sequence, if any, to consider small enough
    /// to use an array instead of a `Vec`.
    ///
    /// For example, if `Some(4)` is provided, then sequences of
    /// more than 4 items in the input will generate a `Vec` in
    /// the resulting struct. Meanwhile, a sequence of 4 values
    /// would instead generate a `[T; 4]`.
    pub max_array_size: Option<usize>,
}

impl ParseOptions {
    /// # Examples
    /// ```
    /// # use edres_core::options::*;
    /// assert_eq!(ParseOptions::new(), ParseOptions {
    ///     default_float_size: FloatSize::F64,
    ///     default_int_size: IntSize::I64,
    ///     max_array_size: None,
    /// });
    /// ```
    pub const fn new() -> Self {
        ParseOptions {
            default_float_size: FloatSize::F64,
            default_int_size: IntSize::I64,
            max_array_size: None,
        }
    }
}

impl Default for ParseOptions {
    /// # Examples
    /// ```
    /// # use edres_core::options::*;
    /// assert_eq!(ParseOptions::default(), ParseOptions::new());
    /// ```
    fn default() -> Self {
        ParseOptions::new()
    }
}

/// Options specific to how `edres` should generate structs.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct StructOptions {
    /// A list of traits to derive.
    ///
    /// These can either be unqualified (like `Clone`) or
    /// qualifier with a crate name (like `serde::Serialize`).
    ///
    /// See the `StructOptions::new` example to see how to easily
    /// set this value.
    pub derived_traits: Cow<'static, [Cow<'static, str>]>,

    /// TODO
    pub struct_data_const_name: Option<Cow<'static, str>>,
}

impl StructOptions {
    /// # Examples
    /// ```
    /// # use edres_core::options::*;
    /// assert_eq!(StructOptions::new(), StructOptions {
    ///     derived_traits: vec!["Debug".into()].into(),
    ///     struct_data_const_name: Some("DATA".into()),
    /// });
    /// ```
    pub const fn new() -> StructOptions {
        StructOptions {
            derived_traits: Cow::Borrowed(&[Cow::Borrowed("Debug")]),
            struct_data_const_name: Some(Cow::Borrowed("DATA")),
        }
    }

    /// # Examples
    /// ```
    /// # use edres_core::options::*;
    /// assert_eq!(StructOptions::minimal(), StructOptions {
    ///     derived_traits: vec![].into(),
    ///     struct_data_const_name: None,
    /// });
    /// ```
    pub const fn minimal() -> StructOptions {
        StructOptions {
            derived_traits: Cow::Borrowed(&[]),
            struct_data_const_name: None,
        }
    }
}

impl Default for StructOptions {
    /// # Examples
    /// ```
    /// # use edres_core::options::*;
    /// assert_eq!(StructOptions::default(), StructOptions::new());
    /// ```
    fn default() -> Self {
        Self::new()
    }
}

/// Options specific to how `edres` should generate enums.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct EnumOptions {
    /// A list of traits to derive.
    ///
    /// These can either be unqualified (like `Clone`) or
    /// qualifier with a crate name (like `serde::Serialize`).
    ///
    /// See the `EnumOptions::new` example to see how to easily
    /// set this value.
    pub derived_traits: Cow<'static, [Cow<'static, str>]>,

    /// Whether generated enums should implement the `Default`
    /// trait.
    ///
    /// This uses the first variant as the default value.
    pub impl_default: bool,

    /// Whether generated enums should implement `Display`.
    ///
    /// This just displays the name of the variant as a string.
    /// For example, `MyEnum::First.to_string() == "First"`.
    pub impl_display: bool,

    /// Whether generated enums should implement `FromStr`.
    ///
    /// This works by matching the name of the variant.
    /// For example, `"First".parse().unwrap() == MyEnum::First`.
    pub impl_from_str: bool,

    /// TODO
    pub all_variants_const_name: Option<Cow<'static, str>>,

    /// TODO
    pub all_values_const_name: Option<Cow<'static, str>>,

    /// TODO
    pub values_struct_name: Option<Cow<'static, str>>,

    /// TODO
    pub values_struct_options: StructOptions,

    /// TODO
    pub get_value_fn_name: Option<Cow<'static, str>>,
}

impl EnumOptions {
    /// # Examples
    /// ```
    /// # use edres_core::options::*;
    /// assert_eq!(EnumOptions::new(), EnumOptions {
    ///     derived_traits: vec![
    ///         "Debug".into(),
    ///         "Clone".into(),
    ///         "Copy".into(),
    ///         "PartialEq".into(),
    ///         "Eq".into(),
    ///         "Hash".into(),
    ///     ].into(),
    ///     impl_default: true,
    ///     impl_display: true,
    ///     impl_from_str: true,
    ///     all_variants_const_name: Some("ALL".into()),
    ///     all_values_const_name: Some("VALUES".into()),
    ///     values_struct_name: None,
    ///     values_struct_options: StructOptions::new(),
    ///     get_value_fn_name: Some("get".into()),
    /// });
    /// ```
    pub const fn new() -> EnumOptions {
        EnumOptions {
            derived_traits: Cow::Borrowed(&[
                Cow::Borrowed("Debug"),
                Cow::Borrowed("Clone"),
                Cow::Borrowed("Copy"),
                Cow::Borrowed("PartialEq"),
                Cow::Borrowed("Eq"),
                Cow::Borrowed("Hash"),
            ]),
            impl_default: true,
            impl_display: true,
            impl_from_str: true,
            all_variants_const_name: Some(Cow::Borrowed("ALL")),
            all_values_const_name: Some(Cow::Borrowed("VALUES")),
            values_struct_name: None,
            values_struct_options: StructOptions::new(),
            get_value_fn_name: Some(Cow::Borrowed("get")),
        }
    }

    /// # Examples
    /// ```
    /// # use edres_core::options::*;
    /// assert_eq!(EnumOptions::minimal(), EnumOptions {
    ///     derived_traits: vec![].into(),
    ///     impl_default: false,
    ///     impl_display: false,
    ///     impl_from_str: false,
    ///     all_variants_const_name: None,
    ///     all_values_const_name: None,
    ///     values_struct_name: None,
    ///     values_struct_options: StructOptions::minimal(),
    ///     get_value_fn_name: None,
    /// });
    /// ```
    pub const fn minimal() -> EnumOptions {
        EnumOptions {
            derived_traits: Cow::Borrowed(&[]),
            impl_default: false,
            impl_display: false,
            impl_from_str: false,
            all_variants_const_name: None,
            all_values_const_name: None,
            values_struct_name: None,
            values_struct_options: StructOptions::minimal(),
            get_value_fn_name: None,
        }
    }
}

impl Default for EnumOptions {
    /// # Examples
    /// ```
    /// # use edres_core::options::*;
    /// assert_eq!(EnumOptions::default(), EnumOptions::new());
    /// ```
    fn default() -> Self {
        Self::new()
    }
}

/// TODO
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FilesOptions {
    /// TODO
    pub file_paths_const_name: Option<Cow<'static, str>>,

    /// TODO
    pub get_path_fn_name: Option<Cow<'static, str>>,

    /// TODO
    pub file_strings_const_name: Option<Cow<'static, str>>,

    /// TODO
    pub get_string_fn_name: Option<Cow<'static, str>>,

    /// TODO
    pub file_bytes_const_name: Option<Cow<'static, str>>,

    /// TODO
    pub get_bytes_fn_name: Option<Cow<'static, str>>,
}

impl FilesOptions {
    /// # Examples
    /// ```
    /// # use edres_core::options::*;
    /// assert_eq!(FilesOptions::new(), FilesOptions {
    ///     file_paths_const_name: Some("FILE_PATHS".into()),
    ///     get_path_fn_name: Some("path".into()),
    ///     file_strings_const_name: None,
    ///     get_string_fn_name: None,
    ///     file_bytes_const_name: None,
    ///     get_bytes_fn_name: None,
    /// });
    /// ```
    pub const fn new() -> FilesOptions {
        FilesOptions {
            file_paths_const_name: Some(Cow::Borrowed("FILE_PATHS")),
            get_path_fn_name: Some(Cow::Borrowed("path")),
            file_strings_const_name: None,
            get_string_fn_name: None,
            file_bytes_const_name: None,
            get_bytes_fn_name: None,
        }
    }

    /// # Examples
    /// ```
    /// # use edres_core::options::*;
    /// assert_eq!(FilesOptions::minimal(), FilesOptions {
    ///     file_paths_const_name: None,
    ///     get_path_fn_name: None,
    ///     file_strings_const_name: None,
    ///     get_string_fn_name: None,
    ///     file_bytes_const_name: None,
    ///     get_bytes_fn_name: None,
    /// });
    /// ```
    pub const fn minimal() -> FilesOptions {
        FilesOptions {
            file_paths_const_name: None,
            get_path_fn_name: None,
            file_strings_const_name: None,
            get_string_fn_name: None,
            file_bytes_const_name: None,
            get_bytes_fn_name: None,
        }
    }

    /// # Examples
    /// ```
    /// # use edres_core::options::*;
    /// assert_eq!(FilesOptions::file_bytes(), FilesOptions {
    ///     file_paths_const_name: None,
    ///     get_path_fn_name: None,
    ///     file_strings_const_name: None,
    ///     get_string_fn_name: None,
    ///     file_bytes_const_name: Some("FILE_BYTES".into()),
    ///     get_bytes_fn_name: Some("bytes".into()),
    /// });
    /// ```
    pub const fn file_bytes() -> FilesOptions {
        FilesOptions {
            file_paths_const_name: None,
            get_path_fn_name: None,
            file_strings_const_name: None,
            get_string_fn_name: None,
            file_bytes_const_name: Some(Cow::Borrowed("FILE_BYTES")),
            get_bytes_fn_name: Some(Cow::Borrowed("bytes")),
        }
    }

    /// # Examples
    /// ```
    /// # use edres_core::options::*;
    /// assert_eq!(FilesOptions::file_strings(), FilesOptions {
    ///     file_paths_const_name: None,
    ///     get_path_fn_name: None,
    ///     file_strings_const_name: Some("FILE_STRINGS".into()),
    ///     get_string_fn_name: Some("string".into()),
    ///     file_bytes_const_name: None,
    ///     get_bytes_fn_name: None,
    /// });
    /// ```
    pub const fn file_strings() -> FilesOptions {
        FilesOptions {
            file_paths_const_name: None,
            get_path_fn_name: None,
            file_strings_const_name: Some(Cow::Borrowed("FILE_STRINGS")),
            get_string_fn_name: Some(Cow::Borrowed("string")),
            file_bytes_const_name: None,
            get_bytes_fn_name: None,
        }
    }
}

impl Default for FilesOptions {
    /// # Examples
    /// ```
    /// # use edres_core::options::*;
    /// assert_eq!(FilesOptions::default(), FilesOptions::new());
    /// ```
    fn default() -> Self {
        FilesOptions::new()
    }
}

/// Options specific to how `edres` should handle its output.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct OutputOptions {
    /// If true, missing destination directories will be created
    /// on output.
    pub create_dirs: bool,

    /// If true, files will only be written if they have changed.
    ///
    /// Generation will still take place. This is not an
    /// optimization, but it can be used to avoid unintentionally
    /// triggering any processes that watch for changes. (For
    /// example, `cargo watch`.)
    pub write_only_if_changed: bool,
}

impl OutputOptions {
    /// # Examples
    /// ```
    /// # use edres_core::options::*;
    /// assert_eq!(OutputOptions::new(), OutputOptions {
    ///     create_dirs: true,
    ///     write_only_if_changed: true,
    /// });
    /// ```
    pub const fn new() -> Self {
        OutputOptions {
            create_dirs: true,
            write_only_if_changed: true,
        }
    }
}

impl Default for OutputOptions {
    /// # Examples
    /// ```
    /// # use edres_core::options::*;
    /// assert_eq!(OutputOptions::default(), OutputOptions::new());
    /// ```
    fn default() -> Self {
        Self::new()
    }
}

/// Options for serde support.
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum SerdeSupport {
    /// Do not derive any serde traits for the struct.
    No,

    /// Derive `Serialize` and `Deserialize` for the struct.
    Yes,

    /// Derive any combination of `Serialize` and `Deserialize`
    /// for the struct.
    Mixed { serialize: bool, deserialize: bool },
}

impl SerdeSupport {
    pub(crate) fn should_derive_ser_de(self) -> Option<(bool, bool)> {
        match self {
            Self::No => None,
            Self::Yes => Some((true, true)),
            Self::Mixed {
                serialize,
                deserialize,
            } => {
                if !(serialize || deserialize) {
                    None
                } else {
                    Some((serialize, deserialize))
                }
            }
        }
    }
}

impl Default for SerdeSupport {
    /// # Examples
    /// ```
    /// # use edres_core::options::*;
    /// assert_eq!(SerdeSupport::default(), SerdeSupport::No);
    /// ```
    fn default() -> Self {
        Self::No
    }
}

/// Used to specify the default size of floating point values
/// (providing they fit within the given size).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FloatSize {
    F32,
    F64,
}

/// Used to specify the default size of integer values
/// (providing they fit within the given size).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum IntSize {
    I8,
    I16,
    I32,
    I64,
    I128,
    ISize,
}
