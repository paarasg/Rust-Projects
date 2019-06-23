fn main() {
    let my_list = vec![10, 5, 67, 45, 90];
    let mut largest = calculate_largest(&my_list);
    println!("{}", largest);


    let my_list_1 = vec!['a', 'r', 'p', 'q', 'f'];
    largest = calculate_largest(&my_list);
    println!("{}", largest);
}

fn calculate_largest<T>(list: &[T])->T {
    let mut largest = list[0];

    for &item in list.iter() {
        if largest < item {
            largest = item;
        }
    }
    largest
}
