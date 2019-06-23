fn main() {
    let mut v: Vec<i32> = Vec::new();

    v.push(1);
    v.push(2);
    v.push(3);

    for i in &v {
        println!("{}", i);
    }


    let mut my_v = vec![1, 2, 3];

    for i in &mut my_v {
        *i += 30;
        println!("{}", i);
    }

    my_v.push(4);


    let two = &my_v[0];
   
    println!("{}", two);

    my_v.push(5);
  /*  let three = v.get(100);

    match three {
        Some(i) => println!("{}", i),
        None => println!("There is no value existed"),
    }
  */

}
