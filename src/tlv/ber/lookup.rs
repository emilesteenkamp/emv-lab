use crate::tlv::ber::BerTlv;

pub trait BerTlvWalker {
    /// Walks through a BerTlv tree using the given `path`.
    ///
    /// Will return `Some(ber_tlv)` in the case that a tlv is found in the exact path, in all other
    /// cases will return `None`.
    ///
    /// In the case of an empty `path`, returns `None`.
    fn walk(&self, path: &Vec<u32>) -> Option<&BerTlv>;
}

pub trait BerTlvLookup {
    fn find_tag(&self, tag_number: u32) -> Option<&BerTlv>;
}

impl BerTlvWalker for BerTlv {
    fn walk(&self, path: &Vec<u32>) -> Option<&BerTlv> {
        if path.is_empty() { return None; }
        let mut current = self;
        for tag in path.iter() {
            current = current.optional_constructed_value()?.find_tag(*tag)?;
        };
        Some(current)
    }
}

impl BerTlvLookup for BerTlv {
    fn find_tag(&self, tag_number: u32) -> Option<&BerTlv> {
        self.optional_constructed_value()?.find_tag(tag_number)
    }
}

impl BerTlvLookup for Vec<BerTlv> {
    fn find_tag(&self, tag_number: u32) -> Option<&BerTlv> {
        self.iter().find(|tag| tag.tag == tag_number)
    }
}