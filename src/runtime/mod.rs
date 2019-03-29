use std::collections::HashMap;
use runtime::class::RuntimeClass;
use std::cell::RefCell;
use std::rc::Rc;

pub mod class;
pub mod gc;
pub mod jit;
pub mod interpreter;
pub mod stack;

// A StackValue is any data type that can be stored in a variable.
// In Java, there are two kinds of data types: primitive types and reference types.
// Reference types are either objects or arrays.
#[derive(Clone, Debug)]
pub enum Value {
    Long(i64),
    Integer(i32),
    Short(i16),
    Byte(i8),
    Character(char),
    ObjectRef(Rc<RefCell<Object>>),
    IntegerArrayRef(Rc<RefCell<IntArray>>),
    Null
}

#[derive(Debug)]
pub struct Object {
    class: Rc<RuntimeClass>, // or perhaps an index into a loaded class table
    memory: Box<ObjectData>
}

#[derive(Debug)]
pub struct ObjectData {
    fields: Vec<Value>
}

#[derive(Debug)]
pub struct IntArray {
    array: Vec<i32>
}

impl IntArray {
    pub fn get(&self, index: usize) -> i32 {
        self.array[index]
    }

    pub fn set(&mut self, index: usize, value: i32) {
        self.array[index] = value;
    }

    pub fn new(size: usize) -> Rc<RefCell<IntArray>> {
        let array = vec![0; size];
        Rc::new(RefCell::new(IntArray { array }))
    }
}
