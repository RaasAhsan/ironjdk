use std::collections::HashMap;
use code::instruction::Instruction;

struct ClassTable {
    pub classes: HashMap<String, RuntimeClass>
}

struct RuntimeClass {

}

pub struct RuntimeMethod {
    pub max_stack: u16,
    pub max_locals: u16,
    pub code: Vec<Instruction>
}
