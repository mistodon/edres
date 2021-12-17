use std::borrow::Cow;

use crate::{error::OptionsError, format::Format, validation};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct WipOptions {
    pub default_float_size: FloatSize,
    pub default_int_size: IntSize,
    pub max_array_size: usize,
    pub derived_traits: Cow<'static, [Cow<'static, str>]>,
    pub serde_support: SerdeSupport,
    pub impl_default: bool,
    pub impl_display: bool,
    pub impl_from_str: bool,
    pub source_path_const_name: Option<Cow<'static, str>>,
    pub struct_data_const_name: Option<Cow<'static, str>>,
    pub all_variants_const_name: Option<Cow<'static, str>>,
    pub all_values_const_name: Option<Cow<'static, str>>,
    pub values_struct_name: Option<Cow<'static, str>>,
    pub get_value_fn_name: Option<Cow<'static, str>>,
    pub values_struct_options: Option<Box<WipOptions>>, // TODO: No boxes, better defaults!
    pub file_paths_const_name: Option<Cow<'static, str>>,
    pub get_path_fn_name: Option<Cow<'static, str>>,
    pub file_strings_const_name: Option<Cow<'static, str>>,
    pub get_string_fn_name: Option<Cow<'static, str>>,
    pub file_bytes_const_name: Option<Cow<'static, str>>,
    pub get_bytes_fn_name: Option<Cow<'static, str>>,
}

impl WipOptions {
    pub const fn new() -> Self {
        WipOptions {
            default_float_size: FloatSize::F64,
            default_int_size: IntSize::I64,
            max_array_size: 0,
            derived_traits: Cow::Borrowed(&[Cow::Borrowed("Debug")]),
            serde_support: SerdeSupport::No,
            impl_default: false,
            impl_display: false,
            impl_from_str: false,
            source_path_const_name: Some(Cow::Borrowed("SOURCE_PATH")),
            struct_data_const_name: Some(Cow::Borrowed("DATA")),
            all_variants_const_name: Some(Cow::Borrowed("ALL")),
            all_values_const_name: Some(Cow::Borrowed("VALUES")),
            values_struct_name: None,
            get_value_fn_name: Some(Cow::Borrowed("get")),
            values_struct_options: None,
            file_paths_const_name: Some(Cow::Borrowed("FILE_PATHS")),
            get_path_fn_name: Some(Cow::Borrowed("path")),
            file_strings_const_name: None,
            get_string_fn_name: None,
            file_bytes_const_name: None,
            get_bytes_fn_name: None,
        }
    }

    pub const fn minimal() -> Self {
        WipOptions {
            default_float_size: FloatSize::F64,
            default_int_size: IntSize::I64,
            max_array_size: 0,
            derived_traits: Cow::Borrowed(&[]),
            serde_support: SerdeSupport::No,
            impl_default: false,
            impl_display: false,
            impl_from_str: false,
            source_path_const_name: None,
            struct_data_const_name: None,
            all_variants_const_name: None,
            all_values_const_name: None,
            values_struct_name: None,
            get_value_fn_name: None,
            values_struct_options: None,
            file_paths_const_name: None,
            get_path_fn_name: None,
            file_strings_const_name: None,
            get_string_fn_name: None,
            file_bytes_const_name: None,
            get_bytes_fn_name: None,
        }
    }

    pub const fn file_bytes() -> Self {
        WipOptions {
            default_float_size: FloatSize::F64,
            default_int_size: IntSize::I64,
            max_array_size: 0,
            derived_traits: Cow::Borrowed(&[Cow::Borrowed("Debug")]),
            serde_support: SerdeSupport::No,
            impl_default: false,
            impl_display: false,
            impl_from_str: false,
            source_path_const_name: Some(Cow::Borrowed("SOURCE_PATH")),
            struct_data_const_name: Some(Cow::Borrowed("DATA")),
            all_variants_const_name: Some(Cow::Borrowed("ALL")),
            all_values_const_name: Some(Cow::Borrowed("VALUES")),
            values_struct_name: None,
            get_value_fn_name: Some(Cow::Borrowed("get")),
            values_struct_options: None,
            file_paths_const_name: Some(Cow::Borrowed("FILE_PATHS")),
            get_path_fn_name: Some(Cow::Borrowed("path")),
            file_bytes_const_name: Some(Cow::Borrowed("FILE_BYTES")),
            get_bytes_fn_name: Some(Cow::Borrowed("bytes")),
            file_strings_const_name: None,
            get_string_fn_name: None,
        }
    }

