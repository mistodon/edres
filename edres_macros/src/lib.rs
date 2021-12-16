use proc_macro::TokenStream;
use quote::quote;

#[proc_macro]
pub fn define_structs(_stream: TokenStream) -> TokenStream {
    quote!().into()
}

#[proc_macro]
pub fn define_enums(_stream: TokenStream) -> TokenStream {
    quote!().into()
}

#[proc_macro]
pub fn define_enum_from_dir(_stream: TokenStream) -> TokenStream {
    quote!().into()
}

#[proc_macro]
pub fn define_structs_from_dir(_stream: TokenStream) -> TokenStream {
    quote!().into()
}
