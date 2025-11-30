use crate::emv::card::reader::read_emv_card;
use crate::pcsc::smartcard::reader::PcscSmartCardReader;

pub fn run() -> anyhow::Result<()> {
    let smart_card_reader = PcscSmartCardReader::new()?;
    
    read_emv_card(smart_card_reader)?;
    
    Ok(())
}