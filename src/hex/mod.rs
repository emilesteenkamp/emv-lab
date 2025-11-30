use std::slice::Iter;

pub trait ToHexString {
    fn to_hex_string(self) -> String;
}

pub trait ToHexStringBorrowed {
    fn to_hex_string(&self) -> String;
}

impl ToHexString for Iter<'_, u8> {
    fn to_hex_string(self) -> String {
        self.map(|b| format!("{:02X}", b))
            .collect::<Vec<_>>()
            .join("")
    }
}

impl ToHexStringBorrowed for Vec<u8> {
    fn to_hex_string(&self) -> String {
        self.iter().to_hex_string()
    }
}

impl ToHexStringBorrowed for [u8] {
    fn to_hex_string(&self) -> String {
        self.iter().to_hex_string()
    }
}

