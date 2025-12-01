use crate::tlv::ber::BerTagLengthValue;

pub trait BerTlvWalker {
    /// Walks through a BerTlv tree using the given `path`.
    /// 
    /// Will return `Some(ber_tlv)` in the case that a tlv is found in the exact path, in all other
    /// cases will return `None`.
    ///
    /// In the case of an empty `path`, returns `None`.
    fn walk(&self, path: &Vec<u32>) -> Option<&BerTagLengthValue>;
}

pub trait BerTlvLookup {
    fn find_tag(&self, tag_number: u32) -> Option<&BerTagLengthValue>;
}

impl BerTlvWalker for BerTagLengthValue {
    fn walk(&self, path: &Vec<u32>) -> Option<&BerTagLengthValue> {
        if path.is_empty() { return None; }
        let mut current = self;
        for tag in path.iter() {
            current = current.optional_constructed_value()?.find_tag(*tag)?;
        };
        Some(current)
    }
}

impl BerTlvLookup for Vec<BerTagLengthValue> {
    fn find_tag(&self, tag_number: u32) -> Option<&BerTagLengthValue> {
        self.iter().find(|tag| tag.tag == tag_number)
    }
}