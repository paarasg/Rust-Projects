use std::thread;

fn main() {
    let my_vec = vec![1, 2, 3];

    let handle = thread::spawn(move || {
        for index in my_vec.iter() {
            println!("{}", index);
        }
    });

    handle.join().unwrap();
}
