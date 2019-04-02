use runtime::{Value, IntArray, Object};
use std::cell::RefCell;
use std::rc::Rc;
use runtime::interpreter::InterpreterError;

// TODO: Implement locals and stack with an array
#[derive(Debug)]
pub struct StackFrame {
    pub locals: Vec<Value>,
    pub stack: Vec<Value>
}

// TODO: Don't use InterpreterError
impl StackFrame {

    pub fn pop(&mut self) -> Option<Value> {
        self.stack.pop()
    }

    pub fn pop_many(&mut self, count: usize) -> Option<Vec<Value>> {
        let mut values: Vec<Value> = Vec::new();

        for i in 0..count {
            let value = self.pop()?;
            values.push(value);
        }

        Some(values)
    }

    pub fn push(&mut self, operand: Value) {
        self.stack.push(operand)
    }

    pub fn get_local(&self, index: usize) -> &Value {
        &self.locals[index]
    }

    pub fn set_local(&mut self, index: usize, var: Value) {
        self.locals[index] = var
    }

    // int helpers

    pub fn push_int(&mut self, integer: i32) {
        self.push(Value::Integer(integer))
    }

    pub fn pop_int(&mut self) -> Result<i32, InterpreterError> {
        let operand = self.pop().unwrap();

        match operand {
            Value::Integer(i) => Ok(i),
            _ => Err(InterpreterError::UnexpectedOperand)
        }
    }

    pub fn get_int_local(&self, index: usize) -> Result<i32, InterpreterError> {
        let operand = self.get_local(index);

        match operand {
            Value::Integer(i) => Ok(*i),
            _ => Err(InterpreterError::UnexpectedOperand)
        }
    }

    pub fn set_int_local(&mut self, index: usize, value: i32) {
        self.set_local(index, Value::Integer(value))
    }

    // int array reference helpers

    pub fn pop_int_array(&mut self) -> Result<Rc<RefCell<IntArray>>, InterpreterError> {
        let operand = self.pop().unwrap();

        match operand {
            Value::IntegerArrayRef(reference) => Ok(reference),
            _ => Err(InterpreterError::UnexpectedOperand)
        }
    }

    // object reference helpers

    pub fn push_object_reference(&mut self, reference: Rc<RefCell<Object>>) {
        self.push(Value::ObjectRef(reference))
    }

    pub fn pop_object_reference(&mut self) -> Result<Rc<RefCell<Object>>, InterpreterError> {
        let operand = self.pop().unwrap();

        match operand {
            Value::ObjectRef(reference) => Ok(reference),
            _ => Err(InterpreterError::UnexpectedOperand)
        }
    }

    pub fn new_frame(max_stack: u16, max_locals: u16) -> StackFrame {
        let locals: Vec<Value> = vec![Value::Null; max_locals as usize];
        let stack: Vec<Value> = Vec::new();

        StackFrame { locals, stack }
    }

    pub fn new_frame_with_locals(max_stack: u16, max_locals: u16, mut locals: Vec<Value>) -> StackFrame {
        let mut remaining_locals = vec![Value::Null; (max_locals as usize) - locals.len()];
        locals.append(&mut remaining_locals);
        StackFrame { locals, stack: Vec::new() }
    }

}