    pub const fn file_strings() -> Self {
        WipOptions {
            default_float_size: FloatSize::F64,
            default_int_size: IntSize::I64,
            max_array_size: 0,
            derived_traits: Cow::Borrowed(&[Cow::Borrowed("Debug")]),
            serde_support: SerdeSupport::No,
            impl_default: false,
            impl_display: false,
            impl_from_str: false,
            source_path_const_name: Some(Cow::Borrowed("SOURCE_PATH")),
            struct_data_const_name: Some(Cow::Borrowed("DATA")),
            all_variants_const_name: Some(Cow::Borrowed("ALL")),
            all_values_const_name: Some(Cow::Borrowed("VALUES")),
            values_struct_name: None,
            get_value_fn_name: Some(Cow::Borrowed("get")),
            values_struct_options: None,
            file_paths_const_name: Some(Cow::Borrowed("FILE_PATHS")),
            get_path_fn_name: Some(Cow::Borrowed("path")),
            file_strings_const_name: Some(Cow::Borrowed("FILE_STRINGS")),
            get_string_fn_name: Some(Cow::Borrowed("string")),
            file_bytes_const_name: None,
            get_bytes_fn_name: None,
        }
    }
}

impl Default for WipOptions {
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

/// When to perform dynamic loading from the config file itself.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DynamicLoading {
    /// Always load the config from file.
    Always,

    /// Load from file in debug mode, but use the statically-included
    /// const in release mode.
    DebugOnly,

    /// Never load dynamically. Always use the statically-included
    /// const.
    Never,
}

impl Default for DynamicLoading {
    fn default() -> Self {
        Self::DebugOnly
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

/// Options for configuring the generation of a struct.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct StructOptions {
    /// The format of the source data.
    ///
    /// Defaults to `None` which will cause it to be inferred from the
    /// file type.
    pub format: Option<Format>,

    /// The name of the resulting struct.
    ///
    /// Defaults to `"Config"`.
    pub struct_name: String,

    /// The name of the resulting const, if generated.
    ///
    /// Defaults to the value of `struct_name` in uppercase.
    pub const_name: Option<String>,

    /// Whether or not to generate a `const` instance of the struct.
    ///
    /// Defaults to `true`.
    pub generate_const: bool,

    /// A list of traits for the struct to derive.
    ///
    /// Defaults to `["Debug", "Clone"]`
    ///
    /// (Note that the `serde_support` option below may add to this
    /// list.)
    pub derived_traits: Vec<String>,

    /// Shorthand for generating the Serialize and Deserialize traits.
    ///
    /// Defaults to `No`.
    pub serde_support: SerdeSupport,

    /// The recommended way to derive Serialize and Deserialize
    /// is via the `serde` crate's
    /// [`derive` feature](https://serde.rs/derive.html).
    ///
    /// If you instead need to use the old method of including the
    /// `serde_derive` crate, set this flag to `true`.
    pub use_serde_derive_crate: bool,

    /// Whether or not to generate helper functions to load the
    /// struct at runtime.
    ///
    /// Defaults to `true`.
    ///
    /// **Note:** These load functions depend on the `Deserialize`
    /// trait, as well as the relevant serde library for the config
    /// format.
    ///
    /// So for example, if you generate a struct from `config.json`
    /// then you will have to enable `serde_support` for the
    /// `Deserialize` trait, and you will also have to include the
    /// `serde_json` library in your crate.
    pub generate_load_fns: bool,

    /// Whether the load functions, if generated, are dynamic,
    /// and when.
    ///
    /// Defaults to `DebugOnly`.
    pub dynamic_loading: DynamicLoading,

    /// Whether or not to create the parent directories of the
    /// output file, if they don't exist.
    ///
    /// Defaults to `true`.
    pub create_dirs: bool,

    /// Whether to check if the destination file would be changed
    /// before writing output.
    ///
    /// This is to avoid unnecessary writes from marking the
    /// destination file as changed (which could, for example,
    /// trigger a process which is watching for changes). This
    /// option only works with the `create_*` functions.
    ///
    /// Defaults to `true`.
    pub write_only_if_changed: bool,

    /// The type of floating point values in the config, where the
    /// format does not make it explicit.
    ///
    /// Defaults to `F64`.
    pub default_float_size: FloatSize,

    /// The type of integer values in the config, where the
    /// format does not make it explicit.
    ///
    /// Defaults to `I64`.
    pub default_int_size: IntSize,

    /// The maximum array size, over which array values in the
    /// config will be represented as slices instead.
    ///
    /// If set to `0`, slices will always be used.
    ///
    /// Defaults to `0`.
    pub max_array_size: usize,
}

impl StructOptions {
    pub(crate) fn validate(&self) -> Result<(), OptionsError> {
        if !validation::valid_identifier(&self.struct_name) {
            return Err(OptionsError::InvalidStructName(self.struct_name.clone()));
        }

        Ok(())
    }

