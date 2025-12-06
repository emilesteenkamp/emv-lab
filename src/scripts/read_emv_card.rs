use log::info;
use crate::emv::card::reader::read_emv_card;
use crate::pcsc::smartcard::reader::PcscSmartCardReader;

pub fn run() -> anyhow::Result<()> {
    let smart_card_reader = PcscSmartCardReader::new()?;
    
    let emv_card_image = read_emv_card(smart_card_reader)?;
    info!("emv_card_image: {:?}", emv_card_image);
    
    Ok(())
}