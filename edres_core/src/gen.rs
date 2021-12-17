use std::path::Path;

use proc_macro2::TokenStream;
use quote::{format_ident, quote};

use crate::error::WipError;
use crate::options::{SerdeSupport, WipOptions};
use crate::parsing;
use crate::value::{Struct, Value};
use crate::Format;

pub fn define_structs(
    data: &Struct,
    struct_name: &str,
    source_file_path: Option<&Path>,
    options: &WipOptions,
) -> Result<TokenStream, WipError> {
    let derives = derive_attribute(
        options.derived_traits.as_ref(),
        options.serde_support,
        false,
    );

    let struct_tokens = define_structs_inner(data, struct_name, options, derives.as_ref())?;

    let mut inherents = vec![];
    if let (Some(source_file_path), Some(const_name)) =
        (source_file_path, options.source_path_const_name.as_ref())
    {
        let source_file_path = source_file_path.display().to_string();
        let source_path_const_name = format_ident!("{}", const_name);
        inherents.push(quote! {
            pub const #source_path_const_name: &'static str = #source_file_path;
        });
    }
    if let Some(const_name) = &options.struct_data_const_name {
        let struct_value = define_struct_value(data, options, struct_name)?;
        let struct_name = format_ident!("{}", struct_name);
        let const_name = format_ident!("{}", const_name);

        inherents.push(quote! {
            pub const #const_name: #struct_name = #struct_value;
        });
    }

    let struct_name = format_ident!("{}", struct_name);
    let inherent_tokens = (!inherents.is_empty())
        .then(|| {
            quote! {
                impl #struct_name {
                    #(#inherents)*
                }
            }
        })
        .into_iter();

    Ok(quote! {
        #struct_tokens
        #(#inherent_tokens)*
    })
}

fn define_structs_inner(
    data: &Struct,
    struct_name: &str,
    options: &WipOptions,
    derives: Option<&TokenStream>,
) -> Result<TokenStream, WipError> {
    let mut fields = vec![];
    let mut sub_structs = vec![];

    for (key, value) in data.0.iter() {
        let field_name = format_ident!("{}", key);
        let decl = type_of_value(
            value,
            options,
            struct_name,
            Some(key),
            None,
            &mut sub_structs,
        )?;
        fields.push(quote!(pub #field_name : #decl));
    }

    let sub_structs: Vec<TokenStream> = sub_structs
        .iter()
        .map(|(name, value)| define_structs_inner(value, name, options, derives))
        .collect::<Result<_, WipError>>()?;

    let struct_name = format_ident!("{}", struct_name);
    let derives = derives.into_iter();

    let tokens = quote!(
        #[allow(non_camel_case_types)]
        #(#derives)*
        pub struct #struct_name {
            #(#fields ,)*
        }

        #(#sub_structs)*
    );

    Ok(tokens)
}

fn define_structs_for_value(
    data: &Value,
    root_struct_name: &str,
    options: &WipOptions,
    dest: &mut Vec<TokenStream>,
) -> Result<(), WipError> {
    let derives = derive_attribute(
        options.derived_traits.as_ref(),
        options.serde_support,
        false,
    );

    match data {
        Value::Option(Some(value)) => {
            define_structs_for_value(value, root_struct_name, options, dest)
        }
        Value::Option(None) => Ok(()),
        Value::Tuple(values) => {
            for (i, value) in values.iter().enumerate() {
                let struct_name = format!("{}__{}", root_struct_name, i);
                define_structs_for_value(value, &struct_name, options, dest)?;
            }
            Ok(())
        }
        Value::Array(_size, values) => match values.first() {
            Some(value) => define_structs_for_value(value, root_struct_name, options, dest),
            None => Ok(()),
        },
        Value::Vec(values) => match values.first() {
            Some(value) => define_structs_for_value(value, root_struct_name, options, dest),
            None => Ok(()),
        },
        Value::Struct(fields) => {
            dest.push(define_structs_inner(
                fields,
                root_struct_name,
                options,
                derives.as_ref(),
            )?);
            Ok(())
        }
        _ => Ok(()),
    }
}

fn define_enum_from_variants_and_values<'a, IK, IV, S>(
    variants: IK,
    values: IV,
    use_values: bool,
    enum_name: &str,
    source_file_path: Option<&Path>,
    options: &WipOptions,
    mut inherents: Vec<TokenStream>,
) -> Result<TokenStream, WipError>
where
    IK: IntoIterator<Item = S>,
    IK::IntoIter: Clone,
    IV: IntoIterator<Item = &'a Value>,
    S: AsRef<str>,
{
    let derives = derive_attribute(
        options.derived_traits.as_ref(),
        options.serde_support,
        options.impl_display,
    )
    .into_iter();
    let enum_name = format_ident!("{}", enum_name);
    let variants = variants.into_iter();
    let enum_variants = variants.clone().map(|s| format_ident!("{}", s.as_ref()));

    // Inherent impl block
    if let (Some(source_file_path), Some(const_name)) =
        (source_file_path, options.source_path_const_name.as_ref())
    {
        let source_file_path = source_file_path.display().to_string();
        let source_path_const_name = format_ident!("{}", const_name);
        inherents.push(quote! {
            pub const #source_path_const_name: &'static str = #source_file_path;
        });
    }
    if let Some(const_name) = &options.all_variants_const_name {
        let const_name = format_ident!("{}", const_name);
        let enum_variants = variants.clone().map(|s| format_ident!("{}", s.as_ref()));
        inherents.push(quote! {
            pub const #const_name: &'static [Self] = &[
                #(Self::#enum_variants,)*
            ];
        });
    }
    let new_struct_tokens = match (use_values, &options.all_values_const_name) {
        (true, Some(const_name)) => {
            let const_name = format_ident!("{}", const_name);

            let struct_name = options
                .values_struct_name
                .as_ref()
                .map(|s| s.to_string())
                .unwrap_or_else(|| format!("{}__Value", enum_name));
            let value_options = options
                .values_struct_options
                .as_ref()
                .cloned()
                .unwrap_or_default();
            let (value_type, values, new_struct_tokens) =
                establish_types_for_values(values.into_iter(), &struct_name, &value_options)?;
            let values = values
                .iter()
                .map(|value| define_value(value, &value_options, &struct_name, None, None))
                .collect::<Result<Vec<_>, _>>()?;

            inherents.push(quote! {
                pub const #const_name: &'static [#value_type] = &[
                    #(#values,)*
                ];
            });

            if let Some(get_value_fn_name) = &options.get_value_fn_name {
                let get_value_fn_name = format_ident!("{}", get_value_fn_name);
                inherents.push(quote! {
                    pub const fn #get_value_fn_name(&self) -> &'static #value_type {
                        &Self::#const_name[*self as usize]
                    }
                });
            }

            new_struct_tokens
        }
        _ => vec![],
    };
    let new_struct_tokens = new_struct_tokens.into_iter();

    let inherent_tokens = (!inherents.is_empty())
        .then(|| {
            quote! {
                impl #enum_name {
                    #(#inherents)*
                }
            }
        })
        .into_iter();

    // Manually implemented traits
    let default_tokens = options
        .impl_default
        .then(|| {
            let first_variant = format_ident!("{}", variants.clone().next().unwrap().as_ref());
            quote! {
                impl Default for #enum_name {
                    fn default() -> Self {
                        Self::#first_variant
                    }
                }
            }
        })
        .into_iter();

    let display_tokens = options
        .impl_display
        .then(|| {
            quote! {
                impl std::fmt::Display for #enum_name {
                    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
                        <Self as std::fmt::Debug>::fmt(self, f)
                    }
                }
            }
        })
        .into_iter();

    let from_str_tokens = options
        .impl_from_str
        .then(|| {
            let enum_variants = variants.clone().map(|s| format_ident!("{}", s.as_ref()));
            let enum_strings = variants.map(|s| s.as_ref().to_string());

            quote! {
                impl std::str::FromStr for #enum_name {
                    type Err = ();

                    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
                        Ok(match s {
                            #(#enum_strings => Self::#enum_variants,)*
                            _ => return Err(())
                        })
                    }
                }
            }
        })
        .into_iter();

    let tokens = quote! {
        #(#derives)*
        pub enum #enum_name {
            #(#enum_variants,)*
        }
        #(#inherent_tokens)*
        #(#default_tokens)*
        #(#display_tokens)*
        #(#from_str_tokens)*
        #(#new_struct_tokens)*
    };
    Ok(tokens)
}