    pub(crate) fn real_const_name(&self) -> String {
        self.const_name
            .clone()
            .unwrap_or_else(|| self.struct_name.to_uppercase())
    }

    /// The default options plus serde support. This includes
    /// `Serialize`/`Deserialize` traits, plus helpers functions
    /// to load the config.
    ///
    /// ```rust
    /// # use edres_core as edres;
    /// use edres::{StructOptions, SerdeSupport};
    ///
    /// let options = StructOptions::serde_default();
    ///
    /// assert_eq!(options, StructOptions {
    ///     serde_support: SerdeSupport::Yes,
    ///     generate_load_fns: true,
    ///     .. StructOptions::default()
    /// });
    /// ```
    pub fn serde_default() -> Self {
        StructOptions {
            serde_support: SerdeSupport::Yes,
            generate_load_fns: true,
            ..Self::default()
        }
    }
}

impl Default for StructOptions {
    /// ```rust
    /// # use edres_core as edres;
    /// use edres::*;
    ///
    /// let default_options = StructOptions {
    ///     format: None,
    ///     struct_name: "Config".to_owned(),
    ///     const_name: None,
    ///     generate_const: true,
    ///     derived_traits: vec![
    ///         "Debug".to_owned(),
    ///         "Clone".to_owned(),
    ///     ],
    ///     serde_support: SerdeSupport::No,
    ///     use_serde_derive_crate: false,
    ///     generate_load_fns: false,
    ///     dynamic_loading: DynamicLoading::DebugOnly,
    ///     create_dirs: true,
    ///     write_only_if_changed: true,
    ///     default_float_size: FloatSize::F64,
    ///     default_int_size: IntSize::I64,
    ///     max_array_size: 0,
    /// };
    /// assert_eq!(default_options, StructOptions::default());
    /// ```
    fn default() -> Self {
        StructOptions {
            format: None,
            struct_name: "Config".to_owned(),
            const_name: None,
            generate_const: true,
            derived_traits: vec!["Debug".to_owned(), "Clone".to_owned()],
            serde_support: SerdeSupport::default(),
            use_serde_derive_crate: false,
            generate_load_fns: false,
            dynamic_loading: DynamicLoading::DebugOnly,
            create_dirs: true,
            write_only_if_changed: true,
            default_float_size: FloatSize::F64,
            default_int_size: IntSize::I64,
            max_array_size: 0,
        }
    }
}

/// Options for configuring the generation of a struct.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct EnumOptions {
    /// The format of the source data.
    ///
    /// Defaults to `None` which will cause it to be inferred from the
    /// file type.
    pub format: Option<Format>,

    /// The name of the resulting enum.
    ///
    /// Defaults to `"Key"`.
    pub enum_name: String,

    /// The name of the const slice containing all variants.
    /// For example, if you specify `Some("ALL")`, then
    /// `MyEnum::ALL` will contain all variants of the enum.
    ///
    /// If you specify `None` then no constant will be
    /// generated.
    ///
    /// Defaults to `Some("ALL")`.
    pub all_variants_const: Option<String>,

    /// A list of traits for the struct to derive.
    ///
    /// Defaults to `["Debug", "Clone", "Copy", "PartialEq",
    /// "Eq", "PartialOrd", "Ord", "Hash"]`
    ///
    /// (Note that the `serde_support` option below may add
    /// to this list.)
    pub derived_traits: Vec<String>,

