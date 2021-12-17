use proc_macro::TokenStream;
use quote::quote;

/// # Example
/// ```rust
/// # use edres_macros::*;
/// define_structs! {
///     #[path = "assets/config.toml"]
///     struct Config;
///
///     #[path = "assets/settings.yaml"]
///     struct Settings;
/// }
/// ```
#[proc_macro]
pub fn define_structs(_stream: TokenStream) -> TokenStream {
    quote!().into()
}

/// # Example
/// ```rust
/// # use edres_macros::*;
/// define_enums! {
///     #[path = "assets/map1.yaml"]
///     enum KeyOnly;
///
///     #[path = "assets/map2.yaml"]
///     enum Key -> struct; // Struct name is generated
///
///     #[path = "assets/map3.yaml"]
///     enum Key2 -> struct Value;
/// }
/// ```
#[proc_macro]
pub fn define_enums(_stream: TokenStream) -> TokenStream {
    quote!().into()
}

/// # Example
/// ```rust
/// # use edres_macros::*;
/// define_enums_from_dirs! {
///     #[path = "assets/dir1"]
///     enum FilenameOnly;
///
///     #[path = "assets/dir2"]
///     enum Filename -> struct; // Struct name is generated
///
///     #[path = "assets/dir3"]
///     enum Filename2 -> struct FileContents;
/// }
/// ```
#[proc_macro]
pub fn define_enums_from_dirs(_stream: TokenStream) -> TokenStream {
    quote!().into()
}

/// # Example
/// ```rust
/// # use edres_macros::*;
/// define_structs_from_dirs! {
///     #[path = "assets/items"]
///     struct ItemInfo;
/// }
/// ```
#[proc_macro]
pub fn define_structs_from_dirs(_stream: TokenStream) -> TokenStream {
    quote!().into()
}
