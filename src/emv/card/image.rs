use std::fmt::{Debug, Formatter};
use crate::hex::ToHexStringBorrowed;

#[derive(Debug)]
pub struct EmvCardImage {
    pub directory_definition_file_vec: Vec<DirectoryDefinitionFile>,
    pub application_definition_file_vec: Vec<ApplicationDefinitionFile>
}

#[derive(Debug)]

pub struct DirectoryDefinitionFile {
    pub file_name: Vec<u8>,
    pub directory_entry_vec: Vec<DirectoryEntry>
}

#[derive(Debug)]
pub struct DirectoryEntry {
    pub tag_vec: Vec<EmvTag>
}

#[derive(Debug)]
pub struct ApplicationDefinitionFile {
    pub application_identifier: Vec<u8>,
    pub application_file_record_vec: Vec<ApplicationFileRecord>
}

#[derive(Debug)]
pub struct ApplicationFileRecord {
    pub sfi: u8,
    pub record_number: u8,
    pub tag_vec: Vec<EmvTag>
}

pub struct EmvTag {
    pub number: u32,
    pub value: Vec<u8>
}

impl Debug for EmvTag {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "EmvTag(tag: {:#X}, value: {:?})",
            self.number,
            self.value.to_hex_string()
        )
    }
}
