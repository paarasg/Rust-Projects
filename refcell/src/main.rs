fn main() {

}

#[cfg(test)]
mod tests {
    use std::cell::RefCell;
    use refcell::*;

    struct MockMessanger {
        mockmessage: RefCell<Vec<String>>,
    }

    impl MockMessanger {
        fn new()->MockMessanger {
            MockMessanger{ mockmessage: RefCell::new(vec![]) }
        }
    }

    impl Messanger for MockMessanger {
        fn send(&self, message: &str) {
            self.mockmessage.borrow_mut().push(String::from(message));
        }
    }

    #[test]

    fn test_mock_message() {
        let mock_messanger = MockMessanger::new();
        let mut limit_tracker = LimitTracker::new(&mock_messanger, 100);
        
        limit_tracker.set_value(95);

        assert_eq!(1, mock_messanger.mockmessage.borrow().len());
    }
}
