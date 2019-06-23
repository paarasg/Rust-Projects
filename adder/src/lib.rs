struct Rectangle {
    length: u32,
    width: u32,
}

impl Rectangle {
    fn larger_contains_smaller(&self, other: &Rectangle) {
        if self.length < other.length || self.width < other.width {
            panic!("The larger should contain sammler rectangle");
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[should_panic(expected = "The larger should contain sammler rectangle")]
    fn test_larger_contains_smaller() {
        let my_rec1 = Rectangle{length: 32, width: 68};
        let my_rec2 = Rectangle{length: 56, width: 43};

        my_rec1.larger_contains_smaller(&my_rec2);
    }
}
