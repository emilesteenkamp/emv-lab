use crate::emv::card::reader::read_emv_card;
use crate::smartcard::pcsc_reader::PcscSmartCardReader;

pub fn run() {
    let smart_card_reader = PcscSmartCardReader::new().unwrap();
    read_emv_card(smart_card_reader).unwrap();
}