pub fn define_enum_from_keys(
    data: &Struct,
    enum_name: &str,
    source_file_path: Option<&Path>,
    options: &WipOptions,
) -> Result<TokenStream, WipError> {
    define_enum_from_variants_and_values(
        data.0.keys(),
        data.0.values(),
        true,
        enum_name,
        source_file_path,
        options,
        vec![],
    )
}

pub fn define_structs_from_values(
    data: &Struct,
    struct_name: &str,
    options: &WipOptions,
) -> Result<TokenStream, WipError> {
    let (value_type, values, new_struct_tokens) =
        establish_types_for_values(data.0.values(), struct_name, options)?;

    let mut const_tokens = None;
    if let Some(const_name) = &options.all_values_const_name {
        let const_name = format_ident!("{}", const_name);
        let values = values
            .iter()
            .map(|value| define_value(value, options, struct_name, None, None))
            .collect::<Result<Vec<_>, _>>()?;

        const_tokens = Some(quote! {
            pub const #const_name: &'static [#value_type] = &[
                #(#values,)*
            ];
        });
    }
    let const_tokens = const_tokens.into_iter();

    Ok(quote! {
        #(#new_struct_tokens)*
        #(#const_tokens)*
    })
}

// NOTE: This is a weird function.
// It returns:
// 1. the (unified) type of the values
// 2. a list of the actual values as data
// 3. any new structs defined
fn establish_types_for_values<'a, I: IntoIterator<Item = &'a Value>>(
    values: I,
    struct_name: &str,
    options: &WipOptions,
) -> Result<(TokenStream, Vec<Value>, Vec<TokenStream>), WipError> {
    let mut values = values.into_iter().map(Clone::clone).collect::<Vec<_>>();
    if values.is_empty() {
        return Err(WipError("Need values in map".into()));
    }
    parsing::unify_values(&mut values)?;
    let first = &values[0];

    let mut new_structs = vec![];
    define_structs_for_value(first, struct_name, options, &mut new_structs)?;

    let mut unused = vec![];
    let value_type = type_of_value(first, options, struct_name, None, None, &mut unused)?;

    Ok((value_type, values, new_structs))
}

