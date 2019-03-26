use code::instruction::Instruction;
use class::{ConstantPool, Method};
use runtime::class::{RuntimeMethod, RuntimeClass};
use std::rc::Rc;
use std::cell::RefCell;

// TODO: Implement locals and stack with an array
#[derive(Debug)]
struct StackFrame {
    locals: Vec<Value>,
    stack: Vec<Value>
}

impl StackFrame {

    fn pop_stack(&mut self) -> Option<Value> {
        self.stack.pop()
    }

    fn push_stack(&mut self, operand: Value) {
        self.stack.push(operand)
    }

    fn get_local(&self, index: usize) -> &Value {
        &self.locals[index]
    }

    fn set_local(&mut self, index: usize, var: Value) {
        self.locals[index] = var
    }

    // int instructions

    fn push_int(&mut self, integer: i32) {
        self.push_stack(Value::Integer(integer))
    }

    fn pop_int(&mut self) -> Result<i32, InterpreterError> {
        let operand = self.pop_stack().unwrap();

        match operand {
            Value::Integer(i) => Ok(i),
            _ => Err(InterpreterError::UnexpectedOperand)
        }
    }

    fn pop_int_array(&mut self) -> Result<Rc<RefCell<IntArray>>, InterpreterError> {
        let operand = self.pop_stack().unwrap();

        match operand {
            Value::IntegerArrayRef(reference) => Ok(reference),
            _ => Err(InterpreterError::UnexpectedOperand)
        }
    }

    fn get_int_local(&self, index: usize) -> Result<i32, InterpreterError> {
        let operand = self.get_local(index);

        match operand {
            Value::Integer(i) => Ok(*i),
            _ => Err(InterpreterError::UnexpectedOperand)
        }
    }

    fn set_int_local(&mut self, index: usize, value: i32) {
        self.set_local(index, Value::Integer(value))
    }

    fn new_frame(max_stack: u16, max_locals: u16) -> StackFrame {
        let locals: Vec<Value> = vec![Value::Null; max_locals as usize];
        let stack: Vec<Value> = Vec::new();

        StackFrame { locals, stack }
    }

}

// A StackValue is any data type that can be stored in a variable.
// In Java, there are two kinds of data types: primitive types and reference types.
// Reference types are either objects or arrays.
#[derive(Clone, Debug)]
enum Value {
    Long(i64),
    Integer(i32),
    Short(i16),
    Byte(i8),
    Character(char),
    IntegerArrayRef(Rc<RefCell<IntArray>>),
    ObjectRef(Rc<RefCell<Object>>),
    Null
}

#[derive(Debug)]
struct Object {
    class: Rc<RuntimeClass>,
    memory: Box<ObjectData>
}

#[derive(Debug)]
struct ObjectData {
    fields: Vec<Value>
}

#[derive(Debug)]
struct IntArray {
    array: Vec<i32>
}

impl IntArray {
    fn get(&self, index: usize) -> i32 {
        self.array[index]
    }

    fn set(&mut self, index: usize, value: i32) {
        self.array[index] = value;
    }
}

#[derive(Debug)]
enum InterpreterError {
    UnhandledInstruction(Instruction),
    UnexpectedOperand,
    InvalidArrayType
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
    println!("{:?}", std::mem::size_of::<Rc<RefCell<Vec<i32>>>>());
}

fn interpret_instruction(instruction: &Instruction, stack_frame: &mut StackFrame) -> Result<(), InterpreterError> {
    println!("{:?}", instruction);

    match instruction {
        Instruction::Aload1 => {
            let operand = stack_frame.get_local(1).clone();
            stack_frame.push_stack(operand);
            Ok(())
        },
        Instruction::Astore1 => {
            let operand = stack_frame.pop_stack().unwrap();
            stack_frame.set_local(1, operand);
            Ok(())
        },
        Instruction::Dup => {
            // TODO: Do we need to clone twice here, or is once sufficient?
            let operand = stack_frame.pop_stack().unwrap();
            stack_frame.push_stack(operand.clone());
            stack_frame.push_stack(operand.clone());
            Ok(())
        },
        Instruction::Iadd => {
            let v2 = stack_frame.pop_int()?;
            let v1 = stack_frame.pop_int()?;
            stack_frame.push_int(v1 + v2);
            Ok(())
        },
        Instruction::Iaload => {
            let index = stack_frame.pop_int()?;
            let array = stack_frame.pop_int_array()?;
            let value = array.borrow().get(index as usize);
            stack_frame.push_int(value);
            Ok(())
        },
        Instruction::Iastore => {
            let value = stack_frame.pop_int()?;
            let index = stack_frame.pop_int()?;
            let mut array = stack_frame.pop_int_array()?;
            array.borrow_mut().set(index as usize, value);
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
            let v2 = stack_frame.pop_int()?;
            let v1 = stack_frame.pop_int()?;
            stack_frame.push_int(v1 * v2);
            Ok(())
        },
        Instruction::Iload { index } => {
            let int = stack_frame.get_int_local(*index as usize)?;
            stack_frame.push_int(int);
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
        Instruction::Newarray { atype } => {
            let count = stack_frame.pop_int()?;
            // These are array type codes. We could classify them.
            match atype {
                10 => {
                    let array = Value::IntegerArrayRef(new_integer_array(count as usize));
                    stack_frame.push_stack(array);
                    Ok(())
                },
                _ => Err(InterpreterError::InvalidArrayType)
            }
        },
        Instruction::Sipush(value) => {
            stack_frame.push_int(*value);
            Ok(())
        },
        Instruction::Return => {
            Ok(())
        },
        x => Err(InterpreterError::UnhandledInstruction(*x))
    }
}

fn new_integer_array(size: usize) -> Rc<RefCell<IntArray>> {
    let array = vec![0; size];
    Rc::new(RefCell::new(IntArray { array }))
}
