use code::instruction::TaggedInstruction;
use class::ClassFile;

#[derive(Debug)]
pub struct RuntimeClass {
    pub class_name: String
}

impl RuntimeClass {

    pub fn from_class_file(class_file: &ClassFile) -> Result<RuntimeClass, String> {
        let class_name = class_file.constant_pool.resolve_class_name(class_file.this_class)?;

        let runtime_class = RuntimeClass {
            class_name
        };

        Ok(runtime_class)
    }



}

struct RuntimeConstantPool {

}

pub struct RuntimeMethod {
    pub max_stack: u16,
    pub max_locals: u16,
    pub code: Vec<TaggedInstruction>
}

impl RuntimeMethod {

}
