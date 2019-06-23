fn main() {
    let z = vec![1, 2, 3];

    let equal_to_x = move |x| x == z;

    println!("{:?}", z);

    let y = vec![1, 2, 3];

    assert!(equal_to_x(y));
}
