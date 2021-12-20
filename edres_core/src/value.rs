use indexmap::IndexMap;

use crate::error::Error;

#[derive(Debug, Default, Clone)]
pub struct Struct(pub(crate) IndexMap<String, Value>);

/// A generic value.
#[derive(Debug, Clone)]
pub enum Value {
    Unit,
    Bool(bool),
    Char(char),
    I8(i8),
    I16(i16),
    I32(i32),
    I64(i64),
    I128(i128),
    ISize(isize),
    U8(u8),
    U16(u16),
    U32(u32),
    U64(u64),
    U128(u128),
    USize(usize),
    F32(f32),
    F64(f64),
    String(String),
    Option(Option<Box<Value>>),
    Tuple(Vec<Value>),
    Array(usize, Vec<Value>),
    Vec(Vec<Value>),
    Struct(Struct),
}

impl Value {
    pub fn assume_struct(self) -> Result<Struct, Error> {
        match self {
            Value::Struct(s) => Ok(s),
            x => {
                let tag = match x {
                    Value::Unit => "()",
                    Value::Bool(_) => "bool",
                    Value::Char(_) => "char",
                    Value::I8(_) => "i8",
                    Value::I16(_) => "i16",
                    Value::I32(_) => "i32",
                    Value::I64(_) => "i64",
                    Value::I128(_) => "i128",
                    Value::ISize(_) => "isize",
                    Value::U8(_) => "u8",
                    Value::U16(_) => "u16",
                    Value::U32(_) => "u32",
                    Value::U64(_) => "u64",
                    Value::U128(_) => "u128",
                    Value::USize(_) => "usize",
                    Value::F32(_) => "f32",
                    Value::F64(_) => "f64",
                    Value::String(_) => "String",
                    Value::Option(_) => "Option",
                    Value::Tuple(_) => "tuple",
                    Value::Array(..) => "array",
                    Value::Vec(_) => "Vec",
                    Value::Struct(_) => unreachable!(),
                };
                Err(Error::ExpectedStruct(tag))
            }
        }
    }
}
