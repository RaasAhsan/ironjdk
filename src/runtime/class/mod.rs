use class::{ClassFile, ConstantPool};
use std::collections::HashMap;
use std::rc::Rc;
use runtime::Value;
use runtime::class::field::{RuntimeField, FieldDescriptor};
use runtime::class::method::RuntimeMethod;

pub mod field;
pub mod method;

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
