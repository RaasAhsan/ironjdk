use std::io::prelude::*;
use std::fs::File;

mod class;
mod classreader;
mod disassembler;
mod instruction;

fn longer<'a>(s1: &'a str, s2: &'a str) -> &'a str {
    if s1.len() > s2.len() {
        s1
    } else {
        s2
    }
}

fn main() {
    println!("IronJDK 1.0.0");

    let mut file = File::open("rt/java/lang/Thread.class").unwrap();
    let mut buffer = Vec::new();

    file.read_to_end(&mut buffer).unwrap();

//    for x in &buffer {
//        println!("{:02X} = {}", x, x);
//    }

    let result = classreader::read_class_file(&mut buffer);
    match result {
        Ok(class_file) => {
            class_file.print_constant_pool();
            class_file.debug()
        },
        Err(e) => {
            println!("{:?}", e);
        }
    }

    ()
}
