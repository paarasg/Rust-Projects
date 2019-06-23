fn main() {
    let input = vec![1, 2, 3];

    let result: Vec<_> = input.iter().map(|x| {
        x + 100
    }).collect();

    println!("{:?}", result);
}
