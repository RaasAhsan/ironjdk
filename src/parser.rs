use class::ConstantPoolEntry;

const MAGIC_NUMBER: u32 = 0xCAFEBABE;

const CONSTANT_METHODREF: u8 = 10;
const CONSTANT_FIELDREF: u8 = 9;
const CONSTANT_CLASS: u8 = 7;
const CONSTANT_UTF8: u8 = 1;
const CONSTANT_NAME_AND_TYPE: u8 = 12;


#[derive(Debug)]
pub enum ParserError {
    NoMoreBytes,
    InvalidMagic(u32),
    InvalidConstantTag(u8),
    InvalidUtf8
}

pub fn parse_class_file(buffer: &mut Vec<u8>) -> Result<(), ParserError> {
    let magic = parse_magic(buffer)?;
    let minor_version = parse_u16(buffer)?;
    let major_version = parse_u16(buffer)?;
    let cp_count = parse_u16(buffer)?;
    let cp_entries = parse_constant_pool_entries(buffer, cp_count - 1)?;
    let access_flags = parse_u16(buffer)?;
    let this_class = parse_u16(buffer)?;
    let super_class = parse_u16(buffer)?;
    let interfaces_count = parse_u16(buffer)?;
//    let interfaces =
    let fields_count = parse_u16(buffer)?;

    println!("Magic: {:X}", magic);
    println!("Minor version: {}", minor_version);
    println!("Major version: {}", major_version);
    println!("Constant pool count: {}", cp_count);
    println!("Constant pool entries count: {}", cp_entries.len());
    println!("{:?}", cp_entries);
    println!("Access flags: {:#04X}", access_flags);
    println!("This class: {:?}", cp_entries.get((this_class - 1) as usize));
    println!("Super class: {:?}", cp_entries.get((super_class - 1) as usize));
    println!("Interfaces count: {}", interfaces_count);
    println!("Fields count: {}", fields_count);

    Ok(())
}

fn parse_constant_pool_entries(buffer: &mut Vec<u8>, length: u16) -> Result<Vec<ConstantPoolEntry>, ParserError> {
    let mut entries: Vec<ConstantPoolEntry> = Vec::new();

    for index in 0..length {
        let entry = parse_constant_pool_entry(buffer)?;
        entries.push(entry)
    }

    Ok(entries)
}

fn parse_constant_pool_entry(buffer: &mut Vec<u8>) -> Result<ConstantPoolEntry, ParserError> {
    let tag = parse_u8(buffer)?;

    match tag {
        CONSTANT_METHODREF => {
            let class_index = parse_u16(buffer)?;
            let name_and_type_index = parse_u16(buffer)?;

            Ok(ConstantPoolEntry::Methodref { class_index, name_and_type_index })
        },
        CONSTANT_FIELDREF => {
            let class_index = parse_u16(buffer)?;
            let name_and_type_index = parse_u16(buffer)?;

            Ok(ConstantPoolEntry::Fieldref { class_index, name_and_type_index })
        },
        CONSTANT_CLASS => {
            let name_index = parse_u16(buffer)?;

            Ok(ConstantPoolEntry::Class { name_index })
        },
        CONSTANT_UTF8 => {
            let length = parse_u16(buffer)?;
            let string = parse_utf8(buffer, length as usize)?;

            Ok(ConstantPoolEntry::Utf8(string))
        },
        CONSTANT_NAME_AND_TYPE => {
            let name_index = parse_u16(buffer)?;
            let descriptor_index = parse_u16(buffer)?;

            Ok(ConstantPoolEntry::NameAndType { name_index, descriptor_index })
        },
        x => Err(ParserError::InvalidConstantTag(x))
    }
}

fn parse_magic(buffer: &mut Vec<u8>) -> Result<u32, ParserError> {
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

fn parse_utf8(buffer: &mut Vec<u8>, length: usize) -> Result<String, ParserError> {
    let bytes = parse_bytes(buffer, length)?;
    let string = String::from_utf8(bytes)
        .map_err(|x| ParserError::InvalidUtf8)?;

    Ok(string)
}

fn parse_bytes(buffer: &mut Vec<u8>, length: usize) -> Result<Vec<u8>, ParserError> {
    if buffer.len() < length {
        Err(ParserError::NoMoreBytes)
    } else {
        let mut bytes: Vec<u8> = Vec::new();

        for index in 0..length {
            let b = parse_u8(buffer)?;
            bytes.push(b);
        }

        Ok(bytes)
    }
}
