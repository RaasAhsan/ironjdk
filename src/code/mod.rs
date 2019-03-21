use code::instruction::Instruction;

pub mod disassembler;
pub mod instruction;

pub struct RuntimeMethod {
    pub max_stack: u16,
    pub max_locals: u16,
    pub code: Vec<Instruction>
}
