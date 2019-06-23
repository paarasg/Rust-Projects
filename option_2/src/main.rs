fn plus_one(x: Option<i32>)->Option<i32> {
    match x {
        None => None,
        Some(i) => {
            println!("{}", i);
            Some(i+1)
        },
    }
}

fn main() {
    let i = plus_one(Some(5));
    let j = plus_one(None); 
    let k = Some(i);

    println!("{:?}", Some(k));
}
