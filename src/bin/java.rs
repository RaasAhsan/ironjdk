extern crate ironjdk;

use ironjdk::class::{reader, ClassFile};
use ironjdk::runtime;
use std::io::prelude::*;
use std::fs::File;
use std::io::Error;
use ironjdk::class::method;
use ironjdk::class::reader::ClassReaderError;
use std::collections::HashMap;
use ironjdk::runtime::class::RuntimeClass;

fn load_class_from_file(path: &str) -> Result<ClassFile, ClassReaderError> {
    let mut file = File::open("Counter.class").unwrap();
    let mut buffer = Vec::new();

    file.read_to_end(&mut buffer).unwrap();

    reader::read_class_file(&mut buffer)
}

fn main() {
    println!("IronJDK 1.0.0");

//    let runtime_context = RuntimeContext {
//        loaded_class_table: HashMap::new()
//    };

    let class_file = load_class_from_file("Counter.class").unwrap();
    class_file.print_constant_pool();
    let entry_class_file = RuntimeClass::from_class_file(&class_file).unwrap();

    println!("Running class file {}", entry_class_file.class_name);

    let main_method = class_file.find_method("main", method::ACC_PUBLIC & method::ACC_STATIC).unwrap();
    let runtime_method = main_method.disassemble().unwrap();

    runtime::interpreter::interpret(&runtime_method, &class_file.constant_pool);

    ()
}
