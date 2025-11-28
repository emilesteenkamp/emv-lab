use std::io::Read;
use log::info;
use crate::smartcard::apdu::command::CommandApdu;
use crate::smartcard::apdu::response::ResponseApdu;
use crate::smartcard::reader::{Error, SmartCardChannel, SmartCardReader};

mod smartcard;

const EMV_RESPONSE_BUFFER_SIZE: usize = 264;
const PPSE_FILE_NAME: [u8; 14] = [0x32, 0x50, 0x41, 0x59, 0x2E, 0x53, 0x59, 0x53, 0x2E, 0x44, 0x44, 0x46, 0x30, 0x31];

fn main() -> Result<(), Error> {
    env_logger::init();

    let smart_card_reader = smartcard::pcsc_reader::PcscSmartCardReaderReader::new()?;
    let mut smart_card_channel = smart_card_reader.connect()?;
    let mut response_apdu_buffer = [0u8; EMV_RESPONSE_BUFFER_SIZE];

    let command_apdu = smartcard::apdu::command::builders::select(Vec::from(PPSE_FILE_NAME));
    info!("Command APDU:  {}", command_apdu.to_apdu_string());
    let response_apdu = smart_card_channel
        .exchange_apdu(command_apdu, &mut response_apdu_buffer)?;
    info!("Response APDU: {}", response_apdu.to_apdu_string());

    smart_card_channel.reset()?;

    Ok(())
}

trait ToApduString {
    fn to_apdu_string(&self) -> String;
}

impl ToApduString for [u8] {
    fn to_apdu_string(&self) -> String {
        self.iter()
            .map(|b| format!("{:02X}", b))
            .collect::<Vec<_>>()
            .join(" ")
    }
}

impl ToApduString for CommandApdu {
    fn to_apdu_string(&self) -> String {
        self.to_bytes().to_apdu_string()
    }
}

impl<'a> ToApduString for ResponseApdu<'a> {
    fn to_apdu_string(&self) -> String {
        self.data
            .iter()
            .chain([self.sw1, self.sw2].iter())
            .map(|b| format!("{:02X}", b))
            .collect::<Vec<_>>()
            .join(" ")
    }
}