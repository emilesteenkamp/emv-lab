use crate::tlv::ber::BerTagLengthValue;

pub trait BerTlvWalker {
    fn walk(&self, path: &Vec<u32>) -> Option<&BerTagLengthValue>;
}

pub trait BerTlvLookup {
    fn find_tag(&self, tag_number: u32) -> Option<&BerTagLengthValue>;
}

impl BerTlvWalker for BerTagLengthValue {
    fn walk(&self, path: &Vec<u32>) -> Option<&BerTagLengthValue> {
        let mut current = self;
        for tag in path.iter() {
            let Some(ber_tlv_vec) = current.optional_constructed_value() else { return None };
            match ber_tlv_vec.find_tag(*tag) {
                None => return None,
                Some(ber_tlv) => {
                    current = ber_tlv;
                }
            }
        };
        Some(current)
    }
}

impl BerTlvLookup for Vec<BerTagLengthValue> {
    fn find_tag(&self, tag_number: u32) -> Option<&BerTagLengthValue> {
        self.iter().find(|tag| tag.tag == tag_number)
    }
}