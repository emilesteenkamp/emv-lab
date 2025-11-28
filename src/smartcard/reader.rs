use crate::smartcard::apdu::command::CommandApdu;
use crate::smartcard::apdu::response::ResponseApdu;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Error {
    InternalError,
    ReaderNotFound,
    EmptyApduResponse
}

pub trait SmartCardReader {
    type SmartCardChannel: SmartCardChannel;

    fn connect(&self) -> Result<Self::SmartCardChannel, Error>;
}

pub trait SmartCardChannel {
    fn exchange_apdu<'buf>(
        &mut self,
        send_apdu: CommandApdu,
        receive_apdu_buffer: &'buf mut [u8; 264]
    ) -> Result<ResponseApdu<'buf>, Error> {
        let command_apdu_bytes = send_apdu.to_bytes();
        let response_apdu_bytes = self.exchange_apdu_raw(&command_apdu_bytes, receive_apdu_buffer)?;
        ResponseApdu::from_bytes(&response_apdu_bytes).map_err(|_| Error::EmptyApduResponse)
    }

    fn exchange_apdu_raw<'buf>(
        &mut self,
        command_apdu: &[u8],
        response_apdu_buffer: &'buf mut [u8; 264]
    ) -> Result<&'buf [u8], Error>;

    fn reset(self) -> Result<(), Error>;
}
