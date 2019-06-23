use std::collections::HashMap;

fn main() {
    let mut my_hash = HashMap::new();

    my_hash.insert(String::from("blue"), 30);
    my_hash.insert(String::from("yellow"), 30);
    println!("{:?}", my_hash);

    for (key, value) in &my_hash {
        println!("{}, {}", key, value);
    }

    let paaras = String::from("Paaras");
    let mehak = String::from("Mehak");

    my_hash.insert(paaras, 50);
    my_hash.insert(mehak, 60);


    let input_str = String::from("paaras gupta");

    for each_char in input_str.chars() {
        let count = my_hash.entry(each_char.to_string()).or_insert(0);
        *count += 1;
        if *count > 1 {
            println!("{} -> {}", each_char, *count);
        }
    }
    println!("{:?}", my_hash);
}
