use std::fmt::{Debug, Formatter};
use crate::hex::ToHexStringBorrowed;

pub mod decoder;
pub mod lookup;

#[derive(Clone)]
pub struct BerTlv {
    pub tag: u32,
    pub length: usize,
    pub value: PrimitiveOrConstructedValue,
}

#[derive(Clone)]
pub enum PrimitiveOrConstructedValue {
    Primitive(Vec<u8>),
    Constructed(Vec<BerTlv>)
}

impl BerTlv {
    pub fn optional_primitive_value(&self) -> Option<&Vec<u8>> {
        match &self.value {
            PrimitiveOrConstructedValue::Primitive(v) => Some(v),
            _ => None
        }
    }

    pub fn optional_constructed_value(&self) -> Option<&Vec<BerTlv>> {
        match &self.value {
            PrimitiveOrConstructedValue::Constructed(v) => Some(v),
            _ => None
        }
    }
}

impl Debug for BerTlv {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "BerTlv {{ tag: {:#X}, length: {}, value: {:?} }}", self.tag, self.length, self.value)
    }
}

impl Debug for PrimitiveOrConstructedValue {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            PrimitiveOrConstructedValue::Primitive(v) => {
                write!(
                    f,
                    "{:?}",
                    v.to_hex_string()
                )
            },
            PrimitiveOrConstructedValue::Constructed(v) => write!(f, "{:?}", v),
        }
    }
}