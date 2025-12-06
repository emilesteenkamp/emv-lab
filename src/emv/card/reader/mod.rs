use log::{error};
use crate::emv::card::constants::{PPSE_FILE_NAME};
use crate::emv::card::image::{ApplicationDefinitionFile, DirectoryDefinitionFile, EmvCardImage, EmvTag};
use crate::emv::card::reader::apdu_exchanger::EmvApduExchanger;
use crate::emv::card::reader::application_definition_file_reader::read_application_definition_file;
use crate::emv::card::reader::directory_definition_file_reader::read_directory_definition_file;
use crate::emv::card::reader::tag_lookup::EmvTagLookup;
use crate::emv::dictionary::TAG_4F;
use crate::smartcard::reader::{SmartCardChannel, SmartCardReader};

mod apdu_exchanger;
mod application_definition_file_reader;
mod directory_definition_file_reader;
mod tag_lookup;
mod error_mapper;

pub mod error {
    use std::fmt::{Display, Formatter};

    #[derive(Debug)]
    pub enum Error {
        SmartCardReaderError,
        ReaderNotFound,
        InvalidApduResponse,
        UnableToConstructApplicationDescriptor,
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

    let channel = reader.connect().map_err_to_emv_reader_error()?;
    let mut apdu_exchanger = EmvApduExchanger::new(channel);

    if let Some(ppse_directory_definition_file) = read_directory_definition_file(
        &mut apdu_exchanger,
        Vec::from(PPSE_FILE_NAME)
    )? {
        image.directory_definition_file_vec.push(ppse_directory_definition_file);
    }

    for directory_definition_file in image.directory_definition_file_vec.iter() {
        let mut application_definition_file = read_all_application_definition_file(
            &mut apdu_exchanger,
            &directory_definition_file
        )?;
        image.application_definition_file_vec.append(&mut application_definition_file);
    }

    Ok(image)
}

fn read_all_application_definition_file(
    apdu_exchanger: &mut EmvApduExchanger<impl SmartCardChannel>,
    directory_definition_file: &DirectoryDefinitionFile
) -> Result<Vec<ApplicationDefinitionFile>, error::Error> {
    let mut application_definition_file_vec = vec![];

    for directory_entry in directory_definition_file.directory_entry_vec.iter() {
        if let Some(application_identifier) = directory_entry.tag_vec.find_tag(TAG_4F) {
                if let Some(application_definition_file) = read_application_definition_file(
                    apdu_exchanger,
                    application_identifier
                )? {
                    application_definition_file_vec.push(application_definition_file);
                }
            }
    }

    Ok(application_definition_file_vec)
}

trait ErrorMapper<SUCCESS>  {
    fn map_err_to_emv_reader_error(self) -> Result<SUCCESS, error::Error>;
}