pub fn define_enum_from_filenames(
    root: &Path,
    enum_name: &str,
    options: &WipOptions,
) -> Result<TokenStream, WipError> {
    use case::CaseExt;
    use ignore::WalkBuilder;

    let filepaths: Vec<String> = {
        let walk = WalkBuilder::new(root)
            .max_depth(Some(1))
            .sort_by_file_name(std::ffi::OsStr::cmp)
            .filter_entry(|entry| entry.file_type().map(|ft| ft.is_file()).unwrap_or(false))
            .build();

        walk.into_iter()
            .skip(1)
            .map(|entry| {
                entry
                    .map_err(|e| WipError(e.to_string()))
                    .map(|entry| entry.path().to_string_lossy().into_owned())
            })
            .collect::<Result<Vec<_>, _>>()?
    };

    let filenames: Vec<String> = filepaths
        .iter()
        .map(|path| {
            <String as AsRef<Path>>::as_ref(path)
                .file_stem()
                .map(|name| name.to_string_lossy().into_owned())
                .ok_or_else(|| WipError("No file stem".into()))
                .map(|s| s.to_camel())
        })
        .collect::<Result<Vec<_>, _>>()?;

    let mut extra_inherents = vec![];

    if let Some(const_name) = &options.file_paths_const_name {
        let const_name = format_ident!("{}", const_name);
        let get_fn = options.get_path_fn_name.as_ref().map(|fn_name| {
            let fn_name = format_ident!("{}", fn_name);
            quote! {
                pub const fn #fn_name(&self) -> &'static str { Self::#const_name[*self as usize] }
            }
        }).into_iter();

        let filepaths = filepaths.iter();
        extra_inherents.push(quote! {
            pub const #const_name: &'static [&'static str] = &[
                #(#filepaths,)*
            ];
            #(#get_fn)*
        });
    }
    if let Some(const_name) = &options.file_bytes_const_name {
        let const_name = format_ident!("{}", const_name);
        let get_fn = options.get_bytes_fn_name.as_ref().map(|fn_name| {
            let fn_name = format_ident!("{}", fn_name);
            quote! {
                pub const fn #fn_name(&self) -> &'static [u8] { Self::#const_name[*self as usize] }
            }
        }).into_iter();

        let filepaths = filepaths.iter();
        extra_inherents.push(quote! {
            pub const #const_name: &'static [&'static [u8]] = &[
                #(include_bytes!(#filepaths),)*
            ];
            #(#get_fn)*
        });
    }
    if let Some(const_name) = &options.file_strings_const_name {
        let const_name = format_ident!("{}", const_name);
        let get_fn = options.get_string_fn_name.as_ref().map(|fn_name| {
            let fn_name = format_ident!("{}", fn_name);
            quote! {
                pub const fn #fn_name(&self) -> &'static str { Self::#const_name[*self as usize] }
            }
        }).into_iter();

        let filepaths = filepaths.iter();
        extra_inherents.push(quote! {
            pub const #const_name: &'static [&'static str] = &[
                #(include_str!(#filepaths),)*
            ];
            #(#get_fn)*
        });
    }

    // TODO: Optionally fill
    let mut values = vec![];
    let use_values = !values.is_empty();

    define_enum_from_variants_and_values(
        filenames.into_iter(),
        values,
        use_values,
        enum_name,
        Some(root),
        options,
        extra_inherents,
    )
}

pub fn define_structs_from_file_contents(
    root: &Path,
    struct_name: &str,
    format: Option<Format>,
    options: &WipOptions,
) -> Result<TokenStream, WipError> {
    use ignore::WalkBuilder;

    let values: Vec<Value> = {
        let walk = WalkBuilder::new(root)
            .max_depth(Some(1))
            .sort_by_file_name(std::ffi::OsStr::cmp)
            .filter_entry(|entry| entry.file_type().map(|ft| ft.is_file()).unwrap_or(false))
            .build();

        walk.into_iter()
            .skip(1)
            .map(|entry| {
                entry
                    .map_err(|e| WipError(e.to_string()))
                    .and_then(|entry| {
                        parsing::parse_source_file_with_format(entry.path(), format, options)
                    })
            })
            .collect::<Result<Vec<_>, _>>()?
    };
    let (value_type, values, new_struct_tokens) =
        establish_types_for_values(values.iter(), struct_name, options)?;

    let mut const_tokens = None;
    if let Some(const_name) = &options.all_values_const_name {
        let const_name = format_ident!("{}", const_name);
        let values = values
            .iter()
            .map(|value| define_value(value, &options, &struct_name, None, None))
            .collect::<Result<Vec<_>, _>>()?;

        const_tokens = Some(quote! {
            pub const #const_name: &'static [#value_type] = &[
                #(#values,)*
            ];
        });
    }
    let const_tokens = const_tokens.into_iter();

    Ok(quote! {
        #(#new_struct_tokens)*
        #(#const_tokens)*
    })
}