    /// Whether to implement the `Default` trait for this enum.
    /// If `true` then the default value will be the first
    /// variant specified.
    ///
    /// Defaults to `true`.
    pub first_variant_is_default: bool,

    /// Whether to implement the `Display` trait for this enum.
    /// This requires the `Debug` trait to be implemented.
    ///
    /// Defaults to `true`.
    pub impl_display: bool,

    /// Whether to implement the `FromStr` trait for this enum.
    /// This requires the `all_variants_const` to be set to
    /// something other than `None`.
    ///
    /// Defaults to `true`.
    pub impl_from_str: bool,

    /// Shorthand for generating the Serialize and Deserialize
    /// traits.
    ///
    /// Defaults to `No`.
    pub serde_support: SerdeSupport,

    /// The recommended way to derive Serialize and Deserialize
    /// is via the `serde` crate's
    /// [`derive` feature](https://serde.rs/derive.html).
    ///
    /// If you instead need to use the old method of including
    /// the `serde_derive` crate, set this flag to `true`.
    pub use_serde_derive_crate: bool,

    /// Whether or not to create the parent directories of the
    /// output file, if they don't exist.
    ///
    /// Defaults to `true`.
    pub create_dirs: bool,

    /// Whether to check if the destination file would be changed
    /// before writing output.
    ///
    /// This is to avoid unnecessary writes from marking the
    /// destination file as changed (which could, for example,
    /// trigger a process which is watching for changes). This
    /// option only works with the `create_*` functions.
    ///
    /// Defaults to `true`.
    pub write_only_if_changed: bool,
}

impl EnumOptions {
    #[allow(clippy::unnecessary_wraps)]
    pub(crate) fn validate(&self) -> Result<(), OptionsError> {
        eprintln!("TODO: EnumOptions::validate");
        Ok(())
    }

    /// The default options plus serde support. This includes
    /// `Serialize`/`Deserialize` traits, plus helpers functions
    /// to load the config.
    ///
    /// ```rust
    /// # use edres_core as edres;
    /// use edres::{EnumOptions, SerdeSupport};
    ///
    /// let options = EnumOptions::serde_default();
    ///
    /// assert_eq!(options, EnumOptions {
    ///     serde_support: SerdeSupport::Yes,
    ///     .. EnumOptions::default()
    /// });
    /// ```
    pub fn serde_default() -> Self {
        EnumOptions {
            serde_support: SerdeSupport::Yes,
            ..Self::default()
        }
    }
}

/// Defaults to `["Debug", "Clone", "Copy", "PartialEq",
/// "Eq", "PartialOrd", "Ord", "Hash"]`
impl Default for EnumOptions {
    /// ```rust
    /// # use edres_core as edres;
    /// use edres::*;
    ///
    /// let default_options = EnumOptions {
    ///     format: None,
    ///     enum_name: "Key".to_owned(),
    ///     all_variants_const: Some("ALL".to_owned()),
    ///     derived_traits: vec![
    ///         "Debug".to_owned(),
    ///         "Clone".to_owned(),
    ///         "Copy".to_owned(),
    ///         "PartialEq".to_owned(),
    ///         "Eq".to_owned(),
    ///         "PartialOrd".to_owned(),
    ///         "Ord".to_owned(),
    ///         "Hash".to_owned(),
    ///     ],
    ///     first_variant_is_default: true,
    ///     impl_display: true,
    ///     impl_from_str: true,
    ///     serde_support: SerdeSupport::No,
    ///     use_serde_derive_crate: false,
    ///     create_dirs: true,
    ///     write_only_if_changed: true,
    /// };
    /// assert_eq!(default_options, EnumOptions::default());
    /// ```
    fn default() -> Self {
        EnumOptions {
            format: None,
            enum_name: "Key".to_owned(),
            all_variants_const: Some("ALL".to_owned()),
            derived_traits: vec![
                "Debug".to_owned(),
                "Clone".to_owned(),
                "Copy".to_owned(),
                "PartialEq".to_owned(),
                "Eq".to_owned(),
                "PartialOrd".to_owned(),
                "Ord".to_owned(),
                "Hash".to_owned(),
            ],
            first_variant_is_default: true,
            impl_display: true,
            impl_from_str: true,
            serde_support: SerdeSupport::default(),
            use_serde_derive_crate: false,
            create_dirs: true,
            write_only_if_changed: true,
        }
    }
}
