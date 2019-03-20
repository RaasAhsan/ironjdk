use std::io::prelude::*;
use std::fs::File;

extern crate ironjdk;

use ironjdk::class::reader;

fn main() {
    println!("IronJDK 1.0.0");

    let mut file = File::open("rt/java/lang/System.class").unwrap();
    let mut buffer = Vec::new();

    file.read_to_end(&mut buffer).unwrap();

//    for x in &buffer {
//        println!("{:02X} = {}", x, x);
//    }

    let result = reader::read_class_file(&mut buffer);
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
