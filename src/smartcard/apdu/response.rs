pub struct ResponseApdu<'buf> {
    pub data: &'buf [u8],
    pub sw1: u8,
    pub sw2: u8
}

impl<'buf> ResponseApdu<'buf> {
    pub fn from_bytes(bytes: &'buf [u8]) -> Result<Self, ()> {
        if bytes.len() < 2 {
            return Err(());
        }
        let (data, sw) = bytes.split_at(bytes.len() - 2);
        Ok(Self {
            data,
            sw1: sw[0],
            sw2: sw[1],
        })
    }

    pub fn status_word(&self) -> u16 {
        ((self.sw1 as u16) << 8) | (self.sw2 as u16)
    }

    pub fn is_ok(&self) -> bool {
        self.sw1 == 0x90 && self.sw2 == 0x00
    }
}