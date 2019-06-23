fn main() {
    let mut result = vec![1, 2, 3];

    let mut my_iter = result.iter_mut();

    assert_eq![my_iter.next(), Some(&mut 1)];

    assert_eq![my_iter.next(), Some(&mut 2)];

    assert_eq![my_iter.next(), Some(&mut 3)];

    assert_eq![my_iter.next(), None];
}