fn derive_attribute<S: AsRef<str>, I: IntoIterator<Item = S>>(
    trait_list: I,
    serde_support: SerdeSupport,
    require_debug: bool,
) -> Option<TokenStream> {
    fn format_derive(s: &str) -> TokenStream {
        match s.split_once("::") {
            Some((crate_name, trait_name)) => {
                let crate_name = format_ident!("{}", crate_name);
                let trait_name = format_ident!("{}", trait_name);
                quote!(#crate_name :: #trait_name)
            }
            None => {
                let tokens = format_ident!("{}", s);
                quote!(#tokens)
            }
        }
    }

    let mut derives = vec![];
    let mut deriving_debug = false;
    for item in trait_list {
        let item = item.as_ref();
        if item == "Debug" {
            deriving_debug = true;
        }

        derives.push(format_derive(item));
    }

    let (ser, de) = serde_support
        .should_derive_ser_de()
        .unwrap_or((false, false));
    if ser {
        derives.push(format_derive("serde::Serialize"));
    }
    if de {
        derives.push(format_derive("serde::Deserialize"));
    }

    if !deriving_debug && require_debug {
        derives.push(format_derive("Debug"));
    }

    (!derives.is_empty()).then(|| quote!(#[derive(#(#derives),*)]))
}

fn type_of_value<'a>(
    value: &'a Value,
    options: &WipOptions,
    struct_name: &str,
    under_key: Option<&str>,
    under_index: Option<usize>,
    new_structs: &mut Vec<(String, &'a Struct)>,
) -> Result<TokenStream, WipError> {
    Ok(match value {
        Value::Unit => quote!(()),
        Value::Bool(_) => quote!(bool),
        Value::Char(_) => quote!(char),
        Value::I8(_) => quote!(i8),
        Value::I16(_) => quote!(i16),
        Value::I32(_) => quote!(i32),
        Value::I64(_) => quote!(i64),
        Value::I128(_) => quote!(i128),
        Value::ISize(_) => quote!(isize),
        Value::U8(_) => quote!(u8),
        Value::U16(_) => quote!(u16),
        Value::U32(_) => quote!(u32),
        Value::U64(_) => quote!(u64),
        Value::U128(_) => quote!(u128),
        Value::USize(_) => quote!(usize),
        Value::F32(_) => quote!(f32),
        Value::F64(_) => quote!(f64),
        Value::String(_) => quote!(std::borrow::Cow<'static, str>),
        Value::Option(x) => match x {
            Some(value) => {
                type_of_value(value, options, struct_name, under_key, None, new_structs)?
            }
            None => quote!(Option<()>),
        },
        Value::Array(len, values) => {
            assert_eq!(values.len(), *len);
            match values.len() {
                0 => quote!([(); #len]),
                _ => {
                    let inner_type = type_of_value(
                        &values[0],
                        options,
                        struct_name,
                        under_key,
                        None,
                        new_structs,
                    )?;
                    quote!([#inner_type; #len])
                }
            }
        }
        Value::Vec(values) => match values.len() {
            0 => quote!(std::borrow::Cow<'static, [()]>),
            _ => {
                let inner_type = type_of_value(
                    &values[0],
                    options,
                    struct_name,
                    under_key,
                    None,
                    new_structs,
                )?;
                quote!(std::borrow::Cow<'static, [#inner_type]>)
            }
        },
        Value::Tuple(values) => {
            let types_in_tuple = values
                .iter()
                .enumerate()
                .map(|(i, v)| {
                    type_of_value(v, options, struct_name, under_key, Some(i), new_structs)
                })
                .collect::<Result<Vec<_>, WipError>>()?;
            quote!((#(#types_in_tuple),*))
        }
        Value::Struct(mapping) => {
            let key = match under_key {
                None => String::new(),
                Some(k) => format!("__{}", k),
            };
            let index = match under_index {
                None => String::new(),
                Some(i) => format!("__{}", i),
            };
            let name = format!("{}{}{}", struct_name, key, index);
            let ident = format_ident!("{}", name);
            new_structs.push((name, mapping));

            quote!(#ident)
        }
    })
}

fn define_value(
    value: &Value,
    options: &WipOptions,
    struct_name: &str,
    under_key: Option<&str>,
    under_index: Option<usize>,
) -> Result<TokenStream, WipError> {
    Ok(match value {
        Value::Unit => quote!(()),
        Value::Bool(x) => quote!(#x),
        Value::Char(x) => quote!(#x),
        Value::I8(x) => quote!(#x),
        Value::I16(x) => quote!(#x),
        Value::I32(x) => quote!(#x),
        Value::I64(x) => quote!(#x),
        Value::I128(x) => quote!(#x),
        Value::ISize(x) => quote!(#x),
        Value::U8(x) => quote!(#x),
        Value::U16(x) => quote!(#x),
        Value::U32(x) => quote!(#x),
        Value::U64(x) => quote!(#x),
        Value::U128(x) => quote!(#x),
        Value::USize(x) => quote!(#x),
        Value::F32(x) => quote!(#x),
        Value::F64(x) => quote!(#x),
        Value::String(x) => quote!(std::borrow::Cow::Borrowed(#x)),
        Value::Option(x) => match x {
            Some(x) => {
                let value = define_value(x, options, struct_name, under_key, under_index)?;
                quote!(Some(#value))
            }
            None => quote!(None),
        },
        Value::Array(_, values) => {
            let values = values
                .iter()
                .map(|value| define_value(value, options, struct_name, under_key, under_index))
                .collect::<Result<Vec<_>, WipError>>()?;
            quote!([#(#values,)*])
        }
        Value::Vec(values) => {
            let values = values
                .iter()
                .map(|value| define_value(value, options, struct_name, under_key, under_index))
                .collect::<Result<Vec<_>, WipError>>()?;
            quote!(std::borrow::Cow::Borrowed(&[#(#values,)*]))
        }
        Value::Tuple(values) => {
            let values = values
                .iter()
                .enumerate()
                .map(|(i, value)| define_value(value, options, struct_name, under_key, Some(i)))
                .collect::<Result<Vec<_>, WipError>>()?;
            quote!((#(#values),*))
        }
        Value::Struct(fields) => {
            let key = match under_key {
                None => String::new(),
                Some(k) => format!("__{}", k),
            };
            let index = match under_index {
                None => String::new(),
                Some(i) => format!("__{}", i),
            };
            let name = format!("{}{}{}", struct_name, key, index);
            define_struct_value(fields, options, &name)?
        }
    })
}

fn define_struct_value(
    data: &Struct,
    options: &WipOptions,
    struct_name: &str,
) -> Result<TokenStream, WipError> {
    let mut fields = vec![];

    for (key, value) in data.0.iter() {
        let value = define_value(value, options, struct_name, Some(key), None)?;
        let key = format_ident!("{}", key);
        fields.push(quote!(#key: #value,));
    }

    let struct_name = format_ident!("{}", struct_name);
    Ok(quote! {
        #struct_name {
            #(#fields)*
        }
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    fn assert_tokens(a: TokenStream, b: TokenStream) {
        assert_eq!(a.to_string(), b.to_string())
    }

    #[test]
    fn type_declarations() {
        let whatever = &mut vec![];
        let options = &WipOptions::default();

        fn some_struct() -> Value {
            Value::Struct(Struct([("key".into(), Value::Unit)].into_iter().collect()))
        }

        let a_struct = some_struct();
        let a_vec = Value::Vec(vec![some_struct(), some_struct()]);
        let a_tuple = Value::Tuple(vec![some_struct(), some_struct()]);

        assert_tokens(
            type_of_value(&Value::Unit, options, "unused", None, None, whatever).unwrap(),
            quote!(()),
        );
        assert_tokens(
            type_of_value(&Value::F32(1.), options, "unused", None, None, whatever).unwrap(),
            quote!(f32),
        );
        assert_tokens(
            type_of_value(&a_struct, options, "StructName", None, None, whatever).unwrap(),
            quote!(StructName),
        );
        assert_tokens(
            type_of_value(&a_vec, options, "StructName", None, None, whatever).unwrap(),
            quote!(std::borrow::Cow<'static, [StructName]>),
        );
        assert_tokens(
            type_of_value(&a_tuple, options, "StructName", None, None, whatever).unwrap(),
            quote!((StructName__0, StructName__1)),
        );
        assert_tokens(
            type_of_value(
                &Value::Struct(Struct(
                    [(
                        "nest".into(),
                        Value::Tuple(vec![Value::Unit, some_struct()]),
                    )]
                    .into_iter()
                    .collect(),
                )),
                options,
                "StructName",
                None,
                None,
                whatever,
            )
            .unwrap(),
            quote!(StructName),
        );
    }

    #[test]
    #[cfg(rustfmt_skip)]
    fn value_declarations() {
        let options = &WipOptions::default();

        fn some_struct() -> Value {
            Value::Struct(Struct([("key".into(), Value::Unit)].into_iter().collect()))
        };

        let a_struct = some_struct();
        let a_vec = Value::Vec(vec![some_struct(), some_struct()]);
        let a_tuple = Value::Tuple(vec![some_struct(), some_struct()]);

        assert_tokens(
            define_value(&Value::Unit, options, "unused", None, None).unwrap(),
            quote!(()),
        );
        assert_tokens(
            define_value(&Value::F32(1.), options, "unused", None, None).unwrap(),
            quote!(1f32),
        );
        assert_tokens(
            define_value(&a_struct, options, "StructName", None, None).unwrap(),
            quote!(StructName { key: () }),
        );
        assert_tokens(
            define_value(&a_vec, options, "StructName", None, None).unwrap(),
            quote!(std::borrow::Cow::Borrowed(&[
                StructName { key: () },
                StructName { key: () },
            ])),
        );
        assert_tokens(
            define_value(&a_tuple, options, "StructName", None, None).unwrap(),
            quote!((StructName__0 { key: () }, StructName__1 { key: () })),
        );
        assert_tokens(
            define_value(
                &Value::Struct(Struct(
                    [(
                        "nest".into(),
                        Value::Tuple(vec![Value::Unit, some_struct()]),
                    )]
                    .into_iter()
                    .collect(),
                )),
                options,
                "StructName",
                None,
                None,
            )
            .unwrap(),
            quote!(StructName {
                nest: ((), StructName__nest__1 { key: () }),
            }),
        );
    }

    #[test]
    fn empty_struct() {
        let fields = Struct([].into_iter().collect());
        let result = define_structs(&fields, "Struct", None, &WipOptions::minimal()).unwrap();
        assert_tokens(
            result,
            quote!(
                #[allow(non_camel_case_types)]
                pub struct Struct {}
            ),
        );
    }

    #[test]
    fn empty_struct_with_path() {
        let fields = Struct([].into_iter().collect());
        let result = define_structs(
            &fields,
            "Struct",
            Some("./path/to/file.toml".as_ref()),
            &WipOptions {
                source_path_const_name: Some("SOURCE_PATH".into()),
                ..WipOptions::minimal()
            },
        )
        .unwrap();
        assert_tokens(
            result,
            quote!(
                #[allow(non_camel_case_types)]
                pub struct Struct {}

                impl Struct {
                    pub const SOURCE_PATH: &'static str = "./path/to/file.toml";
                }
            ),
        );
    }

    #[test]
    fn big_flat_struct() {
        let fields = Struct(
            [
                ("a".into(), Value::Unit),
                ("b".into(), Value::Bool(true)),
                ("c".into(), Value::Char('a')),
                ("d".into(), Value::I8(8)),
                ("e".into(), Value::I16(16)),
                ("f".into(), Value::I32(32)),
                ("g".into(), Value::I64(64)),
                ("h".into(), Value::I128(128)),
                ("i".into(), Value::ISize(64)),
                ("j".into(), Value::U8(8)),
                ("k".into(), Value::U16(16)),
                ("l".into(), Value::U32(32)),
                ("m".into(), Value::U64(64)),
                ("n".into(), Value::U128(128)),
                ("o".into(), Value::USize(64)),
                ("p".into(), Value::F32(32.)),
                ("q".into(), Value::F64(64.)),
                ("r".into(), Value::String("String".into())),
            ]
            .into_iter()
            .collect(),
        );
        let result = define_structs(&fields, "Struct", None, &WipOptions::minimal()).unwrap();
        assert_tokens(
            result,
            quote!(
                #[allow(non_camel_case_types)]
                pub struct Struct {
                    pub a: (),
                    pub b: bool,
                    pub c: char,
                    pub d: i8,
                    pub e: i16,
                    pub f: i32,
                    pub g: i64,
                    pub h: i128,
                    pub i: isize,
                    pub j: u8,
                    pub k: u16,
                    pub l: u32,
                    pub m: u64,
                    pub n: u128,
                    pub o: usize,
                    pub p: f32,
                    pub q: f64,
                    pub r: std::borrow::Cow<'static, str>,
                }
            ),
        );
    }

    #[test]
    fn struct_with_vecs() {
        let fields = Struct(
            [
                ("a".into(), Value::Vec(vec![])),
                ("b".into(), Value::Vec(vec![Value::Bool(true)])),
                (
                    "c".into(),
                    Value::Vec(vec![Value::Bool(true), Value::Bool(false)]),
                ),
            ]
            .into_iter()
            .collect(),
        );
        let result = define_structs(&fields, "Struct", None, &WipOptions::minimal()).unwrap();
        assert_tokens(
            result,
            quote!(
                #[allow(non_camel_case_types)]
                pub struct Struct {
                    pub a: std::borrow::Cow<'static, [()]>,
                    pub b: std::borrow::Cow<'static, [bool]>,
                    pub c: std::borrow::Cow<'static, [bool]>,
                }
            ),
        );
    }

    #[test]
    fn struct_with_arrays() {
        let fields = Struct(
            [
                ("a".into(), Value::Array(0, vec![])),
                ("b".into(), Value::Array(1, vec![Value::Bool(true)])),
                (
                    "c".into(),
                    Value::Array(2, vec![Value::Bool(true), Value::Bool(false)]),
                ),
            ]
            .into_iter()
            .collect(),
        );
        let result = define_structs(&fields, "Struct", None, &WipOptions::minimal()).unwrap();
        assert_tokens(
            result,
            quote!(
                #[allow(non_camel_case_types)]
                pub struct Struct {
                    pub a: [(); 0usize],
                    pub b: [bool; 1usize],
                    pub c: [bool; 2usize],
                }
            ),
        );
    }

    #[test]
    fn nested_struct() {
        let fields = Struct(
            [(
                "nested".into(),
                Value::Struct(Struct(
                    [("inner".into(), Value::Bool(true))].into_iter().collect(),
                )),
            )]
            .into_iter()
            .collect(),
        );
        let result = define_structs(&fields, "Struct", None, &WipOptions::minimal()).unwrap();
        assert_tokens(
            result,
            quote!(
                #[allow(non_camel_case_types)]
                pub struct Struct {
                    pub nested: Struct__nested,
                }

                #[allow(non_camel_case_types)]
                pub struct Struct__nested {
                    pub inner: bool,
                }
            ),
        );
    }

    #[test]
    fn nested_struct_with_derives() {
        let fields = Struct(
            [(
                "nested".into(),
                Value::Struct(Struct(
                    [("inner".into(), Value::Bool(true))].into_iter().collect(),
                )),
            )]
            .into_iter()
            .collect(),
        );
        let result = define_structs(
            &fields,
            "Struct",
            None,
            &WipOptions {
                derived_traits: vec!["Debug".into(), "Clone".into()].into(),
                serde_support: SerdeSupport::Yes,
                ..WipOptions::minimal()
            },
        )
        .unwrap();
        assert_tokens(
            result,
            quote!(
                #[allow(non_camel_case_types)]
                #[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
                pub struct Struct {
                    pub nested: Struct__nested,
                }

                #[allow(non_camel_case_types)]
                #[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
                pub struct Struct__nested {
                    pub inner: bool,
                }
            ),
        );
    }

    #[test]
    fn more_nested_struct() {
        let fields = Struct(
            [(
                "nested".into(),
                Value::Struct(Struct(
                    [(
                        "nested_again".into(),
                        Value::Struct(Struct(
                            [("inner".into(), Value::Bool(true))].into_iter().collect(),
                        )),
                    )]
                    .into_iter()
                    .collect(),
                )),
            )]
            .into_iter()
            .collect(),
        );
        let result = define_structs(&fields, "Struct", None, &WipOptions::minimal()).unwrap();
        assert_tokens(
            result,
            quote!(
                #[allow(non_camel_case_types)]
                pub struct Struct {
                    pub nested: Struct__nested,
                }

                #[allow(non_camel_case_types)]
                pub struct Struct__nested {
                    pub nested_again: Struct__nested__nested_again,
                }

                #[allow(non_camel_case_types)]
                pub struct Struct__nested__nested_again {
                    pub inner: bool,
                }
            ),
        );
    }

    #[test]
    fn structs_in_tuples() {
        let fields = Struct(
            [(
                "tuple".into(),
                Value::Tuple(vec![
                    Value::Bool(true),
                    Value::Struct(Struct(
                        [("struct_a".into(), Value::Bool(true))]
                            .into_iter()
                            .collect(),
                    )),
                    Value::Struct(Struct(
                        [("struct_b".into(), Value::Bool(true))]
                            .into_iter()
                            .collect(),
                    )),
                ]),
            )]
            .into_iter()
            .collect(),
        );
        let result = define_structs(&fields, "Struct", None, &WipOptions::minimal()).unwrap();
        assert_tokens(
            result,
            quote!(
                #[allow(non_camel_case_types)]
                pub struct Struct {
                    pub tuple: (bool, Struct__tuple__1, Struct__tuple__2),
                }

                #[allow(non_camel_case_types)]
                pub struct Struct__tuple__1 {
                    pub struct_a: bool,
                }

                #[allow(non_camel_case_types)]
                pub struct Struct__tuple__2 {
                    pub struct_b: bool,
                }
            ),
        );
    }

    #[test]
    fn simple_enum() {
        let mapping = Struct(
            [
                ("First".into(), Value::I32(1)),
                ("Second".into(), Value::I32(2)),
            ]
            .into_iter()
            .collect(),
        );
        let result = define_enum_from_keys(&mapping, "Enum", None, &WipOptions::minimal()).unwrap();

        assert_tokens(
            result,
            quote!(
                pub enum Enum {
                    First,
                    Second,
                }
            ),
        );
    }

    #[test]
    fn simple_enum_with_file() {
        let mapping = Struct(
            [
                ("First".into(), Value::I32(1)),
                ("Second".into(), Value::I32(2)),
            ]
            .into_iter()
            .collect(),
        );
        let result = define_enum_from_keys(
            &mapping,
            "Enum",
            Some("./path/to/file.toml".as_ref()),
            &WipOptions {
                source_path_const_name: Some("SOURCE_PATH".into()),
                ..WipOptions::minimal()
            },
        )
        .unwrap();

        assert_tokens(
            result,
            quote!(pub enum Enum {
                First,
                Second,
            }
            impl Enum {
                pub const SOURCE_PATH: &'static str = "./path/to/file.toml";
            }),
        );
    }

    #[test]
    fn enum_with_derives() {
        let mapping = Struct(
            [
                ("First".into(), Value::I32(1)),
                ("Second".into(), Value::I32(2)),
            ]
            .into_iter()
            .collect(),
        );
        let result = define_enum_from_keys(
            &mapping,
            "Enum",
            None,
            &WipOptions {
                derived_traits: vec!["Clone".into()].into(),
                serde_support: SerdeSupport::Yes,
                all_variants_const_name: Some("VARIANTS".into()),
                impl_default: true,
                impl_display: true,
                impl_from_str: true,
                ..WipOptions::minimal()
            },
        )
        .unwrap();

        assert_tokens(
            result,
            quote! {
                #[derive(Clone, serde::Serialize, serde::Deserialize, Debug)]
                pub enum Enum {
                    First,
                    Second,
                }

                impl Enum {
                    pub const VARIANTS: &'static [Self] = &[Self::First, Self::Second, ];
                }

                impl Default for Enum {
                    fn default() -> Self {
                        Self::First
                    }
                }

                impl std::fmt::Display for Enum {
                    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
                        <Self as std::fmt::Debug>::fmt(self, f)
                    }
                }

                impl std::str::FromStr for Enum {
                    type Err = ();

                    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
                        Ok(match s {
                            "First" => Self::First,
                            "Second" => Self::Second,
                            _ => return Err(())
                        })
                    }
                }
            },
        );
    }

    #[test]
    fn enum_with_const_values() {
        let mapping = Struct(
            [(
                "First".into(),
                Value::Struct(Struct([("key".into(), Value::Unit)].into_iter().collect())),
            )]
            .into_iter()
            .collect(),
        );
        let result = define_enum_from_keys(
            &mapping,
            "Enum",
            None,
            &WipOptions {
                derived_traits: vec!["Clone".into()].into(),
                serde_support: SerdeSupport::Yes,
                all_variants_const_name: Some("VARIANTS".into()),
                impl_default: true,
                impl_display: true,
                impl_from_str: true,
                ..WipOptions::default()
            },
        )
        .unwrap();

        assert_tokens(
            result,
            quote! {
                #[derive(Clone, serde::Serialize, serde::Deserialize, Debug)]
                pub enum Enum {
                    First,
                }

                impl Enum {
                    pub const VARIANTS: &'static [Self] = &[Self::First, ];
                    pub const VALUES: &'static [Enum__Value] = &[Enum__Value { key: (), }, ];
                    pub const fn get(&self) -> &'static Enum__Value { &Self::VALUES[*self as usize] }
                }

                impl Default for Enum {
                    fn default() -> Self {
                        Self::First
                    }
                }

                impl std::fmt::Display for Enum {
                    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
                        <Self as std::fmt::Debug>::fmt(self, f)
                    }
                }

                impl std::str::FromStr for Enum {
                    type Err = ();

                    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
                        Ok(match s {
                            "First" => Self::First,
                            _ => return Err(())
                        })
                    }
                }

                #[allow(non_camel_case_types)]
                #[derive(Debug)]
                pub struct Enum__Value {
                    pub key: (),
                }
            },
        );
    }

    #[test]
    fn define_consts_from_map_values() {
        let fields = Struct(
            [("Key1".into(), Value::Unit), ("Key2".into(), Value::Unit)]
                .into_iter()
                .collect(),
        );
        let result = define_structs_from_values(&fields, "Struct", &WipOptions::default()).unwrap();

        assert_tokens(
            result,
            quote! {
                pub const VALUES: &'static [()] = &[(), (),];
            },
        );
    }

    #[test]
    fn define_nothing_from_map_values() {
        let fields = Struct(
            [("Key1".into(), Value::Unit), ("Key2".into(), Value::Unit)]
                .into_iter()
                .collect(),
        );
        let result = define_structs_from_values(&fields, "Struct", &WipOptions::minimal()).unwrap();

        assert_tokens(result, quote!());
    }

    #[test]
    fn define_structs_from_map_values() {
        let fields = Struct(
            [
                (
                    "Key1".into(),
                    Value::Struct(Struct(
                        [("key".into(), Value::Bool(true))].into_iter().collect(),
                    )),
                ),
                (
                    "Key2".into(),
                    Value::Struct(Struct(
                        [("key".into(), Value::Bool(false))].into_iter().collect(),
                    )),
                ),
            ]
            .into_iter()
            .collect(),
        );
        let result = define_structs_from_values(&fields, "Struct", &WipOptions::default()).unwrap();

        assert_tokens(
            result,
            quote! {
                #[allow(non_camel_case_types)]
                #[derive(Debug)]
                pub struct Struct {
                    pub key: bool,
                }

                pub const VALUES: &'static [Struct] = &[
                    Struct { key: true, },
                    Struct { key: false, },
                ];
            },
        );
    }

    #[test]
    fn enum_from_filenames() {
        let result =
            define_enum_from_filenames("src".as_ref(), "FileName", &WipOptions::minimal()).unwrap();
        assert_tokens(
            result,
            quote! {
                pub enum FileName {
                    Enums,
                    Error,
                    Files,
                    FilesEnum,
                    Format,
                    Gen,
                    Generation,
                    Lib,
                    LoadFns,
                    Options,
                    Structs,
                    Validation,
                    Value,
                }
            },
        );
    }

    #[test]
    fn enum_from_filenames_with_consts() {
        let result = define_enum_from_filenames(
            "src".as_ref(),
            "FileName",
            &WipOptions {
                file_strings_const_name: Some("STRINGS".into()),
                get_string_fn_name: Some("string".into()),
                file_bytes_const_name: Some("BYTES".into()),
                get_bytes_fn_name: Some("bytes".into()),
                ..WipOptions::new()
            },
        )
        .unwrap();
        assert_tokens(
            result,
            quote! {
                #[derive(Debug)]
                pub enum FileName {
                    Enums,
                    Error,
                    Files,
                    FilesEnum,
                    Format,
                    Gen,
                    Generation,
                    Lib,
                    LoadFns,
                    Options,
                    Structs,
                    Validation,
                    Value,
                }

                impl FileName {
                    pub const FILE_PATHS: &'static [&'static str] = &[
                        "src/enums.rs",
                        "src/error.rs",
                        "src/files.rs",
                        "src/files_enum.rs",
                        "src/format.rs",
                        "src/gen.rs",
                        "src/generation.rs",
                        "src/lib.rs",
                        "src/load_fns.rs",
                        "src/options.rs",
                        "src/structs.rs",
                        "src/validation.rs",
                        "src/value.rs",
                    ];
                    pub const fn path(&self) -> &'static str {
                        Self::FILE_PATHS[*self as usize]
                    }
                    pub const BYTES: &'static [&'static [u8]] = &[
                        include_bytes!("src/enums.rs"),
                        include_bytes!("src/error.rs"),
                        include_bytes!("src/files.rs"),
                        include_bytes!("src/files_enum.rs"),
                        include_bytes!("src/format.rs"),
                        include_bytes!("src/gen.rs"),
                        include_bytes!("src/generation.rs"),
                        include_bytes!("src/lib.rs"),
                        include_bytes!("src/load_fns.rs"),
                        include_bytes!("src/options.rs"),
                        include_bytes!("src/structs.rs"),
                        include_bytes!("src/validation.rs"),
                        include_bytes!("src/value.rs"),
                    ];
                    pub const fn bytes(&self) -> &'static [u8] {
                        Self::BYTES[*self as usize]
                    }
                    pub const STRINGS: &'static [&'static str] = &[
                        include_str!("src/enums.rs"),
                        include_str!("src/error.rs"),
                        include_str!("src/files.rs"),
                        include_str!("src/files_enum.rs"),
                        include_str!("src/format.rs"),
                        include_str!("src/gen.rs"),
                        include_str!("src/generation.rs"),
                        include_str!("src/lib.rs"),
                        include_str!("src/load_fns.rs"),
                        include_str!("src/options.rs"),
                        include_str!("src/structs.rs"),
                        include_str!("src/validation.rs"),
                        include_str!("src/value.rs"),
                    ];
                    pub const fn string(&self) -> &'static str {
                        Self::STRINGS[*self as usize]
                    }
                    pub const SOURCE_PATH: &'static str = "src";
                    pub const ALL: &'static [Self] = &[
                        Self::Enums,
                        Self::Error,
                        Self::Files,
                        Self::FilesEnum,
                        Self::Format,
                        Self::Gen,
                        Self::Generation,
                        Self::Lib,
                        Self::LoadFns,
                        Self::Options,
                        Self::Structs,
                        Self::Validation,
                        Self::Value,
                    ];
                }
            },
        );
    }
}
