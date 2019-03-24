
// Low-level representations of a ClassFile`

pub mod reader;

use code::disassembler;
use code::instruction::Instruction;
use runtime::class::RuntimeMethod;

pub mod method {
    pub const ACC_PUBLIC: u16 = 0x0001;
    pub const ACC_PRIVATE: u16 = 0x0002;
    pub const ACC_PROTECTED: u16 = 0x0004;
    pub const ACC_STATIC: u16 = 0x0008;
    pub const ACC_FINAL: u16 = 0x0010;
    pub const ACC_SYNCHRONIZED: u16 = 0x0020;
    pub const ACC_BRIDGE: u16 = 0x0040;
    pub const ACC_VARARGS: u16 = 0x0080;
    pub const ACC_NATIVE: u16 = 0x0100;
    pub const ACC_ABSTRACT: u16 = 0x0200;
    pub const ACC_STRICT: u16 = 0x0400;
    pub const ACC_SYNTHETIC: u16 = 0x0800;
}

pub struct ClassFile {
    pub magic: u32,
    pub minor_version: u16,
    pub major_version: u16,
    pub constant_pool: ConstantPool,
    pub access_flags: u16,
    pub this_class: u16,
    pub super_class: u16,
    pub interfaces: Vec<u16>,
    pub fields: Vec<Field>,
    pub methods: Vec<Method>,
    pub attributes: Vec<Attribute>
}

impl ClassFile {

    pub fn is_java_lang_object(&self) -> bool {
        self.super_class == 0
    }

    pub fn print_constant_pool(&self) {
        for i in 1 ..(self.constant_pool.size()) {
            let entry = self.constant_pool.get(i as u16);

            match entry {
                Some( e) => match e {
                    &ConstantPoolEntry::Placeholder => {}
                    _ => {
                        println!("{}: {:?}", i, e);
                    }
                }
                None => {}
            }
        }
    }

    pub fn find_method(&self, name: &str, access_flags: u16) -> Option<&Method> {
        for method in self.methods.iter() {
            let name_index = method.name_index;
            let name_item = self.constant_pool.get_utf8(name_index).ok()?;

            if name_item == name && method.access_flags & access_flags == access_flags {
                return Some(&method);
            }
        }

        None
    }

    pub fn debug(&self) -> () {
        println!("Magic: {:X}", self.magic);
        println!("Minor version: {}", self.minor_version);
        println!("Major version: {}", self.major_version);
        println!("{:#?}", self.constant_pool);
        println!("Access flags: {:#04X}", self.access_flags);
        println!("This class: {:?}", self.constant_pool.get(self.this_class));
        if !self.is_java_lang_object() {
            println!("Super class: {:?}", self.constant_pool.get(self.super_class));
        }
        println!("{:#?}", self.interfaces);
        println!("{:#?}", self.fields);
        println!("{:#?}", self.methods);
        println!("{:#?}", self.attributes);

        for m in self.methods.iter() {
            let method_name = self.constant_pool.get_utf8(m.name_index);

            match method_name {
                Ok(name) => {
                    println!("Method: {}", name);
                    for a in m.attributes.iter() {
                        match a {
                            &Attribute::Code { ref code, .. } => {
                                let mut code_buffer = code.clone();
                                let disassemble_result = disassembler::disassemble_code(&mut code_buffer);

                                match disassemble_result {
                                    Ok(instructions) => {
                                        println!("Code: ");
                                        println!("{:#?}", instructions);
                                    },
                                    Err(_) => {}
                                }
                            },
                            _ => {}
                        }
                    }
                },
                Err(_) => {}
            }
        }
    }

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

#[derive(Debug)]
pub struct ConstantPool {
    pub entries: Vec<ConstantPoolEntry>
}

impl ConstantPool {

    // Logical index
    pub fn get(&self, index: u16) -> Option<&ConstantPoolEntry> {
        self.entries.get((index - 1) as usize)
    }

    pub fn size(&self) -> usize {
        self.entries.len()
    }

