use crate::emv::card::reader::error;
use crate::{smartcard, tlv};

pub trait ErrorMapper<SUCCESS>  {
    fn map_err_to_emv_reader_error(self) -> Result<SUCCESS, error::Error>;
}

impl<SUCCESS> crate::emv::card::reader::ErrorMapper<SUCCESS> for Result<SUCCESS, smartcard::reader::error::Error> {
    fn map_err_to_emv_reader_error(self) -> Result<SUCCESS, error::Error> {
        self.map_err(|error| {
            error!("SmartCardReaderError: {}", error);
            match error {
                smartcard::reader::error::Error::InternalError => error::Error::SmartCardReaderError,
                smartcard::reader::error::Error::ReaderNotFound => error::Error::ReaderNotFound,
                smartcard::reader::error::Error::EmptyApduResponse => error::Error::InvalidApduResponse,
            }
        })
    }
}

impl<SUCCESS> crate::emv::card::reader::ErrorMapper<SUCCESS> for Result<SUCCESS, tlv::ber::decoder::error::Error> {
    fn map_err_to_emv_reader_error(self) -> Result<SUCCESS, error::Error> {
        self.map_err(|error| {
            error!("BerTlvDecoderError: {}", error);
            match error {
                tlv::ber::decoder::error::Error::UnexpectedEof => error::Error::InvalidApduResponse,
                tlv::ber::decoder::error::Error::IndefiniteLengthUnsupported =>
                    error::Error::InvalidApduResponse
            }
        })
    }
}