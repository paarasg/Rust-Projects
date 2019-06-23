fn main() {
    let tup:(i8, f32, u32) = (10, 34.4, 500);
    let arr: [i8; 4] = [10, 20, 30, 40];

    let x = tup.0;
    let y = tup.1;
    let z = tup.2;
    println!("x = {}, y = {}, z = {}", x ,y, z);

    let z = {
        x + 4
    };
    println!("x = {}, y = {}, z = {}", x ,y, z);

    let x = my_add(x);
    println!("{}", x);

    for i in arr.iter() {
        println!("{}", i);
    }
}

fn my_add(x: i8) -> i8 {
    x + 1
}
