use runtime::{Value, IntArray, Object};
use std::cell::RefCell;
use std::rc::Rc;
use runtime::interpreter::InterpreterError;

// TODO: Implement locals and stack with an array
#[derive(Debug)]
pub struct StackFrame {
    locals: Vec<Value>,
    stack: Vec<Value>
}

// TODO: Don't use InterpreterError
impl StackFrame {

    pub fn pop_stack(&mut self) -> Option<Value> {
        self.stack.pop()
    }

    pub fn push_stack(&mut self, operand: Value) {
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
        self.push_stack(Value::Integer(integer))
    }

    pub fn pop_int(&mut self) -> Result<i32, InterpreterError> {
        let operand = self.pop_stack().unwrap();

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
        let operand = self.pop_stack().unwrap();

        match operand {
            Value::IntegerArrayRef(reference) => Ok(reference),
            _ => Err(InterpreterError::UnexpectedOperand)
        }
    }

    // object reference helpers

    pub fn push_object_reference(&mut self, reference: Rc<RefCell<Object>>) {
        self.push_stack(Value::ObjectRef(reference))
    }

    pub fn pop_object_reference(&mut self) -> Result<Rc<RefCell<Object>>, InterpreterError> {
        let operand = self.pop_stack().unwrap();

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

}
