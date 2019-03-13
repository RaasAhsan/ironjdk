
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
    attributes: Vector<Attribute>
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

// High
pub enum ConstantPoolEntry {
    Fieldref { class_index: u16, name_and_type_index: u16 },
    Methodref { class_index: u16, name_and_type_index: u16 },
    InterfaceMethodref { class_index: u16, name_and_type_index: u16 },
    String { string_index: u16 },
    Integer { bytes: u32 },
    Float { bytes: u32 },
    Long { high_bytes: u32, low_bytes: u32 },
    Double { high_bytes: u32, low_bytes: u32 },
    NameAndType { name_index: u16, descriptor_index: u16 },
    Utf8 { length: u16, bytes: Vec<u8> },
    MethodHandle { reference_kind: u8, reference_index: u16 },
    MethodType { descriptor_index: u16 },
    InvokeDynamic { bootstrap_method_attr_index: u16, name_and_type_index: u16 }
}

pub struct Field {
    access_flags: u16,
    name_index: u16,
    descriptor_index: u16,
    attributes: Vec<Attribute>
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

pub struct Method {
    access_flags: u16,
    name_index: u16,
    descriptor_index: u16,
    attributes: Vec<Attribute>
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

pub struct AttributeInfo {
    attribute_name_index: u16,
    attribute_length: u32,
    info: Vec<u8>
}

pub enum Attribute {
    ConstantValue { constantvalue_index: u16 },
    Code {
        max_stack: u16,
        max_locals: u16,
        code_length: u32,
        code: Vec<u1>,
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

pub struct ExceptionTableEntry {
    start_pc: u16,
    end_pc: u16,
    handler_pc: u16,
    catch_type: u16
}

pub struct InnerClassTableEntry {
    inner_class_info_index: u16,
    outer_class_info_index: u16,
    inner_name_index: u16,
    inner_class_access_flags: u16
}

pub struct LineNumberTableEntry {
    start_pc: u16,
    line_number: u16
}
