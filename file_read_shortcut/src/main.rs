use std::io::{self, Read};
use std::fs::File;

fn main() {
    match read_from_file() {
        Ok(s) => println!("The content of the file is {}", s),
        Err(err) => panic!("Couldn't read the file {:?}", err),
    };
}

fn read_from_file()->Result<String, io::Error> {
    let mut f = File::open("hello.txt")?;
    let mut s = String::new();
    f.read_to_string(&mut s)?;
    Ok(s)
}

fn short_read_from_file()->Result<String, io::Error> {
    let mut s = String::new();
    File::open("hello.txt")?.read_to_string(&mut s)?;
    Ok(s)
}
