use std::fmt::{Debug, Formatter};

pub mod decoder;

#[derive(Clone)]
pub struct BerTlv {
    pub tag: u32,
    pub length: usize,
    pub value: BerValue,
}

#[derive(Clone)]
pub enum BerValue {
    Primitive(Vec<u8>),
    Constructed(Vec<BerTlv>)
}

impl Debug for BerTlv {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "BerTlv {{ tag: {:#X}, length: {}, value: {:?} }}", self.tag, self.length, self.value)
    }
}

impl Debug for BerValue {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            BerValue::Primitive(v) => {
                write!(
                    f,
                    "{:?}",
                    v.iter()
                        .map(|b| format!("{:02X}", b))
                        .collect::<Vec<_>>()
                        .join("")
                )
            },
            BerValue::Constructed(v) => write!(f, "{:?}", v),
        }
    }
}