use log::{error, info};
use crate::emv::card::constants::{EMV_RESPONSE_BUFFER_SIZE, PPSE_FILE_NAME};
use crate::emv::card::image::{ApplicationDefinitionFile, DirectoryDefinitionFile, DirectoryEntry, EmvCardImage, EmvTag};
use crate::hex::ToHexStringBorrowed;
use crate::{smartcard, tlv};
use crate::smartcard::reader::{SmartCardChannel, SmartCardReader};
use crate::smartcard::apdu::command::builders::select;
use crate::smartcard::apdu::command::CommandApdu;
use crate::smartcard::apdu::response::ResponseApdu;
use crate::tlv::ber::BerTagLengthValue;
use crate::tlv::ber::decoder::decode;
use crate::tlv::ber::lookup::{BerTlvLookup, BerTlvWalker};

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

    let channel = reader.connect().map_err_to_emv_reader_error()?;
    let mut apdu_exchanger = EmvApduExchanger::new(channel);

    if let Some(ppse_directory_definition_file) = read_directory_definition_file(
        &mut apdu_exchanger,
        Vec::from(PPSE_FILE_NAME)
    )? {
        image.directory_definition_file_vec.push(ppse_directory_definition_file);
    }

    info!("Directory Definition Files: {:?}", image.directory_definition_file_vec);

    for directory_definition_file in image.directory_definition_file_vec.iter() {
        let mut application_definition_file = read_all_application_definition_file(
            &mut apdu_exchanger,
            &directory_definition_file
        )?;
        image.application_definition_file_vec.append(&mut application_definition_file);
    }

    Ok(image)
}

fn read_directory_definition_file(
    apdu_exchanger: &mut EmvApduExchanger<impl SmartCardChannel>,
    directory_definition_file_name: Vec<u8>
) -> Result<Option<DirectoryDefinitionFile>, error::Error> {
    let command_apdu = select(directory_definition_file_name.clone()).with_le(0x00);
    let response_apdu = apdu_exchanger.exchange(command_apdu)?;

    if !response_apdu.is_ok() {
        return Ok(None);
    }

    let ber_tlv_vec = decode(response_apdu.data).map_err_to_emv_reader_error()?;
    let tag_6f = match ber_tlv_vec.find_tag(0x6F) {
        None => return Ok(None),
        Some(ber_tlv) => ber_tlv
    };
    let tag_bf0c = match tag_6f.walk(&vec![0xA5, 0xBF0C]) {
        None => return Ok(None),
        Some(ber_tlv) => ber_tlv
    };

    match tag_bf0c.optional_constructed_value() {
        None => Ok(None),
        Some(ber_tlv_vec) => {
            let directory_entry_vec = ber_tlv_vec.iter()
                .filter_map(|ber_tlv| { construct_directory_entry(ber_tlv) })
                .collect();
            Ok(
                Some(
                    DirectoryDefinitionFile {
                        file_name: directory_definition_file_name,
                        directory_entry_vec
                    }
                )
            )
        }
    }
}

fn construct_directory_entry(
    ber_tlv_vec: &BerTagLengthValue,
) -> Option<DirectoryEntry> {
    let tag_vec = ber_tlv_vec.optional_constructed_value()?.iter()
        .filter_map(|ber_tlv| {
            Some(
                EmvTag {
                    number: ber_tlv.tag,
                    value: ber_tlv.optional_primitive_value()?.clone(),
                }
            )
        })
        .collect();
    Some(DirectoryEntry { tag_vec })
}

fn read_all_application_definition_file(
    apdu_exchanger: &mut EmvApduExchanger<impl SmartCardChannel>,
    directory_definition_file: &DirectoryDefinitionFile
) -> Result<Vec<ApplicationDefinitionFile>, error::Error> {
    let mut application_definition_file_vec = vec![];

    for directory_entry in directory_definition_file.directory_entry_vec.iter() {
        if let Some(application_identifier) = directory_entry.tag_vec.find_tag(0x4F) {
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

fn read_application_definition_file(
    apdu_exchanger: &mut EmvApduExchanger<impl SmartCardChannel>,
    application_identifier: &Vec<u8>
) -> Result<Option<ApplicationDefinitionFile>, error::Error> {
    todo!()
}

struct EmvApduExchanger<CHANNEL: SmartCardChannel> {
    response_apdu_buffer: [u8; EMV_RESPONSE_BUFFER_SIZE],
    smart_card_channel: CHANNEL
}

impl<CHANNEL: SmartCardChannel> EmvApduExchanger<CHANNEL> {
    fn new(channel: CHANNEL) -> Self {
        Self {
            response_apdu_buffer: [0u8; EMV_RESPONSE_BUFFER_SIZE],
            smart_card_channel: channel
        }
    }

    fn exchange(&mut self, command_apdu: CommandApdu) -> Result<ResponseApdu<'_>, error::Error> {
        info!("Command APDU:  {}", command_apdu.to_apdu_string());
        let response_apdu = self.smart_card_channel
            .exchange_apdu(command_apdu, &mut self.response_apdu_buffer)
            .map_err_to_emv_reader_error()?;
        info!("Response APDU: {}", response_apdu.to_apdu_string());

        Ok(response_apdu)
    }
}

trait EmvTagLookup {
    fn find_tag(&self, tag_number: u32) -> Option<&Vec<u8>>;
}

impl EmvTagLookup for Vec<EmvTag> {
    fn find_tag(&self, tag_number: u32) -> Option<&Vec<u8>> {
        self.iter().find(|tag| tag.number == tag_number).map(|tag| &tag.value)
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

trait EmvReaderErrorMapper<SUCCESS>  {
    fn map_err_to_emv_reader_error(self) -> Result<SUCCESS, error::Error>;
}

impl<SUCCESS> EmvReaderErrorMapper<SUCCESS> for Result<SUCCESS, smartcard::reader::error::Error> {
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

impl<SUCCESS> EmvReaderErrorMapper<SUCCESS> for Result<SUCCESS, tlv::ber::decoder::error::Error> {
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