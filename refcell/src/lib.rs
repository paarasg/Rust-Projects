pub trait Messanger {
    fn send(&self, message: &str);
}

pub struct LimitTracker<'a, T: 'a + Messanger> {
    messanger: &'a T,
    value: usize,
    max: usize,
}

impl<'a, T> LimitTracker<'a, T>
    where T: Messanger {
    pub fn new(messanger: &T, max: usize)->LimitTracker<T> {
        LimitTracker {
            messanger,
            value: 0,
            max,
        }
    }

    pub fn set_value(&mut self, value: usize) {
        self.value = value;

        let percentage = self.value as f64 / self.max as f64;

        if percentage > 1.0 {
            self.messanger.send("Error: You are over your quota!");
        } else if percentage >= 0.9 {
            self.messanger.send("Urgent warning: You've used up over 90% of your quota!");
        } else if percentage >= 0.75 {
            self.messanger.send("Warning: You've used up over 75% of your quota!");
        }
    }
}

