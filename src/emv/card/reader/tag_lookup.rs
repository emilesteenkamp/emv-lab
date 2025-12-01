use crate::emv::card::image::EmvTag;

pub trait EmvTagLookup {
    fn find_tag(&self, tag_number: u32) -> Option<&Vec<u8>>;
}

impl EmvTagLookup for Vec<EmvTag> {
    fn find_tag(&self, tag_number: u32) -> Option<&Vec<u8>> {
        self.iter().find(|tag| tag.number == tag_number).map(|tag| &tag.value)
    }
}