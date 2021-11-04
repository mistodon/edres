use proc_macro2::TokenStream;
use quote::{format_ident, quote};

use crate::error::WipError;
use crate::options::WipOptions;
use crate::parsing;
use crate::value::{Struct, Value};

fn type_of_value<'a>(
    value: &'a Value,
    options: &WipOptions,
    in_struct: &str,
    under_key: &str,
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
            Some(value) => type_of_value(value, options, in_struct, under_key, None, new_structs)?,
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
                        in_struct,
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
                let inner_type =
                    type_of_value(&values[0], options, in_struct, under_key, None, new_structs)?;
                quote!(std::borrow::Cow<'static, [#inner_type]>)
            }
        },
        Value::Tuple(values) => {
            let types_in_tuple = values
                .iter()
                .enumerate()
                .map(|(i, v)| type_of_value(v, options, in_struct, under_key, Some(i), new_structs))
                .collect::<Result<Vec<_>, WipError>>()?;
            quote!((#(#types_in_tuple),*))
        }
        Value::Struct(mapping) => {
            let index = match under_index {
                None => String::new(),
                Some(i) => format!("__{}", i),
            };
            let name = format!("{}__{}{}", in_struct, under_key, index);
            let ident = format_ident!("{}", name);
            new_structs.push((name, mapping));

            quote!(#ident)
        }
    })
}

pub fn define_structs(
    data: &Struct,
    struct_name: &str,
    options: &WipOptions,
) -> Result<TokenStream, WipError> {
    let mut traits = options.derived_traits.clone();
    let (ser, de) = options
        .serde_support
        .should_derive_ser_de()
        .unwrap_or((false, false));
    if ser {
        traits.push("serde::Serialize".into())
    }
    if de {
        traits.push("serde::Deserialize".into())
    }

    let mut derives = vec![];
    for trait_path in traits {
        match trait_path.split_once("::") {
            Some((crate_name, trait_name)) => {
                let crate_name = format_ident!("{}", crate_name);
                let trait_name = format_ident!("{}", trait_name);
                derives.push(quote!(#crate_name :: #trait_name));
            }
            None => {
                let tokens = format_ident!("{}", trait_path);
                derives.push(quote!(#tokens));
            }
        }
    }

    let derive_string = (derives.len() > 0).then(|| quote!(#[derive(#(#derives),*)]));

    define_structs_inner(data, struct_name, options, derive_string.as_ref())
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
        let decl = type_of_value(value, options, struct_name, key, None, &mut sub_structs)?;
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

fn define_value(
    value: &Value,
    options: &WipOptions,
    in_struct: &str,
    under_key: &str,
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
                let value = define_value(x, options, in_struct, under_key, under_index)?;
                quote!(Some(#value))
            }
            None => quote!(None),
        },
        Value::Array(_, values) => {
            let values = values
                .iter()
                .map(|value| define_value(value, options, in_struct, under_key, under_index))
                .collect::<Result<Vec<_>, WipError>>()?;
            quote!([#(#values),*])
        }
        Value::Vec(values) => {
            let values = values
                .iter()
                .map(|value| define_value(value, options, in_struct, under_key, under_index))
                .collect::<Result<Vec<_>, WipError>>()?;
            quote!(std::borrow::Cow::Borrowed(&'static [#(#values),*]))
        }
        Value::Tuple(values) => {
            let values = values
                .iter()
                .enumerate()
                .map(|(i, value)| define_value(value, options, in_struct, under_key, Some(i)))
                .collect::<Result<Vec<_>, WipError>>()?;
            quote!((#(#values),*))
        }
        Value::Struct(fields) => {
            let index = match under_index {
                None => String::new(),
                Some(i) => format!("__{}", i),
            };
            let name = format!("{}__{}{}", in_struct, under_key, index);
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
        let value = define_value(value, options, struct_name, key, None)?;
        let key = format_ident!("{}", key);
        fields.push(quote!(pub #key: #value,));
    }

    let struct_name = format_ident!("{}", struct_name);
    Ok(quote! {
        #struct_name {
            #(#fields)*
        }
    })
}

pub fn define_const(
    data: &Struct,
    struct_name: &str,
    const_name: &str,
    options: &WipOptions,
) -> Result<TokenStream, WipError> {
    let struct_value = define_struct_value(data, options, struct_name)?;
    let struct_name = format_ident!("{}", struct_name);
    let const_name = format_ident!("{}", const_name);
    let tokens = quote!(pub const #const_name: #struct_name = #struct_value);
    Ok(tokens)
}

// TODO: Derives etc. and also Default, FromStr/Display, const array...
pub fn define_enum_from_keys(
    data: &Struct,
    enum_name: &str,
    _options: &WipOptions,
) -> Result<TokenStream, WipError> {
    let enum_name = format_ident!("{}", enum_name);
    let enum_variants = data.0.keys();
    let tokens = quote! {
        pub enum #enum_name {
            #(#enum_variants),*
        }
    };
    Ok(tokens)
}

// TODO: Derives etc.
pub fn define_structs_from_values(
    data: &Struct,
    struct_name: &str,
    options: &WipOptions,
) -> Result<TokenStream, WipError> {
    let mut values = data.0.values().map(Clone::clone).collect::<Vec<_>>();
    if values.is_empty() {
        return Err(WipError("Need values in map".into()));
    }
    parsing::unify_values(&mut values)?;
    if let Value::Struct(fields) = &values[0] {
        define_structs(fields, struct_name, options)
    } else {
        Err(WipError("Values must be structs".into()))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn assert_tokens(a: TokenStream, b: TokenStream) {
        assert_eq!(a.to_string(), b.to_string())
    }

    #[test]
    fn empty_struct() {
        let fields = Struct([].into_iter().collect());
        let result = define_structs(&fields, "Struct", &WipOptions::default()).unwrap();
        assert_tokens(
            result,
            quote!(
                #[allow(non_camel_case_types)]
                pub struct Struct {}
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
        let result = define_structs(&fields, "Struct", &WipOptions::default()).unwrap();
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
        let result = define_structs(&fields, "Struct", &WipOptions::default()).unwrap();
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
        let result = define_structs(&fields, "Struct", &WipOptions::default()).unwrap();
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
        let result = define_structs(&fields, "Struct", &WipOptions::default()).unwrap();
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
            &WipOptions {
                derived_traits: vec!["Debug".into(), "Clone".into()],
                serde_support: crate::options::SerdeSupport::Yes,
                ..Default::default()
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
        let result = define_structs(&fields, "Struct", &WipOptions::default()).unwrap();
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
        let result = define_structs(&fields, "Struct", &WipOptions::default()).unwrap();
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
}
