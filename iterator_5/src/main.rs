struct Counter {
    count: u32,
}

impl Counter {
    fn new()->Counter {
        Counter{0)
    }
}

impl Iterator for Counter {
    type Item = u32;

    fn next(&mut self)->Option<self::Item> {
        self.count += 1;
        
        if self.count < 6 {
            Some(self.count)
        } else {
            None
        }
    }
}

fn main() {

}
