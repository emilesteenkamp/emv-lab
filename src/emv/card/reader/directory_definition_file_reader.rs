use crate::emv::card::image::{DirectoryDefinitionFile, DirectoryEntry, EmvTag};
use crate::emv::card::reader::{error, ErrorMapper};
use crate::emv::card::reader::apdu_exchanger::EmvApduExchanger;
use crate::smartcard::apdu::command::builders::select;
use crate::smartcard::reader::SmartCardChannel;
use crate::tlv::ber::BerTagLengthValue;
use crate::tlv::ber::decoder::decode;
use crate::tlv::ber::lookup::{BerTlvLookup, BerTlvWalker};

pub fn read_directory_definition_file(
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
