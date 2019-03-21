use code::instruction::Instruction;
use class::{ConstantPool, Method};
use code::RuntimeMethod;

#[derive(Debug)]
struct StackFrame {
    locals: Vec<StackVariable>,
    stack: Vec<StackVariable>
}

impl StackFrame {

    fn new_frame(max_locals: u16, max_stack: u16) -> StackFrame {
        let locals: Vec<StackVariable> = vec![StackVariable::Empty; max_locals as usize];
        let stack: Vec<StackVariable> = Vec::new();

        StackFrame { locals, stack }
    }

}

#[derive(Clone, Debug)]
enum StackVariable {
    Integer(i32),
    Empty
}

pub fn interpret(method: &RuntimeMethod, cp: &ConstantPool) {
    let mut stack: Vec<StackFrame> = Vec::new();

    let mut stack_frame = StackFrame::new_frame(method.max_stack, method.max_locals);

    for instruction in method.code.iter() {
        match instruction {
            Instruction::Iconst0 => {
                stack_frame.stack.push(StackVariable::Integer(0));
            },
            Instruction::Istore1 => {
                let operand = stack_frame.stack.pop().unwrap();

                match operand {
                    i @ StackVariable::Integer { .. } => {
                        stack_frame.locals[1] = i;
                    },
                    _ => {
                        panic!("Found non-integer on top of operand stack.");
                    }
                }
            },
            _ => {
                println!("{:?}", instruction);
            }
        }
    }

    println!("{:?}", stack_frame);
}
