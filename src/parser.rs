
const MAGIC_NUMBER: u32 = 0xCAFEBABE;

#[derive(Debug)]
pub enum ParserError {
    NoMoreBytes,
    InvalidMagic(u32)
}

pub fn parse_class_file(buffer: &mut Vec<u8>) -> Result<(), ParserError> {
    let magic = parse_magic(buffer)?;

    println!("Magic: {}", magic);

    Ok(())
}

pub fn parse_magic(buffer: &mut Vec<u8>) -> Result<u32, ParserError> {
    let b1 = parse_byte(buffer)? as u32;
    let b2 = parse_byte(buffer)? as u32;
    let b3 = parse_byte(buffer)? as u32;
    let b4 = parse_byte(buffer)? as u32;

    let magic = (b1 << 24) + (b2 << 16) + (b3 << 8) + b4;

    if magic == MAGIC_NUMBER {
        Ok(magic)
    } else {
        Err(ParserError::InvalidMagic(magic))
    }
}

pub fn parse_byte(buffer: &mut Vec<u8>) -> Result<u8, ParserError> {
    match buffer.get(0) {
        Some(&byte) => {
            buffer.remove(0);
            Ok(byte)
        },
        None => Err(ParserError::NoMoreBytes)
    }
}
