use class::{Method, Attribute, ConstantPool};
use code::disassembler;
use code::instruction::TaggedInstruction;

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

struct MethodDescriptor {
    parameters: Vec<FieldType>,
    return_type: ReturnDescriptor
}

enum ReturnDescriptor {
    Void,
    Field(FieldType)
}

enum FieldType {
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

enum LexingRule {
    String(String),
    Regex()
}
