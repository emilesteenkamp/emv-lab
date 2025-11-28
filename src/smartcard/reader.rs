#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Error {
    InternalError,
    ReaderNotFound
}

pub trait SmartCardChannel {
    fn exchange_apdu<'buf>(
        &mut self,
        send_apdu: &[u8],
        receive_apdu_buffer: &'buf mut [u8; 264]
    ) -> Result<&'buf [u8], Error>;

    fn reset(self) -> Result<(), Error>;
}

pub trait SmartCardReader {
    type SmartCardChannel: SmartCardChannel;

    fn connect(&self) -> Result<Self::SmartCardChannel, Error>;
}
