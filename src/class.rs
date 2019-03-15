
// Low-level representations of a ClassFile

pub struct ClassFile {
    magic: u32,
    minor_version: u16,
    major_version: u16,
    constant_pool: Vec<ConstantPoolEntry>,
    access_flags: u16,
    this_class: u16,
    super_class: u16,
    interfaces: Vec<u16>,
    fields: Vec<Field>,
    methods: Vec<Method>,
    attributes: Vec<Attribute>
}

pub struct ConstantPoolInfo {
    tag: u8,
    info: Vec<u8>
}

pub enum ConstantPoolTag {
    Class,
    Fieldref,
    Methodref,
    InterfaceMethodref,
    String,
    Integer,
    Float,
    Long,
    Double,
    NameAndType,
    Utf8,
    MethodHandle,
    MethodType,
    InvokeDynamic
}

impl ConstantPoolTag {

}

// High
#[derive(Debug)]
pub enum ConstantPoolEntry {
    Class { name_index: u16 },
    Fieldref { class_index: u16, name_and_type_index: u16 },
    Methodref { class_index: u16, name_and_type_index: u16 },
    InterfaceMethodref { class_index: u16, name_and_type_index: u16 },
    String { string_index: u16 },
    Integer { bytes: u32 },
    Float { bytes: u32 },
    Long { high_bytes: u32, low_bytes: u32 },
    Double { high_bytes: u32, low_bytes: u32 },
    NameAndType { name_index: u16, descriptor_index: u16 },
    Utf8(String),
    MethodHandle { reference_kind: u8, reference_index: u16 },
    MethodType { descriptor_index: u16 },
    InvokeDynamic { bootstrap_method_attr_index: u16, name_and_type_index: u16 }
}

#[derive(Debug)]
pub struct Field {
    pub access_flags: u16,
    pub name_index: u16,
    pub descriptor_index: u16,
    pub attributes: Vec<AttributeInfo>
}

pub enum FieldAccessFlag {
    Public,
    Private,
    Protected,
    Static,
    Final,
    Volatile,
    Transient,
    Synthetic,
    Enum
}

impl FieldAccessFlag {

}

#[derive(Debug)]
pub struct Method {
    pub access_flags: u16,
    pub name_index: u16,
    pub descriptor_index: u16,
    pub attributes: Vec<AttributeInfo>
}

pub enum MethodAccessFlag {
    Public,
    Private,
    Protected,
    Static,
    Final,
    Synchronized,
    Bridge,
    Varargs,
    Native,
    Abstract,
    Strict,
    Synthetic
}

impl MethodAccessFlag {

}

#[derive(Debug)]
pub struct AttributeInfo {
    pub attribute_name_index: u16,
    pub bytes: Vec<u8>
}

#[derive(Debug)]
pub enum Attribute {
    ConstantValue { constantvalue_index: u16 },
    Code {
        max_stack: u16,
        max_locals: u16,
        code_length: u32,
        code: Vec<u8>,
        exceptions: Vec<ExceptionTableEntry>,
        attributes: Vec<Attribute>
    },
    StackMapTable {},
    Exceptions { exceptions: Vec<u16> },
    InnerClasses { classes: Vec<InnerClassTableEntry> },
    EnclosingMethod {},
    Synthetic {},
    SourceFile { sourcefile_index: u16 },
    SourceDebugExtension {},
    LineNumberTable { lines: Vec<LineNumberTableEntry> },
    LocalVariableTable {},
    LocalVariableTypeTable {},
    Deprecated {},
    RuntimeVisibleAnnotations {},
    ElementValue {},
    RuntimeInvisibleAnnotations {},
    RuntimeVisibleParameterAnnotations {},
    RuntimeInvisibleParameterAnnotations {},
    AnnotationDefault {},
    BootstrapMethods {}
}

#[derive(Debug)]
pub struct ExceptionTableEntry {
    start_pc: u16,
    end_pc: u16,
    handler_pc: u16,
    catch_type: u16
}

#[derive(Debug)]
pub struct InnerClassTableEntry {
    inner_class_info_index: u16,
    outer_class_info_index: u16,
    inner_name_index: u16,
    inner_class_access_flags: u16
}

#[derive(Debug)]
pub struct LineNumberTableEntry {
    start_pc: u16,
    line_number: u16
}
