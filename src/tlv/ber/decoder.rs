use crate::tlv::ber::{BerTlv, BerValue};

pub mod error {
    #[derive(Debug)]
    pub enum Error {
        UnexpectedEof,
        IndefiniteLengthUnsupported,
    }

    impl std::fmt::Display for Error {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "BerTlvDecoderError({:?})", self)
        }
    }

    impl std::error::Error for Error {}
}


pub fn decode(input: &[u8]) -> Result<Vec<BerTlv>, error::Error> {
    let mut ber_tlv_vec = Vec::new();
    let mut input = input;

    while !input.is_empty() {
        let (tlv, consumed) = decode_one(input)?;
        ber_tlv_vec.push(tlv);
        input = &input[consumed..];
    }

    Ok(ber_tlv_vec)
}

/// AI generated slop.
fn decode_one(input: &[u8]) -> Result<(BerTlv, usize), error::Error> {
    let mut offset = 0;

    // 1. Parse tag (1–N bytes)
    let first_tag_byte = *input
        .get(offset)
        .ok_or(error::Error::UnexpectedEof)?;
    offset += 1;

    let mut tag_bytes = vec![first_tag_byte];

    // High-tag-number form if bits 5–1 are all ones (0x1F)
    if (first_tag_byte & 0x1F) == 0x1F {
        loop {
            let b = *input
                .get(offset)
                .ok_or(error::Error::UnexpectedEof)?;
            offset += 1;
            tag_bytes.push(b);

            // Last tag byte has bit 8 = 0
            if (b & 0x80) == 0 {
                break;
            }
        }
    }

    // Turn tag bytes into u32 like 0x9F02, 0x5A, etc.
    let mut tag: u32 = 0;
    for b in &tag_bytes {
        tag = (tag << 8) | (*b as u32);
    }

    // 2. Parse length (short or long definite)
    let first_len_byte = *input
        .get(offset)
        .ok_or(error::Error::UnexpectedEof)?;
    offset += 1;

    let length: usize;

    if (first_len_byte & 0x80) == 0 {
        // Short form: length is in the low 7 bits (0..127)
        length = first_len_byte as usize;
    } else {
        // Long form: low 7 bits indicate number of subsequent length bytes
        let num_len_bytes = (first_len_byte & 0x7F) as usize;

        if num_len_bytes == 0 {
            // 0x80 => indefinite length, which we don't support for EMV
            return Err(error::Error::IndefiniteLengthUnsupported);
        }

        if offset + num_len_bytes > input.len() {
            return Err(error::Error::UnexpectedEof);
        }

        let mut len_val: usize = 0;
        for _ in 0..num_len_bytes {
            let b = input[offset];
            offset += 1;
            len_val = (len_val << 8) | (b as usize);
        }

        length = len_val;
    }

    if offset + length > input.len() {
        return Err(error::Error::UnexpectedEof);
    }

    let value_bytes = &input[offset .. offset + length];
    offset += length;

    // 3. Determine primitive vs constructed
    // Constructed bit is bit 6 of the first tag byte (0x20).
    let is_constructed = (first_tag_byte & 0x20) != 0;

    let value = if is_constructed {
        // Parse nested TLVs recursively inside value_bytes
        let mut children = Vec::new();
        let mut inner_off = 0usize;

        while inner_off < value_bytes.len() {
            let (child, consumed) = decode_one(&value_bytes[inner_off..])?;
            children.push(child);
            inner_off += consumed;
        }

        BerValue::Constructed(children)
    } else {
        BerValue::Primitive(value_bytes.to_vec())
    };

    Ok((
        BerTlv {
            tag,
            length,
            value,
        },
        offset,
    ))
}
