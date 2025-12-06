pub mod builders;

pub mod ins {
    pub const SELECT: u8 = 0xA4;
    pub const READ_RECORD: u8 = 0xB2;
}

/// ISO-7816: Command APDU
pub struct CommandApdu {
    /// ISO-7816: CLA
    pub instruction_class: u8,
    /// ISO-7816: INS
    pub instruction_code: u8,
    /// ISO-7816: P1
    pub parameter_1:  u8,
    /// ISO-7816: P2
    pub parameter_2:  u8,
    /// ISO-7816: Command Data
    pub data: Vec<u8>,
    /// ISO-7816: Le
    pub expected_length: Option<u8>,
}

impl CommandApdu {
    pub fn with_le(mut self, le: u8) -> Self {
        self.expected_length = Some(le);
        self
    }

    pub fn with_no_le(mut self) -> Self {
        self.expected_length = None;
        self
    }

    pub fn to_bytes(&self) -> Vec<u8> {
        let mut bytes = Vec::with_capacity(5 + self.data.len() + self.expected_length.map(|_| 1).unwrap_or(0));

        bytes.push(self.instruction_class);
        bytes.push(self.instruction_code);
        bytes.push(self.parameter_1);
        bytes.push(self.parameter_2);

        if !self.data.is_empty() {
            bytes.push(self.data.len() as u8); // Lc
            bytes.extend_from_slice(&self.data);
        }

        if let Some(le) = self.expected_length {
            bytes.push(le);
        }

        bytes
    }
}