fn main() {
    let arr = vec![20, 45, 12, 7, 100, 89];

    let largest = calculate_highest(&arr);

    println!("The largest number is {}", largest);
}

fn calculate_highest(list: &[i32])->i32 {
    let mut largest = list[0];

    for item in list.iter() {
        if *item > largest {
            largest = *item;
        }
    }
    largest
}
