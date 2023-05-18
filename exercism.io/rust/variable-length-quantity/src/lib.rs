#[derive(Debug, PartialEq, Eq)]
pub enum Error {
    IncompleteNumber,
    Overflow,
}

/// Convert a list of numbers to a stream of bytes encoded with variable length encoding.
pub fn to_bytes(values: &[u32]) -> Vec<u8> {
    let res = values
        .iter()
        .map(|value| {
            let mut value = *value;
            let mut bytes: Vec<u8> = Vec::new();
            loop {
                let mut byte_7 = (value & 0x7F) as u8;

                value >>= 7;
                byte_7 |= 0x80;

                bytes.push(byte_7);

                if value == 0 {
                    break;
                }
            }

            bytes[0] &= 0x7F;
            bytes.reverse();

            bytes
        })
        .flatten()
        .collect::<Vec<u8>>();

    res.iter().for_each(|x| println!("{:08b}", x));
    println!("\n");

    res
}

/// Given a stream of bytes, extract all numbers which are encoded in there.
pub fn from_bytes(bytes: &[u8]) -> Result<Vec<u32>, Error> {
    let mut values: Vec<u32> = vec![];
    let mut value: u64 = 0;
    let mut completed = false;

    for byte in bytes {
        value |= (byte & 0x7F) as u64;
        completed = false;

        if value > u32::MAX as u64 {
            return Err(Error::Overflow);
        }

        if byte & 0x80 == 0 {
            values.push(value as u32);
            value = 0;
            completed = true;
        } else {
            value <<= 7;
        }
    }

    if completed {
        return Ok(values);
    } else {
        return Err(Error::IncompleteNumber);
    }
}
