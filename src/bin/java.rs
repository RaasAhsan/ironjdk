extern crate ironjdk;

use ironjdk::class::{reader, ClassFile};
use ironjdk::runtime;
use std::io::prelude::*;
use std::fs::File;
use std::io::Error;
use ironjdk::class::method;
use ironjdk::class::reader::ClassReaderError;
use ironjdk::runtime::class::{RuntimeClass, ClassTable};

fn load_class_from_file(path: &str) -> Result<ClassFile, ClassReaderError> {
    let mut file = File::open("Counter.class").unwrap();
    let mut buffer = Vec::new();

    file.read_to_end(&mut buffer).unwrap();

    reader::read_class_file(&mut buffer)
}

fn main() {
    println!("IronJDK 1.0.0");

    let mut class_table = ClassTable::new();

    let class_file = load_class_from_file("Counter.class").unwrap();
    let runtime_class = RuntimeClass::from_class_file(&class_file).unwrap();

    class_table.load_class(&runtime_class);
    println!("{:?}", class_table);

    println!("Running class file {}", runtime_class.class_name);

    let main_method = runtime_class.get_method("main").unwrap();
    let expected_access_flags = method::ACC_PUBLIC | method::ACC_STATIC;
    if main_method.access_flags & expected_access_flags == expected_access_flags {
        let arguments = Vec::new(); // String[]
        runtime::interpreter::invoke_static(arguments, &main_method, &runtime_class, &class_table);
    }

    ()
}
