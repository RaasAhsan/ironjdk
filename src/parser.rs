
const MAGIC_NUMBER: u32 = 0xCAFEBABE;

#[derive(Debug)]
pub enum ParserError {
    NoMoreBytes,
    InvalidMagic(u32)
}

pub fn parse_class_file(buffer: &mut Vec<u8>) -> Result<(), ParserError> {
    let magic = parse_magic(buffer)?;
    let minor_version = parse_u16(buffer)?;
    let major_version = parse_u16(buffer)?;
    let constant_pool_count = parse_u16(buffer)?;

    println!("Magic: {:X}", magic);
    println!("Minor version: {}", minor_version);
    println!("Major version: {}", major_version);
    println!("Constant pool count: {}", constant_pool_count);

    Ok(())
}

pub fn parse_magic(buffer: &mut Vec<u8>) -> Result<u32, ParserError> {
    let magic = parse_u32(buffer)?;

    if magic == MAGIC_NUMBER {
        Ok(magic)
    } else {
        Err(ParserError::InvalidMagic(magic))
    }
}

fn parse_u8(buffer: &mut Vec<u8>) -> Result<u8, ParserError> {
    match buffer.get(0) {
        Some(&byte) => {
            buffer.remove(0);
            Ok(byte)
        },
        None => Err(ParserError::NoMoreBytes)
    }
}

fn parse_u16(buffer: &mut Vec<u8>) -> Result<u16, ParserError> {
    let b1 = parse_u8(buffer)? as u16;
    let b2 = parse_u8(buffer)? as u16;

    Ok((b1 << 8) + b2)
}

fn parse_u32(buffer: &mut Vec<u8>) -> Result<u32, ParserError> {
    let b1 = parse_u8(buffer)? as u32;
    let b2 = parse_u8(buffer)? as u32;
    let b3 = parse_u8(buffer)? as u32;
    let b4 = parse_u8(buffer)? as u32;

    Ok((b1 << 24) + (b2 << 16) + (b3 << 8) + b4)
}
