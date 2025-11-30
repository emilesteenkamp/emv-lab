use crate::emv::card::image::EmvCardImage;
use crate::smartcard::reader::SmartCardReader;
use crate::smartcard;

pub mod error {
    use std::fmt::{Display, Formatter};

    #[derive(Debug)]
    pub enum Error {
        SmartCardReaderError,
        ReaderNotFound,
        InvalidApduResponse,
    }

    impl Display for Error {
        fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
            write!(f, "EmvCardReaderError({:?})", self)
        }
    }

    impl std::error::Error for Error {}
}

pub fn read_emv_card<R: SmartCardReader>(
    reader: R
) -> Result<EmvCardImage, error::Error> {
    let mut image = EmvCardImage {
        directory_definition_file_vec: vec![],
        application_definition_file_vec: vec![],
    };
    let channel = reader.connect()
        .map_err(|error| match error {
            smartcard::reader::error::Error::InternalError => error::Error::SmartCardReaderError,
            smartcard::reader::error::Error::ReaderNotFound => error::Error::ReaderNotFound,
            smartcard::reader::error::Error::EmptyApduResponse => error::Error::InvalidApduResponse,
        })?;

    Ok(image)
}