use std::fmt::{Display, Formatter};
use crate::smartcard::apdu::command::CommandApdu;
use crate::smartcard::apdu::response::ResponseApdu;

pub mod error {
    use std::fmt::{Display, Formatter};

    #[derive(Debug)]
    pub enum Error {
        InternalError,
        ReaderNotFound,
        EmptyApduResponse
    }

    impl Display for Error {
        fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
            write!(f, "SmartCardReaderError({:?})", self)
        }
    }

    impl std::error::Error for Error {}
}

pub trait SmartCardReader {
    type SmartCardChannel: SmartCardChannel;

    fn connect(&self) -> Result<Self::SmartCardChannel, error::Error>;
}

pub trait SmartCardChannel {
    fn exchange_apdu<'buf>(
        &mut self,
        send_apdu: CommandApdu,
        receive_apdu_buffer: &'buf mut [u8; 264]
    ) -> Result<ResponseApdu<'buf>, error::Error> {
        let command_apdu_bytes = send_apdu.to_bytes();
        let response_apdu_bytes = self.exchange_apdu_raw(&command_apdu_bytes, receive_apdu_buffer)?;
        ResponseApdu::from_bytes(&response_apdu_bytes).map_err(|_| error::Error::EmptyApduResponse)
    }

    fn exchange_apdu_raw<'buf>(
        &mut self,
        command_apdu: &[u8],
        response_apdu_buffer: &'buf mut [u8; 264]
    ) -> Result<&'buf [u8], error::Error>;

    fn reset(self) -> Result<(), error::Error>;
}
