use log::info;
use crate::emv::card::constants::{EMV_RESPONSE_BUFFER_SIZE, PPSE_FILE_NAME};
use crate::hex::ToHexStringBorrowed;
use crate::pcsc::smartcard::reader::PcscSmartCardReader;
use crate::smartcard::apdu::command::{builders, CommandApdu};
use crate::smartcard::apdu::response::ResponseApdu;
use crate::smartcard::reader::{SmartCardChannel, SmartCardReader};
use crate::tlv;

pub fn run() -> anyhow::Result<()> {
    let smart_card_reader = PcscSmartCardReader::new()?;
    let mut smart_card_channel = smart_card_reader.connect()?;
    let mut response_apdu_buffer = [0u8; EMV_RESPONSE_BUFFER_SIZE];

    let command_apdu = builders::select(Vec::from(PPSE_FILE_NAME)).with_le(0x00);
    info!("Command APDU:  {}", command_apdu.to_apdu_string());
    let response_apdu = smart_card_channel
        .exchange_apdu(command_apdu, &mut response_apdu_buffer)?;
    info!("Response APDU: {}", response_apdu.to_apdu_string());

    smart_card_channel.reset()?;

    let ber_tlv_vec = tlv::ber::decoder::decode(&response_apdu.data)?;
    info!("ber_tlv_vec: {:?}", ber_tlv_vec);

    Ok(())
}

trait ToApduString {
    fn to_apdu_string(&self) -> String;
}

impl ToApduString for CommandApdu {
    fn to_apdu_string(&self) -> String {
        self.to_bytes().to_hex_string()
    }
}

impl ToApduString for ResponseApdu<'_> {
    fn to_apdu_string(&self) -> String {
        let s1 = self.data.to_hex_string();
        let s2 = [self.sw1, self.sw2].to_hex_string();
        s1 + s2.as_str()
    }
}