use code::instruction::Instruction;
use class::{ConstantPool, Method};
use code::RuntimeMethod;

// TODO: Implement locals and stack with an array
#[derive(Debug)]
struct StackFrame {
    locals: Vec<StackValue>,
    stack: Vec<StackValue>
}

impl StackFrame {

    fn pop_stack(&mut self) -> Option<StackValue> {
        self.stack.pop()
    }

    fn push_stack(&mut self, operand: StackValue) {
        self.stack.push(operand)
    }

    fn get_local(&self, index: usize) -> StackValue {
        self.locals[index]
    }

    fn set_local(&mut self, index: usize, var: StackValue) {
        self.locals[index] = var
    }

    // int instructions

    fn push_int(&mut self, integer: i32) {
        self.push_stack(StackValue::Integer(integer))
    }

    fn pop_int(&mut self) -> Result<i32, InterpreterError> {
        let operand = self.pop_stack().unwrap();

        match operand {
            StackValue::Integer(i) => Ok(i),
            _ => Err(InterpreterError::UnexpectedOperand)
        }
    }

    fn get_int_local(&self, index: usize) -> Result<i32, InterpreterError> {
        let operand = self.get_local(index);

        match operand {
            StackValue::Integer(i) => Ok(i),
            _ => Err(InterpreterError::UnexpectedOperand)
        }
    }

    fn set_int_local(&mut self, index: usize, value: i32) {
        self.set_local(index, StackValue::Integer(value))
    }

    fn new_frame(max_stack: u16, max_locals: u16) -> StackFrame {
        let locals: Vec<StackValue> = vec![StackValue::Empty; max_locals as usize];
        let stack: Vec<StackValue> = Vec::new();

        StackFrame { locals, stack }
    }

}

// A StackValue is any data type that can be stored in a variable.
// In Java, there are two kinds of data types: primitive types and reference types.
#[derive(Copy, Clone, Debug)]
enum StackValue {
    Long(i64),
    Integer(i32),
    Short(i16),
    Byte(i8),
    Character(char),
    Empty
}

#[derive(Debug)]
enum InterpreterError {
    UnhandledInstruction(Instruction),
    UnexpectedOperand
}

pub fn interpret(method: &RuntimeMethod, cp: &ConstantPool) {
    let mut stack: Vec<StackFrame> = Vec::new();

    let mut stack_frame = StackFrame::new_frame(method.max_stack, method.max_locals);

    for instruction in method.code.iter() {
        let res = interpret_instruction(instruction, &mut stack_frame);
        match res {
            Ok(_) => {},
            Err(e) => {
                println!("{:?}", e);
                return
            }
        }
    }

    println!("{:?}", stack_frame);
}

fn interpret_instruction(instruction: &Instruction, stack_frame: &mut StackFrame) -> Result<(), InterpreterError> {
    println!("{:?}", instruction);

    match instruction {
        Instruction::Iadd => {
            let value2 = stack_frame.pop_int()?;
            let value1 = stack_frame.pop_int()?;

            stack_frame.push_int(value1 + value2);

            Ok(())
        },
        Instruction::Iconst0 => {
            stack_frame.push_int(0);
            Ok(())
        },
        Instruction::Iconst1 => {
            stack_frame.push_int(1);
            Ok(())
        },
        Instruction::Iconst2 => {
            stack_frame.push_int(2);
            Ok(())
        },
        Instruction::Iconst3 => {
            stack_frame.push_int(3);
            Ok(())
        },
        Instruction::Iconst4 => {
            stack_frame.push_int(4);
            Ok(())
        },
        Instruction::Iconst5 => {
            stack_frame.push_int(5);
            Ok(())
        },
        Instruction::Imul => {
            let value2 = stack_frame.pop_int()?;
            let value1 = stack_frame.pop_int()?;

            stack_frame.push_int(value1 * value2);

            Ok(())
        },
        Instruction::Iload0 => {
            let int = stack_frame.get_int_local(0)?;
            stack_frame.push_int(int);
            Ok(())
        },
        Instruction::Iload1 => {
            let int = stack_frame.get_int_local(1)?;
            stack_frame.push_int(int);
            Ok(())
        },
        Instruction::Iload2 => {
            let int = stack_frame.get_int_local(2)?;
            stack_frame.push_int(int);
            Ok(())
        },
        Instruction::Iload3 => {
            let int = stack_frame.get_int_local(3)?;
            stack_frame.push_int(int);
            Ok(())
        },
        Instruction::Istore(index) => {
            let int = stack_frame.pop_int()?;
            stack_frame.set_int_local(*index as usize, int);
            Ok(())
        },
        Instruction::Istore0 => {
            let int = stack_frame.pop_int()?;
            stack_frame.set_int_local(0, int);
            Ok(())
        },
        Instruction::Istore1 => {
            let int = stack_frame.pop_int()?;
            stack_frame.set_int_local(1, int);
            Ok(())
        },
        Instruction::Istore2 => {
            let int = stack_frame.pop_int()?;
            stack_frame.set_int_local(2, int);
            Ok(())
        },
        Instruction::Istore3 => {
            let int = stack_frame.pop_int()?;
            stack_frame.set_int_local(3, int);
            Ok(())
        },
        Instruction::Isub => {
            let value2 = stack_frame.pop_int()?;
            let value1 = stack_frame.pop_int()?;

            stack_frame.push_int(value1 - value2);

            Ok(())
        },
        Instruction::Sipush(value) => {
            stack_frame.push_int(*value);
            Ok(())
        },
        Instruction::Return => {
            Ok(())
        },
        x => return Err(InterpreterError::UnhandledInstruction(*x))
    }
}
