pub mod cla {
    pub const INTER_INDUSTRY_STANDARD: u8 = 0x00;
}

pub mod ins {
    pub const SELECT: u8 = 0xA4;
}

pub mod builders {
    use crate::smartcard::apdu::command::{cla, ins, CommandApdu};

    pub fn select(file_name: Vec<u8>) -> CommandApdu {
        CommandApdu {
            cla: cla::INTER_INDUSTRY_STANDARD,
            ins: ins::SELECT,
            p1: 0x04,
            p2: 0x00,
            data: file_name,
            le: None,
        }
    }
}

pub struct CommandApdu {
    pub cla: u8,
    pub ins: u8,
    pub p1:  u8,
    pub p2:  u8,
    pub data: Vec<u8>,
    pub le: Option<u8>,
}

impl CommandApdu {
    pub fn to_bytes(&self) -> Vec<u8> {
        let mut v = Vec::with_capacity(5 + self.data.len() + self.le.map(|_| 1).unwrap_or(0));

        v.push(self.cla);
        v.push(self.ins);
        v.push(self.p1);
        v.push(self.p2);

        if !self.data.is_empty() {
            v.push(self.data.len() as u8); // Lc
            v.extend_from_slice(&self.data);
        } else {
            v.push(0x00);
        }

        if let Some(le) = self.le {
            v.push(le);
        }

        v
    }
}