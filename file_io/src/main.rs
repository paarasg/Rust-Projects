use std::io::{self, Read};
use std::fs::File;

fn main() {
    match read_from_file() {
        Ok(s) => println!("The content of the file is {}", s),
        Err(err) => panic!("Could not read file {:?}", err),
    };
}

fn read_from_file()->Result<String, io::Error> {
    let f = File::open("hello.txt");
    let mut f = match f {
        Ok(file) => file,
        Err(err) => return Err(err),
    };
    let mut s = String::new();
    match f.read_to_string(&mut s) {
        Ok(_) => Ok(s),
        Err(err) => Err(err),
    }
}
