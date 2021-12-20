use std::borrow::Cow;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Options {
    pub source_path_const_name: Option<Cow<'static, str>>,
    pub serde_support: SerdeSupport,

    pub parse: ParseOptions,
    pub structs: StructOptions,
    pub enums: EnumOptions,
    pub files: FilesOptions,
    pub loading: LoadOptions,
    pub output: OutputOptions,
}

impl Options {
    pub const fn new() -> Options {
        Options {
            source_path_const_name: Some(Cow::Borrowed("SOURCE_PATH")),
            serde_support: SerdeSupport::No,

            parse: ParseOptions::new(),
            structs: StructOptions::new(),
            enums: EnumOptions::new(),
            files: FilesOptions::new(),
            loading: LoadOptions::new(),
            output: OutputOptions::new(),
        }
    }

    pub const fn serde_default() -> Options {
        Options {
            source_path_const_name: Some(Cow::Borrowed("SOURCE_PATH")),
            serde_support: SerdeSupport::Yes,

            parse: ParseOptions::new(),
            structs: StructOptions::new(),
            enums: EnumOptions::new(),
            files: FilesOptions::new(),
            loading: LoadOptions::new(),
            output: OutputOptions::new(),
        }
    }

    pub const fn with_load() -> Options {
        Options {
            source_path_const_name: Some(Cow::Borrowed("SOURCE_PATH")),
            serde_support: SerdeSupport::Yes,

            parse: ParseOptions::new(),
            structs: StructOptions::new(),
            enums: EnumOptions::new(),
            files: FilesOptions::new(),
            loading: LoadOptions::full(),
            output: OutputOptions::new(),
        }
    }

    pub const fn minimal() -> Options {
        Options {
            source_path_const_name: None,
            serde_support: SerdeSupport::No,

            parse: ParseOptions::new(),
            structs: StructOptions::minimal(),
            enums: EnumOptions::minimal(),
            files: FilesOptions::minimal(),
            loading: LoadOptions::new(),
            output: OutputOptions::new(),
        }
    }
}

impl Default for Options {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ParseOptions {
    pub default_float_size: FloatSize,
    pub default_int_size: IntSize,
    pub max_array_size: Option<usize>,
}

impl ParseOptions {
    pub const fn new() -> Self {
        ParseOptions {
            default_float_size: FloatSize::F64,
            default_int_size: IntSize::I64,
            max_array_size: None,
        }
    }
}

impl Default for ParseOptions {
    fn default() -> Self {
        ParseOptions::new()
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct StructOptions {
    pub derived_traits: Cow<'static, [Cow<'static, str>]>,
    pub struct_data_const_name: Option<Cow<'static, str>>,
}

impl StructOptions {
    pub const fn new() -> StructOptions {
        StructOptions {
            derived_traits: Cow::Borrowed(&[Cow::Borrowed("Debug")]),
            struct_data_const_name: Some(Cow::Borrowed("DATA")),
        }
    }

    pub const fn minimal() -> StructOptions {
        StructOptions {
            derived_traits: Cow::Borrowed(&[]),
            struct_data_const_name: None,
        }
    }
}

impl Default for StructOptions {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct EnumOptions {
    pub derived_traits: Cow<'static, [Cow<'static, str>]>,
    pub impl_default: bool,
    pub impl_display: bool,
    pub impl_from_str: bool,
    pub all_variants_const_name: Option<Cow<'static, str>>,
    pub all_values_const_name: Option<Cow<'static, str>>,
    pub values_struct_name: Option<Cow<'static, str>>,
    pub values_struct_options: StructOptions,
    pub get_value_fn_name: Option<Cow<'static, str>>,
}

impl EnumOptions {
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
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FilesOptions {
    pub file_paths_const_name: Option<Cow<'static, str>>,
    pub get_path_fn_name: Option<Cow<'static, str>>,
    pub file_strings_const_name: Option<Cow<'static, str>>,
    pub get_string_fn_name: Option<Cow<'static, str>>,
    pub file_bytes_const_name: Option<Cow<'static, str>>,
    pub get_bytes_fn_name: Option<Cow<'static, str>>,
}

impl FilesOptions {
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
    fn default() -> Self {
        FilesOptions::new()
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LoadOptions {
    pub load_fn_name: Option<Cow<'static, str>>,
    pub load_from_file_fn_name: Option<Cow<'static, str>>,
    pub fetch_fn_name: Option<Cow<'static, str>>,
}

impl LoadOptions {
    pub const fn new() -> Self {
        LoadOptions {
            load_fn_name: None,
            load_from_file_fn_name: None,
            fetch_fn_name: None,
        }
    }

    pub const fn full() -> Self {
        LoadOptions {
            load_fn_name: Some(Cow::Borrowed("load")),
            load_from_file_fn_name: Some(Cow::Borrowed("load_from_file")),
            fetch_fn_name: Some(Cow::Borrowed("fetch")),
        }
    }
}

impl Default for LoadOptions {
    fn default() -> Self {
        LoadOptions::new()
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct OutputOptions {
    pub create_dirs: bool,
    pub write_only_if_changed: bool,
}

impl OutputOptions {
    pub const fn new() -> Self {
        OutputOptions {
            create_dirs: true,
            write_only_if_changed: true,
        }
    }
}

impl Default for OutputOptions {
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
    fn default() -> Self {
        Self::No
    }
}

/// Represents a floating-point type.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FloatSize {
    F32,
    F64,
}

/// Represents an integer type.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum IntSize {
    I8,
    I16,
    I32,
    I64,
    I128,
    ISize,
}
