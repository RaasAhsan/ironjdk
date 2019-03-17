use class::ConstantPool;
use class::ConstantPoolEntry;
use class::Field;
use class::Attribute;
use class::Method;
use class::ExceptionTableEntry;
use class::LineNumberTableEntry;
use class::ClassFile;
use class::StackMapFrame;
use class::VerificationTypeInfo;
use class::InnerClassTableEntry;
use class::Annotation;
use class::AnnotationElementPair;
use class::AnnotationElementValue;

const MAGIC_NUMBER: u32 = 0xCAFEBABE;

const CONSTANT_METHODREF: u8 = 10;
const CONSTANT_FIELDREF: u8 = 9;
const CONSTANT_CLASS: u8 = 7;
const CONSTANT_UTF8: u8 = 1;
const CONSTANT_NAME_AND_TYPE: u8 = 12;
const CONSTANT_STRING: u8 = 8;
const CONSTANT_INTEGER: u8 = 3;
const CONSTANT_INTERFACE_METHODREF: u8 = 11;
const CONSTANT_LONG: u8 = 5;
const CONSTANT_FLOAT: u8 = 4;

const ATTRIBUTE_CODE: &str = "Code";
const ATTRIBUTE_SOURCE_FILE: &str = "SourceFile";
const ATTRIBUTE_LINE_NUMBER_TABLE: &str = "LineNumberTable";
const ATTRIBUTE_SIGNATURE: &str = "Signature";
const ATTRIBUTE_STACK_MAP_TABLE: &str = "StackMapTable";
const ATTRIBUTE_EXCEPTIONS: &str = "Exceptions";
const ATTRIBUTE_CONSTANT_VALUE: &str = "ConstantValue";
const ATTRIBUTE_INNER_CLASSES: &str = "InnerClasses";
const ATTRIBUTE_DEPRECATED: &str = "Deprecated";
const ATTRIBUTE_RUNTIME_VISIBLE_ANNOTATIONS: &str = "RuntimeVisibleAnnotations";

#[derive(Debug)]
pub enum ClassReaderError {
    EndOfStream,
    InvalidMagic(u32),
    InvalidConstantTag(u8),
    InvalidUtf8,
    RemainingBytes,
    ExpectedAttributeName,
    InvalidAttributeName(String),
    InvalidStackMapFrame(u8),
    InvalidVerificationTypeInfo(u8),
    InvalidAnnotationElementValue(char)
}

pub fn read_class_file(buffer: &mut Vec<u8>) -> Result<ClassFile, ClassReaderError> {
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
        Err(ClassReaderError::RemainingBytes)
    }
}

fn read_magic(buffer: &mut Vec<u8>) -> Result<u32, ClassReaderError> {
    let magic = read_u32(buffer)?;

    if magic == MAGIC_NUMBER {
        Ok(magic)
    } else {
        Err(ClassReaderError::InvalidMagic(magic))
    }
}

fn read_constant_pool_entries(buffer: &mut Vec<u8>, length: u16) -> Result<Vec<ConstantPoolEntry>, ClassReaderError> {
    let mut entries: Vec<ConstantPoolEntry> = Vec::new();

    let mut index = 0;
    loop {
        let entry = read_constant_pool_entry(buffer)?;
        entries.push(entry.clone());

        // All 8-byte constants (longs and doubles) consume two entries in the constant pool table.
        // Therefore we must increment the counter twice once we see a long or double.
        match entry {
            ConstantPoolEntry::Long { .. } => {
                entries.push(ConstantPoolEntry::Placeholder);
                index += 2;
            },
            _ => {
                index += 1;
            }
        }

        if index == length {
            break;
        }
    }

    Ok(entries)
}

