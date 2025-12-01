use log::info;
use crate::emv::card::constants::EMV_RESPONSE_BUFFER_SIZE;
use crate::emv::card::reader::{error, ErrorMapper};
use crate::hex::ToHexStringBorrowed;
use crate::smartcard::apdu::command::CommandApdu;
use crate::smartcard::apdu::response::ResponseApdu;
use crate::smartcard::reader::SmartCardChannel;

pub struct EmvApduExchanger<CHANNEL: SmartCardChannel> {
    response_apdu_buffer: [u8; EMV_RESPONSE_BUFFER_SIZE],
    smart_card_channel: CHANNEL
}

impl<CHANNEL: SmartCardChannel> EmvApduExchanger<CHANNEL> {
    pub fn new(channel: CHANNEL) -> Self {
        Self {
            response_apdu_buffer: [0u8; EMV_RESPONSE_BUFFER_SIZE],
            smart_card_channel: channel
        }
    }

    pub fn exchange(&mut self, command_apdu: CommandApdu) -> Result<ResponseApdu<'_>, error::Error> {
        info!("Command APDU:  {}", command_apdu.to_apdu_string());
        let response_apdu = self.smart_card_channel
            .exchange_apdu(command_apdu, &mut self.response_apdu_buffer)
            .map_err_to_emv_reader_error()?;
        info!("Response APDU: {}", response_apdu.to_apdu_string());

        Ok(response_apdu)
    }
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
        let data = self.data.to_hex_string();
        let sw12 = [self.sw1, self.sw2].to_hex_string();
        data + sw12.as_str()
    }
}
