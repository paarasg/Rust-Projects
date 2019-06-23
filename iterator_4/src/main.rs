#[derive(PartialEq, Debug)]
struct Shoe {
    size: u32,
    style: String,
}

fn shoe_in_size(shoes: Vec<Shoe>, size: u32)->Vec<Shoe> {
    shoes.into_iter().
        filter(|x| x.size == size).
        collect()
}

#[test]

fn shoe_in_size_test() {
    let shoes = vec![ 
        Shoe {size: 10, style: String::from("sneekers")},
        Shoe {size: 20, style: String::from("boot")},
        Shoe {size: 10, style: String::from("sandel")},];

    let shoe_in_size = shoe_in_size(shoes, 10);

    assert_eq!(shoe_in_size,
               vec![Shoe{size:10, style: String::from("sneekers")},
                    Shoe{size:10, style:  String::from("sandel")},]
    );
} 
