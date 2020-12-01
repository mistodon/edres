use crate::{error::OptionsError, format::Format, validation};

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
    pub(crate) fn validate(&self) -> Result<(), OptionsError> {
        eprintln!("TODO: EnumOptions::validate");
        Ok(())
    }

    /// The default options plus serde support. This includes
    /// `Serialize`/`Deserialize` traits, plus helpers functions
    /// to load the config.
    ///
    /// ```rust
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
