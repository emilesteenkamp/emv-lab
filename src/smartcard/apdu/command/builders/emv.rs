use crate::smartcard::apdu::command::CommandApdu;

mod instruction_class {
    pub const EMV_PROPRIETARY: u8 = 0x00;
}

mod instruction_code {
    pub const GET_PROCESSING_OPTIONS: u8 = 0xA8;
}

pub fn get_processing_options(processing_options_data_objects_list: Vec<u8>) -> CommandApdu {
    CommandApdu {
        instruction_class: instruction_class::EMV_PROPRIETARY,
        instruction_code: instruction_code::GET_PROCESSING_OPTIONS,
        parameter_1: 0x00,
        parameter_2: 0x00,
        data: processing_options_data_objects_list,
        expected_length: Some(0xFF),
    }
}

