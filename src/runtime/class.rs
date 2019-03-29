use code::instruction::TaggedInstruction;
use class::{ClassFile, ConstantPool, Method, Field, Attribute};
use std::collections::HashMap;
use std::rc::Rc;
use code::disassembler;

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
    pub fields: Vec<Field>,
    pub methods: Vec<RuntimeMethod>
}

impl RuntimeClass {

    pub fn find_method(&self, name: &str, access_flags: u16) -> Option<&RuntimeMethod> {
        for method in self.methods.iter() {
            let name_index = method.name_index;
            let name_item = self.constant_pool.get_utf8(name_index).ok()?;

            if name_item == name && method.access_flags & access_flags == access_flags {
                return Some(&method);
            }
        }

        None
    }

    pub fn from_class_file(class_file: &ClassFile) -> Result<Rc<RuntimeClass>, String> {
        let class_name = class_file.constant_pool.get_class(class_file.this_class)?;
        let constant_pool = class_file.constant_pool.clone(); // TODO: Better representation?
        let fields = class_file.fields.to_vec();
        let methods = class_file.methods
            .iter()
            .map(|method| RuntimeMethod::from_class_method(method).unwrap())
            .collect();

        let runtime_class = RuntimeClass {
            class_name,
            constant_pool,
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
    pub name_index: u16,
    pub access_flags: u16,
    pub max_stack: u16,
    pub max_locals: u16,
    pub code: Vec<TaggedInstruction>
}

impl RuntimeMethod {

    pub fn from_class_method(method: &Method) -> Option<RuntimeMethod> {
        for a in method.attributes.iter() {
            match a {
                &Attribute::Code { max_stack, max_locals, ref code, .. } => {
                    let code = disassembler::disassemble_code(&code).ok()?;
                    let runtime_method = RuntimeMethod {
                        name_index: method.name_index,
                        access_flags: method.access_flags,
                        max_stack,
                        max_locals,
                        code
                    };

                    return Some(runtime_method);
                },
                _ => {}
            }
        }

        None
    }

}