fn read_constant_pool_entry(buffer: &mut Vec<u8>) -> Result<ConstantPoolEntry, ClassReaderError> {
    let tag = read_u8(buffer)?;

    match tag {
        CONSTANT_FIELDREF => {
            let class_index = read_u16(buffer)?;
            let name_and_type_index = read_u16(buffer)?;

            Ok(ConstantPoolEntry::Fieldref { class_index, name_and_type_index })
        },
        CONSTANT_METHODREF => {
            let class_index = read_u16(buffer)?;
            let name_and_type_index = read_u16(buffer)?;

            Ok(ConstantPoolEntry::Methodref { class_index, name_and_type_index })
        },
        CONSTANT_INTERFACE_METHODREF => {
            let class_index = read_u16(buffer)?;
            let name_and_type_index = read_u16(buffer)?;

            Ok(ConstantPoolEntry::InterfaceMethodref { class_index, name_and_type_index })
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
        CONSTANT_STRING => {
            let string_index = read_u16(buffer)?;

            Ok(ConstantPoolEntry::String { string_index })
        },
        CONSTANT_INTEGER => {
            let bytes = read_u32(buffer)?;

            Ok(ConstantPoolEntry::Integer { bytes })
        },
        CONSTANT_FLOAT => {
            let bytes = read_u32(buffer)?;

            Ok(ConstantPoolEntry::Float { bytes })
        },
        CONSTANT_LONG => {
            let high_bytes = read_u32(buffer)?;
            let low_bytes = read_u32(buffer)?;

            Ok(ConstantPoolEntry::Long { high_bytes, low_bytes })
        },
        x => Err(ClassReaderError::InvalidConstantTag(x))
    }
}

fn read_fields(buffer: &mut Vec<u8>, length: u16, cp: &ConstantPool) -> Result<Vec<Field>, ClassReaderError> {
    let mut entries: Vec<Field> = Vec::new();

    for index in 0..length {
        let entry = read_field(buffer, cp)?;
        entries.push(entry)
    }

    Ok(entries)
}

fn read_field(buffer: &mut Vec<u8>, cp: &ConstantPool) -> Result<Field, ClassReaderError> {
    let access_flags = read_u16(buffer)?;
    let name_index = read_u16(buffer)?;
    let descriptor_index = read_u16(buffer)?;
    let attributes_count = read_u16(buffer)?;
    let attributes = read_attributes(buffer, attributes_count, cp)?;

    let field = Field { access_flags, name_index, descriptor_index, attributes };

    Ok(field)
}

fn read_methods(buffer: &mut Vec<u8>, length: u16, cp: &ConstantPool) -> Result<Vec<Method>, ClassReaderError> {
    let mut entries: Vec<Method> = Vec::new();

    for index in 0..length {
        let entry = read_method(buffer, cp)?;
        entries.push(entry)
    }

    Ok(entries)
}

fn read_method(buffer: &mut Vec<u8>, cp: &ConstantPool) -> Result<Method, ClassReaderError> {
    let access_flags = read_u16(buffer)?;
    let name_index = read_u16(buffer)?;
    let descriptor_index = read_u16(buffer)?;
    let attributes_count = read_u16(buffer)?;
    let attributes = read_attributes(buffer, attributes_count, cp)?;

    let method = Method { access_flags, name_index, descriptor_index, attributes };

    Ok(method)
}

//
// Read attributes
//

fn read_attributes(buffer: &mut Vec<u8>, length: u16, cp: &ConstantPool) -> Result<Vec<Attribute>, ClassReaderError> {
    let mut entries: Vec<Attribute> = Vec::new();

    for index in 0..length {
        let entry = read_attribute(buffer, cp)?;
        entries.push(entry)
    }

    Ok(entries)
}

fn read_attribute(buffer: &mut Vec<u8>, cp: &ConstantPool) -> Result<Attribute, ClassReaderError> {
    let attribute_name_index = read_u16(buffer)?;
    let attribute_length = read_u32(buffer)?;
    let ref mut attribute_buffer = read_bytes(buffer, attribute_length as usize)?;

    let attribute_name = cp.get_utf8(attribute_name_index)
        .map_err(|x| ClassReaderError::ExpectedAttributeName)?;

    let attribute_option = match attribute_name.as_ref() {
        ATTRIBUTE_CODE => {
            let max_stack = read_u16(attribute_buffer)?;
            let max_locals = read_u16(attribute_buffer)?;
            let code_length = read_u32(attribute_buffer)?;
            let code = read_bytes(attribute_buffer, code_length as usize)?;
            let exception_table_length = read_u16(attribute_buffer)?;
            let exceptions: Vec<ExceptionTableEntry> = read_exception_table_entries(attribute_buffer, exception_table_length)?;
            let attributes_count = read_u16(attribute_buffer)?;
            let attributes = read_attributes(attribute_buffer, attributes_count, cp)?;

            Some(Attribute::Code { max_stack, max_locals, code, exceptions, attributes })
        },
        ATTRIBUTE_STACK_MAP_TABLE => {
            let number_of_entries = read_u16(attribute_buffer)?;
            let entries = read_stack_map_frames(attribute_buffer, number_of_entries)?;

            Some(Attribute::StackMapTable { entries })
        },
        ATTRIBUTE_LINE_NUMBER_TABLE => {
            let line_number_table_length = read_u16(attribute_buffer)?;
            let line_number_table_entries = read_line_number_table_entries(attribute_buffer, line_number_table_length)?;

            Some(Attribute::LineNumberTable(line_number_table_entries))
        },
        ATTRIBUTE_SOURCE_FILE => {
            let index = read_u16(attribute_buffer)?;

            Some(Attribute::SourceFile { index })
        },
        ATTRIBUTE_SIGNATURE => {
            let index = read_u16(attribute_buffer)?;

            Some(Attribute::Signature { index })
        },
        ATTRIBUTE_EXCEPTIONS => {
            let number_of_exceptions = read_u16(attribute_buffer)?;
            let exception_index = read_u16_array(attribute_buffer, number_of_exceptions)?;

            Some(Attribute::Exceptions { exception_index })
        },
        ATTRIBUTE_CONSTANT_VALUE => {
            let index = read_u16(attribute_buffer)?;

            Some(Attribute::ConstantValue { index })
        },
        ATTRIBUTE_INNER_CLASSES => {
            let number_of_classes = read_u16(attribute_buffer)?;
            let classes = read_inner_class_entries(attribute_buffer, number_of_classes)?;

            Some(Attribute::InnerClasses { classes })
        },
        ATTRIBUTE_DEPRECATED => Some(Attribute::Deprecated),
        ATTRIBUTE_RUNTIME_VISIBLE_ANNOTATIONS => {
            let count = read_u16(attribute_buffer)?;
            let annotations = read_annotations(attribute_buffer, count)?;

            Some(Attribute::RuntimeVisibleAnnotations { annotations })
        },
        _ => None
    };

    match attribute_option {
        Some(attribute) => {
            if attribute_buffer.len() > 0 {
                println!("Failed to parse attribute {}", attribute_name);
                Err(ClassReaderError::RemainingBytes)
            } else {
                Ok(attribute)
            }
        },
        None => Err(ClassReaderError::InvalidAttributeName(attribute_name))
    }
}

fn read_annotations(buffer: &mut Vec<u8>, length: u16) -> Result<Vec<Annotation>, ClassReaderError> {
    let mut entries: Vec<Annotation> = Vec::new();

    for index in 0..length {
        let entry = read_annotation(buffer)?;
        entries.push(entry);
    }

    Ok(entries)
}

fn read_annotation(buffer: &mut Vec<u8>) -> Result<Annotation, ClassReaderError> {
    let type_index = read_u16(buffer)?;
    let length = read_u16(buffer)?;
    let elements = read_annotation_element_pairs(buffer, length)?;

    Ok(Annotation { type_index, elements })
}

fn read_annotation_element_values(buffer: &mut Vec<u8>, length: u16) -> Result<Vec<AnnotationElementValue>, ClassReaderError> {
    let mut entries: Vec<AnnotationElementValue> = Vec::new();

    for index in 0..length {
        let entry = read_annotation_element_value(buffer)?;
        entries.push(entry);
    }

    Ok(entries)
}

fn read_annotation_element_value(buffer: &mut Vec<u8>) -> Result<AnnotationElementValue, ClassReaderError> {
    let tag = read_u8(buffer)? as char;

    match tag {
        'B' | 'C' | 'D' | 'F' | 'I' | 'J' | 'S' | 'Z' | 's' => {
            let value = read_u16(buffer)?;
            Ok(AnnotationElementValue::Const(value))
        },
        'e' => {
            let type_name_index = read_u16(buffer)?;
            let const_name_index = read_u16(buffer)?;
            Ok(AnnotationElementValue::EnumConst { type_name_index, const_name_index })
        },
        'c' => {
            let class_info_index = read_u16(buffer)?;
            Ok(AnnotationElementValue::ClassInfo(class_info_index))
        },
        '@' => {
            let annotation = read_annotation(buffer)?;
            Ok(AnnotationElementValue::Annotation(annotation))
        },
        '[' => {
            let num_values = read_u16(buffer)?;
            let values = read_annotation_element_values(buffer, num_values)?;
            Ok(AnnotationElementValue::Array(values))
        },
        _ => Err(ClassReaderError::InvalidAnnotationElementValue(tag))
    }
}

fn read_annotation_element_pairs(buffer: &mut Vec<u8>, length: u16) -> Result<Vec<AnnotationElementPair>, ClassReaderError> {
    let mut entries: Vec<AnnotationElementPair> = Vec::new();

    for index in 0..length {
        let entry = read_annotation_element_pair(buffer)?;
        entries.push(entry);
    }

    Ok(entries)
}

fn read_annotation_element_pair(buffer: &mut Vec<u8>) -> Result<AnnotationElementPair, ClassReaderError> {
    let element_name_index = read_u16(buffer)?;
    let element_value = read_annotation_element_value(buffer)?;

    Ok(AnnotationElementPair { element_name_index, element_value })
}

fn read_inner_class_entries(buffer: &mut Vec<u8>, length: u16) -> Result<Vec<InnerClassTableEntry>, ClassReaderError> {
    let mut entries: Vec<InnerClassTableEntry> = Vec::new();

    for index in 0..length {
        let entry = read_inner_class_entry(buffer)?;
        entries.push(entry);
    }

    Ok(entries)
}

fn read_inner_class_entry(buffer: &mut Vec<u8>) -> Result<InnerClassTableEntry, ClassReaderError> {
    let inner_class_info_index = read_u16(buffer)?;
    let outer_class_info_index = read_u16(buffer)?;
    let inner_name_index = read_u16(buffer)?;
    let inner_class_access_flags = read_u16(buffer)?;

    Ok(InnerClassTableEntry {
        inner_class_info_index,
        outer_class_info_index,
        inner_name_index,
        inner_class_access_flags
    })
}

fn read_exception_table_entries(buffer: &mut Vec<u8>, length: u16) -> Result<Vec<ExceptionTableEntry>, ClassReaderError> {
    let mut entries: Vec<ExceptionTableEntry> = Vec::new();

    for index in 0..length {
        let entry = read_exception_table_entry(buffer)?;
        entries.push(entry);
    }

    Ok(entries)
}

fn read_exception_table_entry(buffer: &mut Vec<u8>) -> Result<ExceptionTableEntry, ClassReaderError> {
    let start_pc = read_u16(buffer)?;
    let end_pc = read_u16(buffer)?;
    let handler_pc = read_u16(buffer)?;
    let catch_type = read_u16(buffer)?;

    Ok(ExceptionTableEntry { start_pc, end_pc, handler_pc, catch_type })
}

fn read_stack_map_frames(buffer: &mut Vec<u8>, length: u16) -> Result<Vec<StackMapFrame>, ClassReaderError> {
    let mut entries: Vec<StackMapFrame> = Vec::new();

    for index in 0..length {
        let entry = read_stack_map_frame(buffer)?;
        entries.push(entry);
    }

    Ok(entries)
}

fn read_stack_map_frame(buffer: &mut Vec<u8>) -> Result<StackMapFrame, ClassReaderError> {
    let frame_type = read_u8(buffer)?;

    match frame_type {
        0 ... 63 => Ok(StackMapFrame::SameFrame),
        64 ... 127 => {
            let info = read_verification_type_info(buffer)?;
            Ok(StackMapFrame::SameLocals1StackItemFrame { info })
        },
        247 => {
            let info = read_verification_type_info(buffer)?;
            Ok(StackMapFrame::SameLocals1StackItemFrameExtended { info })
        },
        248 ... 250 => {
            let offset_delta = read_u16(buffer)?;
            Ok(StackMapFrame::ChopFrame { offset_delta })
        },
        251 => {
            let offset_delta = read_u16(buffer)?;
            Ok(StackMapFrame::SameFrameExtended { offset_delta })
        },
        x @ 252 ... 254 => {
            let offset_delta = read_u16(buffer)?;
            let locals = read_verification_type_infos(buffer, (x - 251) as u16)?;
            Ok(StackMapFrame::AppendFrame { offset_delta, locals })
        },
        255 => {
            let offset_delta = read_u16(buffer)?;
            let number_of_locals = read_u16(buffer)?;
            let locals = read_verification_type_infos(buffer, number_of_locals)?;
            let number_of_stack_items = read_u16(buffer)?;
            let stack = read_verification_type_infos(buffer, number_of_stack_items)?;
            Ok(StackMapFrame::FullFrame { offset_delta, locals, stack })
        },
        _ => Err(ClassReaderError::InvalidStackMapFrame(frame_type))
    }
}

fn read_verification_type_infos(buffer: &mut Vec<u8>, length: u16) -> Result<Vec<VerificationTypeInfo>, ClassReaderError> {
    let mut entries: Vec<VerificationTypeInfo> = Vec::new();

    for index in 0..length {
        let entry = read_verification_type_info(buffer)?;
        entries.push(entry);
    }

    Ok(entries)
}

fn read_verification_type_info(buffer: &mut Vec<u8>) -> Result<VerificationTypeInfo, ClassReaderError> {
    let tag = read_u8(buffer)?;

    match tag {
        0 => Ok(VerificationTypeInfo::Top),
        1 => Ok(VerificationTypeInfo::Integer),
        2 => Ok(VerificationTypeInfo::Float),
        3 => Ok(VerificationTypeInfo::Double),
        4 => Ok(VerificationTypeInfo::Long),
        5 => Ok(VerificationTypeInfo::Null),
        6 => Ok(VerificationTypeInfo::UninitializedThis),
        7 => {
            let cpool_index = read_u16(buffer)?;
            Ok(VerificationTypeInfo::Object(cpool_index))
        },
        8 => {
            let offset = read_u16(buffer)?;
            Ok(VerificationTypeInfo::Uninitialized(offset))
        },
        x => Err(ClassReaderError::InvalidVerificationTypeInfo(x))
    }
}

fn read_line_number_table_entries(buffer: &mut Vec<u8>, length: u16) -> Result<Vec<LineNumberTableEntry>, ClassReaderError> {
    let mut entries: Vec<LineNumberTableEntry> = Vec::new();

    for index in 0..length {
        let entry = read_line_number_table_entry(buffer)?;
        entries.push(entry);
    }

    Ok(entries)
}

fn read_line_number_table_entry(buffer: &mut Vec<u8>) -> Result<LineNumberTableEntry, ClassReaderError> {
    let start_pc = read_u16(buffer)?;
    let line_number = read_u16(buffer)?;

    Ok(LineNumberTableEntry { start_pc, line_number })
}

fn read_u8(buffer: &mut Vec<u8>) -> Result<u8, ClassReaderError> {
    match buffer.get(0) {
        Some(&byte) => {
            buffer.remove(0);
            Ok(byte)
        },
        None => Err(ClassReaderError::EndOfStream)
    }
}

fn read_u16(buffer: &mut Vec<u8>) -> Result<u16, ClassReaderError> {
    let b1 = read_u8(buffer)? as u16;
    let b2 = read_u8(buffer)? as u16;

    Ok((b1 << 8) + b2)
}

fn read_u16_array(buffer: &mut Vec<u8>, length: u16) -> Result<Vec<u16>, ClassReaderError> {
    let mut entries: Vec<u16> = Vec::new();

    for index in 0..length {
        let entry = read_u16(buffer)?;
        entries.push(entry);
    }

    Ok(entries)
}

fn read_u32(buffer: &mut Vec<u8>) -> Result<u32, ClassReaderError> {
    let b1 = read_u8(buffer)? as u32;
    let b2 = read_u8(buffer)? as u32;
    let b3 = read_u8(buffer)? as u32;
    let b4 = read_u8(buffer)? as u32;

    Ok((b1 << 24) + (b2 << 16) + (b3 << 8) + b4)
}

fn read_utf8(buffer: &mut Vec<u8>, length: usize) -> Result<String, ClassReaderError> {
    let bytes = read_bytes(buffer, length)?;
    let string = String::from_utf8(bytes)
        .map_err(|x| ClassReaderError::InvalidUtf8)?;

    Ok(string)
}

fn read_bytes(buffer: &mut Vec<u8>, length: usize) -> Result<Vec<u8>, ClassReaderError> {
    if buffer.len() < length {
        panic!("Buffer too small.");
        Err(ClassReaderError::EndOfStream)
    } else {
        let mut bytes: Vec<u8> = Vec::new();

        for index in 0..length {
            let b = read_u8(buffer)?;
            bytes.push(b);
        }

        Ok(bytes)
    }
}
