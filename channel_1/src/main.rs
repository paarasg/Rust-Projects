use std::thread;
use std::time::Duration;
use std::sync::mpsc;

fn main() {
    let (tx, rx) = mpsc::channel();
    let tx1 = mpsc::Sender::clone(&tx);

    thread::spawn(move || {
        let vec = vec![String::from("Hi"),
                       String::from("this"),
                       String::from("is"),
                       String::from("first"),
                       String::from("for"),
                       String::from("you"),
                    ];

        for message in vec {
            tx.send(message).unwrap();
            thread::sleep(Duration::from_secs(1));
        }

    });

    thread::spawn(move || {
        let vec = vec![String::from("Hi"),
                       String::from("this"),
                       String::from("is"),
                       String::from("second"),
                       String::from("for"),
                       String::from("you"),
                    ];
        
        for message in vec {
            tx1.send(message).unwrap();
            thread::sleep(Duration::from_secs(1));
        }

    });

    for received in rx {
        println!("{}", received);
    }

}
