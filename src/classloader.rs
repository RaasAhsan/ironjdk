use class::ConstantPool;
use class::ConstantPoolEntry;
use class::Field;
use class::Attribute;
use class::Method;
use class::ExceptionTableEntry;
use class::LineNumberTableEntry;
use class::ClassFile;

const MAGIC_NUMBER: u32 = 0xCAFEBABE;

const CONSTANT_METHODREF: u8 = 10;
const CONSTANT_FIELDREF: u8 = 9;
const CONSTANT_CLASS: u8 = 7;
const CONSTANT_UTF8: u8 = 1;
const CONSTANT_NAME_AND_TYPE: u8 = 12;

const ATTRIBUTE_CODE: &str = "Code";
const ATTRIBUTE_SOURCE_FILE: &str = "SourceFile";
const ATTRIBUTE_LINE_NUMBER_TABLE: &str = "LineNumberTable";

#[derive(Debug)]
pub enum ClassLoaderError {
    EndOfStream,
    InvalidMagic(u32),
    InvalidConstantTag(u8),
    InvalidUtf8,
    RemainingBytes,
    ExpectedAttributeName,
    InvalidAttributeName(String)
}

pub fn read_class_file(buffer: &mut Vec<u8>) -> Result<ClassFile, ClassLoaderError> {
    let magic = read_magic(buffer)?;
    let minor_version = read_u16(buffer)?;
    let major_version = read_u16(buffer)?;
    let cp_count = read_u16(buffer)?;
    let cp_entries = read_constant_pool_entries(buffer, cp_count - 1)?;
    let constant_pool = ConstantPool { entries: cp_entries };
    let access_flags = read_u16(buffer)?;
    let this_class = read_u16(buffer)?;
    let super_class = read_u16(buffer)?;
    let interfaces_count = read_u16(buffer)?;
    let interfaces = read_u16_array(buffer, interfaces_count)?;
    let fields_count = read_u16(buffer)?;
    let fields = read_fields(buffer, fields_count, &constant_pool)?;
    let methods_count = read_u16(buffer)?;
    let methods = read_methods(buffer, methods_count, &constant_pool)?;
    let attributes_count = read_u16(buffer)?;
    let attributes = read_attributes(buffer, attributes_count, &constant_pool)?;

    let class_file = ClassFile {
        magic,
        minor_version,
        major_version,
        constant_pool,
        access_flags,
        this_class,
        super_class,
        interfaces,
        fields,
        methods,
        attributes
    };

    if buffer.len() == 0 {
        Ok(class_file)
    } else {
        Err(ClassLoaderError::RemainingBytes)
    }
}

fn read_magic(buffer: &mut Vec<u8>) -> Result<u32, ClassLoaderError> {
    let magic = read_u32(buffer)?;

    if magic == MAGIC_NUMBER {
        Ok(magic)
    } else {
        Err(ClassLoaderError::InvalidMagic(magic))
    }
}

fn read_constant_pool_entries(buffer: &mut Vec<u8>, length: u16) -> Result<Vec<ConstantPoolEntry>, ClassLoaderError> {
    let mut entries: Vec<ConstantPoolEntry> = Vec::new();

    for index in 0..length {
        let entry = read_constant_pool_entry(buffer)?;
        entries.push(entry)
    }

    Ok(entries)
}

fn read_constant_pool_entry(buffer: &mut Vec<u8>) -> Result<ConstantPoolEntry, ClassLoaderError> {
    let tag = read_u8(buffer)?;

    match tag {
        CONSTANT_METHODREF => {
            let class_index = read_u16(buffer)?;
            let name_and_type_index = read_u16(buffer)?;

            Ok(ConstantPoolEntry::Methodref { class_index, name_and_type_index })
        },
        CONSTANT_FIELDREF => {
            let class_index = read_u16(buffer)?;
            let name_and_type_index = read_u16(buffer)?;

            Ok(ConstantPoolEntry::Fieldref { class_index, name_and_type_index })
        },
        CONSTANT_CLASS => {
            let name_index = read_u16(buffer)?;

            Ok(ConstantPoolEntry::Class { name_index })
        },
        CONSTANT_UTF8 => {
            let length = read_u16(buffer)?;
            let string = read_utf8(buffer, length as usize)?;

            Ok(ConstantPoolEntry::Utf8(string))
        },
        CONSTANT_NAME_AND_TYPE => {
            let name_index = read_u16(buffer)?;
            let descriptor_index = read_u16(buffer)?;

            Ok(ConstantPoolEntry::NameAndType { name_index, descriptor_index })
        },
        x => Err(ClassLoaderError::InvalidConstantTag(x))
    }
}

fn read_fields(buffer: &mut Vec<u8>, length: u16, cp: &ConstantPool) -> Result<Vec<Field>, ClassLoaderError> {
    let mut entries: Vec<Field> = Vec::new();

    for index in 0..length {
        let entry = read_field(buffer, cp)?;
        entries.push(entry)
    }

    Ok(entries)
}

fn read_field(buffer: &mut Vec<u8>, cp: &ConstantPool) -> Result<Field, ClassLoaderError> {
    let access_flags = read_u16(buffer)?;
    let name_index = read_u16(buffer)?;
    let descriptor_index = read_u16(buffer)?;
    let attributes_count = read_u16(buffer)?;
    let attributes = read_attributes(buffer, attributes_count, cp)?;

    let field = Field { access_flags, name_index, descriptor_index, attributes };

    Ok(field)
}

fn read_methods(buffer: &mut Vec<u8>, length: u16, cp: &ConstantPool) -> Result<Vec<Method>, ClassLoaderError> {
    let mut entries: Vec<Method> = Vec::new();

    for index in 0..length {
        let entry = read_method(buffer, cp)?;
        entries.push(entry)
    }

    Ok(entries)
}

