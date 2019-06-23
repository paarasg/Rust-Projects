use std::fs::File;
use std::io::ErrorKind;

fn main() {
    let f = File::open("hello.txt");
    let f  = match f {
        Ok(file) => file,
        Err(error) => match error.kind() {
            ErrorKind::NotFound => match File::create("hello.txt") {
                                Ok(fc) => fc,
                                Err(err) => panic!("Tried creating the file but failed !"),
                            },
            other_error => panic!("Failed to open file ! {:?}", other_error),
        },
    };
}

