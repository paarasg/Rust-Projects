fn main() {
    let s = String::from("hello");
    let x = 10;

    my_func_str(&s);
    my_func_int(x);

    println!("{}", s);
}

fn my_func_str(s: &String) {
    println!("{}", s);
}

fn my_func_int(x: i32) {
    println!("{}", x);
}
