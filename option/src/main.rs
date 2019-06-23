#[derive(Debug)]

fn main() {
    let x: u8 = 5;

    let y: Option<i8> = Some(5);

    let z = x + y;

    println!("sum is {}", z);
}
