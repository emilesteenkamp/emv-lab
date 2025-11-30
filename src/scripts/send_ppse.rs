use log::info;
use crate::pcsc::smartcard::reader::PcscSmartCardReader;
use crate::smartcard::apdu::command::{builders, CommandApdu};
use crate::smartcard::apdu::response::ResponseApdu;
use crate::smartcard::reader::{SmartCardChannel, SmartCardReader};
use crate::tlv;

const EMV_RESPONSE_BUFFER_SIZE: usize = 264;
const PPSE_FILE_NAME: &[u8; 14] = b"2PAY.SYS.DDF01";

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

impl ToApduString for [u8] {
    fn to_apdu_string(&self) -> String {
        self.iter()
            .map(|b| format!("{:02X}", b))
            .collect::<Vec<_>>()
            .join("")
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
            .join("")
    }
}