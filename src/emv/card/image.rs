pub struct EmvCardImage {
    pub directory_definition_file_vec: Vec<DirectoryDefinitionFile>,
    pub application_definition_file_vec: Vec<ApplicationDefinitionFile>
}

pub struct DirectoryDefinitionFile {
    pub file_name: Vec<u8>,
    pub directory_entry_vec: Vec<DirectoryEntry>
}

pub struct DirectoryEntry {
    pub tag_vec: Vec<EmvTag>
}

pub struct ApplicationDefinitionFile {
    pub application_identifier: Vec<u8>,
    pub application_file_record_vec: Vec<ApplicationFileRecord>
}

pub struct ApplicationFileRecord {
    pub sfi: u8,
    pub record_number: u8,
    pub tag_vec: Vec<EmvTag>
}

pub struct EmvTag {
    pub number: u32,
    pub value: Vec<u8>
}
