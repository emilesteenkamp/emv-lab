use log::info;
use crate::smartcard::reader::{Error, SmartCardChannel, SmartCardReader};

mod smartcard;

fn main() -> Result<(), Error> {
    env_logger::init();

    let smart_card_reader = smartcard::pcsc_reader::PcscSmartCardReaderReader::new()?;
    let mut smart_card_channel = smart_card_reader.connect()?;

    let send_apdu = [0xFF, 0xCA, 0x00, 0x00, 0x00];
    let mut receive_apdu_buffer = [0u8; 264];

    info!("Send APDU: {}", send_apdu.to_apdu_string());

    let receive_apdu = smart_card_channel
        .exchange_apdu(&send_apdu, &mut receive_apdu_buffer)?;

    info!("Receive APDU: {}", receive_apdu.to_apdu_string());

    smart_card_channel.reset()?;

    Ok(())
}

trait ToApduString {
    fn to_apdu_string(&self) -> String;
}

impl ToApduString for [u8] {
    fn to_apdu_string(&self) -> String {
        format!("{:02X?}", self)
    }
}
