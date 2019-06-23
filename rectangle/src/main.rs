#[derive(Debug)]

struct Rect {
    width: u32,
    length: u32,
}

//Method
impl Rect {
    fn area(&mut self)->u32 {
        self.width * self.length
    }
}

//Associated function
impl Rect {
    fn square(size: u32)->Rect {
        Rect {width:size, length:size}
    }
}

//Mutiple instances

impl Rect { 
    fn fits_in(&mut self, other: &Rect)->bool {
        if self.length > other.length && self.width > other.width {
            return true;
        } else {
            return false;
        }
    }
}

fn main() {
    let mut my_rect = Rect {width:20, length:30};

    println!("The area of rect is {}", my_rect.area());

    let other_rect = Rect {width:5, length:10};

    println!("The other rectangle fits in {}", my_rect.fits_in(&other_rect));

    let sq = Rect::square(15);

    println!("The square is {:?}", sq);
}
