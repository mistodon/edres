use indexmap::IndexMap;

use crate::error::WipError;

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
    pub fn assume_struct(self) -> Result<Struct, WipError> {
        match self {
            Value::Struct(s) => Ok(s),
            _ => Err(WipError("Expected struct".into())),
        }
    }
}
