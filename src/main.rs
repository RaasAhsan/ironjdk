use std::io::prelude::*;
use std::fs::File;

mod class;

fn remove(buffer: &mut Vec<u8>) {
    buffer.remove(0);
}

fn main() {
    let mut file = File::open("Counter.class").unwrap();
    let mut buffer = Vec::new();

    file.read_to_end(&mut buffer).unwrap();

    for x in &buffer {
        println!("{:02X} = {}", x, x);
    }

    remove(&mut buffer);
    remove(&mut buffer);

    ()
}
