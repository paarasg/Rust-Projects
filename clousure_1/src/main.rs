fn main() {
    let x = 4;

    let equal_to_x = |z| z == x;

    assert!(equal_to_x(5));
}
