use crate::emv;
use crate::smartcard::reader::SmartCardReader;

pub fn read_emv_card<R: SmartCardReader>(
    smart_card_reader: R
) -> Result<emv::card::image::EmvCardImage, ()> {
    todo!()
}