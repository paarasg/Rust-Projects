enum List {
    Cons(i32, Box<List>),
    Nil,
}

use crate::List::{Cons, Nil};

fn main() {
    let list = Cons(2,
                Box::new(Cons(3,
                Box::new(Cons(4,
                Box::new(Cons(5,
                Box::new(Nil))))))));
}
