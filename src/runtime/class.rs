use code::instruction::TaggedInstruction;
use class::{ClassFile, ConstantPool, Method, Field, Attribute};
use std::collections::HashMap;
use std::rc::Rc;
use code::disassembler;
use runtime::Value;

#[derive(Debug)]
pub struct ClassTable {
    classes: HashMap<String, Rc<RuntimeClass>>
}

impl ClassTable {

    pub fn load_class(&mut self, class: &Rc<RuntimeClass>) {
        self.classes.insert(class.class_name.clone(), class.clone());
    }

    pub fn get_class(&self, name: &str) -> Option<&Rc<RuntimeClass>> {
        self.classes.get(name)
    }

    pub fn new() -> ClassTable {
        ClassTable {
            classes: HashMap::new()
        }
    }

}

#[derive(Debug)]
pub struct RuntimeClass {
    pub class_name: String,
    pub constant_pool: ConstantPool,
    pub fields: Vec<RuntimeField>,
    pub methods: Vec<RuntimeMethod>
//    pub static_fields: Vec<RuntimeField> // TODO: Here or somewhere else?
}

impl RuntimeClass {

    pub fn default_fields(&self) -> Vec<Value> {
        self.fields
            .iter()
            .map(|field|
                // See JVMS $2.3 and $2.4 for default values.
                match field.descriptor {
                    FieldDescriptor::Integer => Value::Integer(0),
                    _ => Value::Null
                }
            )
            .collect()
    }

    pub fn get_method(&self, name: &str) -> Option<&RuntimeMethod> {
        self.methods
            .iter()
            .find(|method| method.name == name)
    }

    pub fn from_class_file(class_file: &ClassFile) -> Result<Rc<RuntimeClass>, String> {
        let class_name = class_file.constant_pool.get_class_name(class_file.this_class)?;
        let cp = class_file.constant_pool.clone(); // TODO: Better representation?

        let fields = class_file.fields
            .iter()
            .map(|field| RuntimeField::from_class_field(field, &cp).unwrap())
            .collect();

        let methods = class_file.methods
            .iter()
            .map(|method| RuntimeMethod::from_class_method(method, &cp).unwrap())
            .collect();

        let runtime_class = RuntimeClass {
            class_name,
            constant_pool: cp,
            fields,
            methods
        };

        Ok(Rc::new(runtime_class))
    }

}

struct RuntimeConstantPool {

}

#[derive(Debug)]
pub struct RuntimeMethod {
    pub name: String,
    pub access_flags: u16,
    pub code: Code
}

impl RuntimeMethod {

    pub fn from_class_method(method: &Method, cp: &ConstantPool) -> Option<RuntimeMethod> {
        let name = cp.get_utf8(method.name_index).unwrap();
        let code = RuntimeMethod::get_code(method)?;

        let runtime_method = RuntimeMethod {
            name,
            access_flags: method.access_flags,
            code
        };

        Some(runtime_method)
    }

    fn get_code(method: &Method) -> Option<Code> {
        for a in method.attributes.iter() {
            match a {
                &Attribute::Code { max_stack, max_locals, ref code, .. } => {
                    let instructions = disassembler::disassemble_code(&code).ok()?;

                    let code = Code {
                        max_stack,
                        max_locals,
                        instructions
                    };

                    return Some(code);
                },
                _ => {}
            }
        }

        None
    }

}

#[derive(Debug)]
pub struct Code {
    pub max_stack: u16,
    pub max_locals: u16,
    pub instructions: Vec<TaggedInstruction>
}

#[derive(Debug)]
pub struct RuntimeField {
    pub access_flags: u16,
    pub name: String,
    pub descriptor: FieldDescriptor
}

impl RuntimeField {

    pub fn from_class_field(field: &Field, cp: &ConstantPool) -> Option<RuntimeField> {
        let name = cp.get_utf8(field.name_index).unwrap();
        let descriptor_tag = cp.get_utf8(field.descriptor_index).unwrap();

        // TODO: We can do this decoding in the class representation
        let descriptor = FieldDescriptor::from_str(descriptor_tag.as_str()).unwrap();

        let runtime_field = RuntimeField {
            access_flags: field.access_flags,
            name,
            descriptor
        };

        Some(runtime_field)
    }

}

// TODO: See $4.3.2 of JVMS for a more precise encoding.
#[derive(Debug)]
pub enum FieldDescriptor {
    Byte,
    Character,
    Double,
    Float,
    Integer,
    Long,
    ClassReference { class_name: String },
    Short,
    Boolean,
    ArrayReference()
}

impl FieldDescriptor {

    pub fn from_str(s: &str) -> Option<FieldDescriptor> {
        match s {
            "B" => Some(FieldDescriptor::Byte),
            "C" => Some(FieldDescriptor::Character),
            "D" => Some(FieldDescriptor::Double),
            "F" => Some(FieldDescriptor::Float),
            "I" => Some(FieldDescriptor::Integer),
            "J" => Some(FieldDescriptor::Long),
            x if x.starts_with("L") && x.ends_with(";") => {
                let class_name = String::from(&x[1..(x.len()-1)]);
                Some(FieldDescriptor::ClassReference { class_name })
            },
            "S" => Some(FieldDescriptor::Short),
            "Z" => Some(FieldDescriptor::Boolean),
            // TODO: Parse array types
            _ => None
        }
    }

}
