fn main() {
    let result = vec![1, 2, 3];

    let my_iter = result.iter();

    let total: i32 = my_iter.sum();

    println!("{}", total);
}
