//! Contains the `Value` type - a markup-agnostic generic value.

use indexmap::IndexMap;

use crate::error::Error;

/// A key-value object for representing both maps and structs.
#[derive(Debug, Default, Clone, PartialEq)]
pub struct Struct(pub(crate) IndexMap<String, Value>);

impl Struct {
    pub fn from_pairs<S: Into<String>>(pairs: impl IntoIterator<Item = (S, Value)>) -> Self {
        Struct(pairs.into_iter().map(|(k, v)| (k.into(), v)).collect())
    }
}

/// A type alias for `Struct`.
pub type Map = Struct;

/// A generic value.
#[derive(Debug, Clone, PartialEq)]
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

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn assume_struct() {
        let bads = [
            Value::Unit,
            Value::Bool(true),
            Value::Char('a'),
            Value::I8(1),
            Value::I16(2),
            Value::I32(3),
            Value::I64(4),
            Value::I128(5),
            Value::ISize(6),
            Value::U8(7),
            Value::U16(8),
            Value::U32(9),
            Value::U64(10),
            Value::U128(11),
            Value::USize(12),
            Value::F32(13.0),
            Value::F64(14.0),
            Value::String("String".into()),
            Value::Option(None),
            Value::Tuple(vec![Value::Unit]),
            Value::Array(1, vec![Value::Unit]),
            Value::Vec(vec![Value::Unit]),
        ];
        let good = Value::Struct(Struct([("key".into(), Value::Unit)].into_iter().collect()));

        assert!(good.assume_struct().is_ok());
        for bad in bads {
            assert!(bad.assume_struct().is_err());
        }
    }
}
