use crate::smartcard::apdu::command::{CommandApdu};

mod instruction_class {
    pub const INTER_INDUSTRY_STANDARD: u8 = 0x00;
}

mod instruction_code {
    pub const SELECT: u8 = 0xA4;
    pub const READ_RECORD: u8 = 0xB2;
}

pub fn select(file_name: Vec<u8>) -> CommandApdu {
    CommandApdu {
        instruction_class: instruction_class::INTER_INDUSTRY_STANDARD,
        instruction_code: instruction_code::SELECT,
        parameter_1: 0x04,
        parameter_2: 0x00,
        data: file_name,
        expected_length: Some(0xFF),
    }
}

pub fn read_record(short_file_identifier: u8, record_number: u8) -> CommandApdu {
    CommandApdu {
        instruction_class: instruction_class::INTER_INDUSTRY_STANDARD,
        instruction_code: instruction_code::READ_RECORD,
        parameter_1: record_number,
        parameter_2: (short_file_identifier << 3) | 0x04,
        data: vec![],
        expected_length: Some(0xFF),
    }
}