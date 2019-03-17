
// Low-level representations of a ClassFile

use disassembler::disassemble_code;

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

    pub fn debug(&self) -> () {
        println!("Magic: {:X}", self.magic);
        println!("Minor version: {}", self.minor_version);
        println!("Major version: {}", self.major_version);
        println!("{:#?}", self.constant_pool);
        println!("Access flags: {:#04X}", self.access_flags);
        println!("This class: {:?}", self.constant_pool.get(self.this_class - 1));
        println!("Super class: {:?}", self.constant_pool.get(self.super_class - 1));
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
                                let disassemble_result = disassemble_code(&mut code_buffer);

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
    pub attributes: Vec<Attribute>
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
    pub attributes: Vec<Attribute>
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
        code: Vec<u8>,
        exceptions: Vec<ExceptionTableEntry>,
        attributes: Vec<Attribute>
    },
    StackMapTable {},
    Exceptions { exceptions: Vec<u16> },
    InnerClasses { classes: Vec<InnerClassTableEntry> },
    EnclosingMethod {},
    Synthetic {},
    SourceFile { index: u16 },
    SourceDebugExtension {},
    LineNumberTable(Vec<LineNumberTableEntry>),
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
    pub start_pc: u16,
    pub line_number: u16
}
