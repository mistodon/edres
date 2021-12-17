use std::borrow::Cow;

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
