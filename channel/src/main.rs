use std::thread;
use std::sync::mpsc;

fn main() {
    let (tx, rx) = mpsc::channel();

    thread::spawn(move || {
        let value = String::from("HI");
        tx.send(value).unwrap();
    });

    let received = rx.recv().unwrap();

    println!("{}", received);
}
