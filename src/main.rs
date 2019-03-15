use std::io::prelude::*;
use std::fs::File;

mod class;
mod disassembler;
mod instruction;
mod parser;

fn longer<'a>(s1: &'a str, s2: &'a str) -> &'a str {
    if s1.len() > s2.len() {
        s1
    } else {
        s2
    }
}

fn main() {
    let mut file = File::open("Counter.class").unwrap();
    let mut buffer = Vec::new();

    file.read_to_end(&mut buffer).unwrap();

//    for x in &buffer {
//        println!("{:02X} = {}", x, x);
//    }

    let result = parser::parse_class_file(&mut buffer);
    println!("{:#?}", result);

    ()
}
