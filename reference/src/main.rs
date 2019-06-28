use crate::List::{Cons, Nil};
use std::rc::Rc;

enum List {
    Cons(u32, Rc<List>),
    Nil,
}

fn main() {
    let a = Rc::new(Cons(3,
                Rc::new(Cons(4,
                    Rc::new(Cons(5,
                        Rc::new(Nil)))))));

    let b = Cons(6, Rc::clone(&a));

    let c = Cons(7, Rc::clone(&a));
              

}
