use std::ffi::CStr;
use log::error;
use crate::smartcard::reader::{SmartCardChannel, SmartCardReader};
use crate::smartcard::reader::error::Error;

pub struct PcscSmartCardReader {
    pcsc_context: pcsc::Context
}

impl PcscSmartCardReader {
    pub fn new() -> Result<Self, Error> {
        let pcsc_context = pcsc::Context::establish(pcsc::Scope::User)
            .map_err_to_smart_card_reader_error()?;
        Ok(Self { pcsc_context })
    }

    fn find_first_reader_name<'buffer>(
        &self,
        reader_names_buffer: &'buffer mut [u8; 2048]
    ) -> Result<&'buffer CStr, Error> {
        let mut reader_names = self.pcsc_context
            .list_readers(reader_names_buffer)
            .map_err_to_smart_card_reader_error()?;

        match reader_names.next() {
            Some(cstr) => Ok(cstr),
            None => Err(Error::ReaderNotFound)
        }
    }
}

impl SmartCardReader for PcscSmartCardReader {
    type SmartCardChannel = PcscSmartCardChannel;

    fn connect(&self) -> Result<Self::SmartCardChannel, Error> {
        let mut readers_buf = [0u8; 2048];
        let first_reader_name = self.find_first_reader_name(&mut readers_buf)?;
        let pcsc_card = self.pcsc_context
            .connect(
                first_reader_name,
                pcsc::ShareMode::Shared,
                pcsc::Protocols::ANY
            )
            .map_err_to_smart_card_reader_error()?;

        Ok(PcscSmartCardChannel { pcsc_card })
    }
}

pub struct PcscSmartCardChannel {
    pcsc_card: pcsc::Card
}

impl SmartCardChannel for PcscSmartCardChannel {
    fn exchange_apdu_raw<'buffer>(
        &mut self,
        command_apdu: &[u8],
        response_apdu_buffer: &'buffer mut [u8; 264]
    ) -> Result<&'buffer [u8], Error> {
        self.pcsc_card.transmit(command_apdu, response_apdu_buffer).map_err_to_smart_card_reader_error()
    }

    fn reset(self) -> Result<(), Error> {
        self.pcsc_card
            .disconnect(pcsc::Disposition::ResetCard)
            .map_err(|err| err.1)
            .map_err_to_smart_card_reader_error()
    }
}

trait SmartCardReaderErrorMapper<SUCCESS>  {
    fn map_err_to_smart_card_reader_error(self) -> Result<SUCCESS, Error>;
}

impl<SUCCESS> SmartCardReaderErrorMapper<SUCCESS> for Result<SUCCESS, pcsc::Error> {
    fn map_err_to_smart_card_reader_error(self) -> Result<SUCCESS, Error> {
        self.map_err(|err| {
            error!("PC/SC error: {}", err);
            match err {
                pcsc::Error::InternalError => Error::InternalError,
                _ => Error::InternalError
            }
        })
    }
}