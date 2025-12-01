use crate::emv::card::image::ApplicationDefinitionFile;
use crate::emv::card::reader::{error};
use crate::emv::card::reader::apdu_exchanger::EmvApduExchanger;
use crate::smartcard::reader::SmartCardChannel;

pub fn read_application_definition_file(
    apdu_exchanger: &mut EmvApduExchanger<impl SmartCardChannel>,
    application_identifier: &Vec<u8>
) -> Result<Option<ApplicationDefinitionFile>, error::Error> {
    todo!()
}