    pub fn get_utf8(&self, index: u16) -> Result<String, &str> {
        let elem: Option<&ConstantPoolEntry> = self.get(index);

        match elem {
            Some(e) => match e {
                &ConstantPoolEntry::Utf8(ref string) => Ok(string.clone()),
                _ => Err("Expected Utf8 attribute")
            },
            None => Err("Constant pool index out of bounds")
        }
    }

}

#[derive(Clone, Debug)]
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
    InvokeDynamic { bootstrap_method_attr_index: u16, name_and_type_index: u16 },

    Placeholder
}

#[derive(Debug)]
pub struct Field {
    pub access_flags: u16,
    pub name_index: u16,
    pub descriptor_index: u16,
    pub attributes: Vec<Attribute>
}

#[derive(Debug)]
pub struct Method {
    pub access_flags: u16,
    pub name_index: u16,
    pub descriptor_index: u16,
    pub attributes: Vec<Attribute>
}

impl Method {

    pub fn disassemble(&self) -> Option<RuntimeMethod> {
        for a in self.attributes.iter() {
            match a {
                &Attribute::Code { max_stack, max_locals, ref code, .. } => {
                    let code = disassembler::disassemble_code(&code).ok()?;

                    return Some(RuntimeMethod { max_stack, max_locals, code })
                },
                _ => {}
            }
        }

        None
    }

}

#[derive(Debug)]
pub struct AttributeInfo {
    pub attribute_name_index: u16,
    pub bytes: Vec<u8>
}

#[derive(Debug)]
pub enum Attribute {
    ConstantValue { index: u16 },
    Code {
        max_stack: u16,
        max_locals: u16,
        code: Vec<u8>,
        exceptions: Vec<ExceptionTableEntry>,
        attributes: Vec<Attribute>
    },
    StackMapTable { entries: Vec<StackMapFrame> },
    Exceptions { exception_index: Vec<u16> },
    InnerClasses { classes: Vec<InnerClassTableEntry> },
    EnclosingMethod {},
    Synthetic {},
    Signature { index: u16 },
    SourceFile { index: u16 },
    SourceDebugExtension {},
    LineNumberTable(Vec<LineNumberTableEntry>),
    LocalVariableTable {},
    LocalVariableTypeTable {},
    Deprecated,
    RuntimeVisibleAnnotations { annotations: Vec<Annotation> },
    ElementValue {},
    RuntimeInvisibleAnnotations {},
    RuntimeVisibleParameterAnnotations {},
    RuntimeInvisibleParameterAnnotations {},
    AnnotationDefault {},
    BootstrapMethods {}
}

#[derive(Debug)]
pub struct Annotation {
    pub type_index: u16,
    pub elements: Vec<AnnotationElementPair>
}

#[derive(Debug)]
pub struct AnnotationElementPair {
    pub element_name_index: u16,
    pub element_value: AnnotationElementValue
}

#[derive(Debug)]
pub enum AnnotationElementValue {
    Const(u16),
    EnumConst { type_name_index: u16, const_name_index: u16 },
    ClassInfo(u16),
    Annotation(Annotation),
    Array(Vec<AnnotationElementValue>)
}

#[derive(Debug)]
pub enum StackMapFrame {
    SameFrame,
    SameLocals1StackItemFrame { info: VerificationTypeInfo },
    SameLocals1StackItemFrameExtended { info: VerificationTypeInfo },
    ChopFrame { offset_delta: u16 },
    SameFrameExtended { offset_delta: u16 },
    AppendFrame { offset_delta: u16, locals: Vec<VerificationTypeInfo> },
    FullFrame {
        offset_delta: u16,
        locals: Vec<VerificationTypeInfo>,
        stack: Vec<VerificationTypeInfo>
    }
}

#[derive(Debug)]
pub enum VerificationTypeInfo {
    Top,
    Integer,
    Float,
    Null,
    UninitializedThis,
    Object(u16),
    Uninitialized(u16),
    Long,
    Double
}

#[derive(Debug)]
pub struct ExceptionTableEntry {
    pub start_pc: u16,
    pub end_pc: u16,
    pub handler_pc: u16,
    pub catch_type: u16
}

#[derive(Debug)]
pub struct InnerClassTableEntry {
    pub inner_class_info_index: u16,
    pub outer_class_info_index: u16,
    pub inner_name_index: u16,
    pub inner_class_access_flags: u16
}

#[derive(Debug)]
pub struct LineNumberTableEntry {
    pub start_pc: u16,
    pub line_number: u16
}