fn read_method(buffer: &mut Vec<u8>, cp: &ConstantPool) -> Result<Method, ClassLoaderError> {
    let access_flags = read_u16(buffer)?;
    let name_index = read_u16(buffer)?;
    let descriptor_index = read_u16(buffer)?;
    let attributes_count = read_u16(buffer)?;
    let attributes = read_attributes(buffer, attributes_count, cp)?;

    let method = Method { access_flags, name_index, descriptor_index, attributes };

    Ok(method)
}

fn read_attributes(buffer: &mut Vec<u8>, length: u16, cp: &ConstantPool) -> Result<Vec<Attribute>, ClassLoaderError> {
    let mut entries: Vec<Attribute> = Vec::new();

    for index in 0..length {
        let entry = read_attribute(buffer, cp)?;
        entries.push(entry)
    }

    Ok(entries)
}

fn read_attribute(buffer: &mut Vec<u8>, cp: &ConstantPool) -> Result<Attribute, ClassLoaderError> {
    let attribute_name_index = read_u16(buffer)?;
    let attribute_length = read_u32(buffer)?;
    let ref mut attribute_buffer = read_bytes(buffer, attribute_length as usize)?;

    let attribute_name = cp.get_utf8(attribute_name_index)
        .map_err(|x| ClassLoaderError::ExpectedAttributeName)?;

    let attribute_option = match attribute_name.as_ref() {
        ATTRIBUTE_CODE => {
            let max_stack = read_u16(attribute_buffer)?;
            let max_locals = read_u16(attribute_buffer)?;
            let code_length = read_u32(attribute_buffer)?;
            let code = read_bytes(attribute_buffer, code_length as usize)?;
            let exception_table_length = read_u16(attribute_buffer)?;
            let exceptions: Vec<ExceptionTableEntry> = Vec::new();
            let attributes_count = read_u16(attribute_buffer)?;
            let attributes = read_attributes(attribute_buffer, attributes_count, cp)?;

            Some(Attribute::Code { max_stack, max_locals, code, exceptions, attributes })
        },
        ATTRIBUTE_LINE_NUMBER_TABLE => {
            let line_number_table_length = read_u16(attribute_buffer)?;
            let line_number_table_entries = read_line_number_table_entries(attribute_buffer, line_number_table_length)?;

            Some(Attribute::LineNumberTable(line_number_table_entries))
        },
        ATTRIBUTE_SOURCE_FILE => {
            let index = read_u16(attribute_buffer)?;

            Some(Attribute::SourceFile { index })
        }
        _ => None
    };

    match attribute_option {
        Some(attribute) => {
            if attribute_buffer.len() > 0 {
                println!("Failed to parse attribute {}", attribute_name);
                Err(ClassLoaderError::RemainingBytes)
            } else {
                Ok(attribute)
            }
        },
        None => Err(ClassLoaderError::InvalidAttributeName(attribute_name))
    }
}

fn read_line_number_table_entries(buffer: &mut Vec<u8>, length: u16) -> Result<Vec<LineNumberTableEntry>, ClassLoaderError> {
    let mut entries: Vec<LineNumberTableEntry> = Vec::new();

    for index in 0..length {
        let entry = read_line_number_table_entry(buffer)?;
        entries.push(entry);
    }

    Ok(entries)
}

fn read_line_number_table_entry(buffer: &mut Vec<u8>) -> Result<LineNumberTableEntry, ClassLoaderError> {
    let start_pc = read_u16(buffer)?;
    let line_number = read_u16(buffer)?;

    Ok(LineNumberTableEntry { start_pc, line_number })
}

fn read_u8(buffer: &mut Vec<u8>) -> Result<u8, ClassLoaderError> {
    match buffer.get(0) {
        Some(&byte) => {
            buffer.remove(0);
            Ok(byte)
        },
        None => Err(ClassLoaderError::EndOfStream)
    }
}

fn read_u16(buffer: &mut Vec<u8>) -> Result<u16, ClassLoaderError> {
    let b1 = read_u8(buffer)? as u16;
    let b2 = read_u8(buffer)? as u16;

    Ok((b1 << 8) + b2)
}

fn read_u16_array(buffer: &mut Vec<u8>, length: u16) -> Result<Vec<u16>, ClassLoaderError> {
    let mut entries: Vec<u16> = Vec::new();

    for index in 0..length {
        let entry = read_u16(buffer)?;
        entries.push(entry);
    }

    Ok(entries)
}

fn read_u32(buffer: &mut Vec<u8>) -> Result<u32, ClassLoaderError> {
    let b1 = read_u8(buffer)? as u32;
    let b2 = read_u8(buffer)? as u32;
    let b3 = read_u8(buffer)? as u32;
    let b4 = read_u8(buffer)? as u32;

    Ok((b1 << 24) + (b2 << 16) + (b3 << 8) + b4)
}

fn read_utf8(buffer: &mut Vec<u8>, length: usize) -> Result<String, ClassLoaderError> {
    let bytes = read_bytes(buffer, length)?;
    let string = String::from_utf8(bytes)
        .map_err(|x| ClassLoaderError::InvalidUtf8)?;

    Ok(string)
}

fn read_bytes(buffer: &mut Vec<u8>, length: usize) -> Result<Vec<u8>, ClassLoaderError> {
    if buffer.len() < length {
        Err(ClassLoaderError::EndOfStream)
    } else {
        let mut bytes: Vec<u8> = Vec::new();

        for index in 0..length {
            let b = read_u8(buffer)?;
            bytes.push(b);
        }

        Ok(bytes)
    }
}
