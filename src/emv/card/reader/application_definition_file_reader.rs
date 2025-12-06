use crate::emv::card::image::ApplicationDefinitionFile;
use crate::emv::card::reader::{error, ErrorMapper};
use crate::emv::card::reader::apdu_exchanger::EmvApduExchanger;
use crate::emv::dictionary::{TAG_77, TAG_80, TAG_82, TAG_94, TAG_9F38};
use crate::smartcard::apdu::command::builders::emv::get_processing_options;
use crate::smartcard::apdu::command::builders::inter_industry::select;
use crate::smartcard::reader::SmartCardChannel;
use crate::tlv::ber::decoder::decode;
use crate::tlv::ber::lookup::BerTlvLookup;

mod application_descriptor {
    use crate::emv::dictionary::{TAG_77, TAG_80, TAG_82, TAG_94};
    use crate::tlv::ber::BerTlv;
    use crate::tlv::ber::lookup::BerTlvLookup;

    pub struct ApplicationDescriptor {
        pub application_interchange_profile: u16,
        pub application_file_locator: Vec<u8>
    }

    pub fn build(ber_tlv_vec: Vec<BerTlv>) -> Result<ApplicationDescriptor, ()> {
        match ber_tlv_vec.find_tag(TAG_80) {
            None => {}
            Some(ber_tlv) => match build_from_template_1(&ber_tlv) {
                Ok(application_descriptor) => return Ok(application_descriptor),
                Err(_) => {}
            }
        }

        match ber_tlv_vec.find_tag(TAG_77) {
            None => {}
            Some(ber_tlv) => match build_from_template_2(&ber_tlv) {
                Ok(application_descriptor) => return Ok(application_descriptor),
                Err(_) => {}
            }
        };

        Err(())
    }

    fn build_from_template_1(
        ber_tlv: &BerTlv
    ) -> Result<ApplicationDescriptor, ()> {
        match ber_tlv.optional_primitive_value() {
            None => Err(()),
            Some(primitive_value) => {
                if primitive_value.len() >= 2 {
                    let Some(byte_1) = primitive_value.get(0) else { return Err(()) };
                    let Some(byte_2) = primitive_value.get(1) else { return Err(()) };
                    let application_interchange_profile = ((byte_1.clone() as u16) << 8) | byte_2.clone() as u16;
                    let application_file_locator = Vec::from(&primitive_value[2..]);

                    Ok(
                        ApplicationDescriptor {
                            application_interchange_profile,
                            application_file_locator
                        }
                    )
                } else {
                    Err(())
                }
            }
        }
    }

    fn build_from_template_2(
        ber_tlv: &BerTlv
    ) -> Result<ApplicationDescriptor, ()> {
        let Some(application_interchange_profile) = ber_tlv.find_tag(TAG_82)
            .and_then(|ber_tlv| ber_tlv.optional_primitive_value())
            .and_then(|primitive_value| {
                let byte_1 = primitive_value.get(0)?.clone();
                let byte_2 = primitive_value.get(1)?.clone();
                Some(((byte_1 as u16) << 8) | byte_2 as u16)
            }) else { return Err(()) };

        let Some(application_file_locator) = ber_tlv.find_tag(TAG_94)
            .and_then(|ber_tlv| ber_tlv.optional_primitive_value())
            .map(|primitive_value| primitive_value.clone()) else { return Err(()) };

        Ok(
            ApplicationDescriptor {
                application_interchange_profile,
                application_file_locator,
            }
        )
    }
}

pub fn read_application_definition_file(
    apdu_exchanger: &mut EmvApduExchanger<impl SmartCardChannel>,
    application_identifier: &Vec<u8>
) -> Result<Option<ApplicationDefinitionFile>, error::Error> {
    // Select application.
    let command_apdu = select(application_identifier.clone());
    let response_apdu = apdu_exchanger.exchange(command_apdu)?;
    let Ok(data) = response_apdu.expect_ok() else { return Ok(None) };
    let ber_tlv_vec = decode(data).map_err_to_emv_reader_error()?;
    // TODO: Build PDOL data from PDOL definition.
    let _processing_data_object_list = match ber_tlv_vec.find_tag(TAG_9F38) {
        None => vec![],
        Some(ber_tlv) => ber_tlv
            .optional_primitive_value()
            .map(|primitive_value| primitive_value.clone())
            .unwrap_or(vec![])
    };

    // Get processing options.
    let command_apdu = get_processing_options(vec![]);
    let response_apdu = apdu_exchanger.exchange(command_apdu)?;
    let Ok(data) = response_apdu.expect_ok() else { return Err(error::Error::InvalidApduResponse) };
    let ber_tlv_vec = decode(data).map_err_to_emv_reader_error()?;
    let Ok(_application_descriptor) = application_descriptor::build(ber_tlv_vec) else {
        return Err(error::Error::UnableToConstructApplicationDescriptor)
    };

    // TODO: Read records from application descriptor.

    todo!()
}
