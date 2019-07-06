pub struct AverageObject {
    list: Vec<u32>,
    average: f32,
}

impl AverageObject {
    pub fn add(&mut self, value:u32) {
        self.list.push(value);
        self.update_average();
    }

    pub fn remove(&mut self)->Option<u32> {
        let object = self.list.pop();

        match object {
            Some(value)=> {
                self.update_average();
                Some(value)
            },
            None=>None,
        }
    }

    fn update_average(&mut self) {
        let total: u32 = self.list.iter().sum();
        self.average = total as f32 / self.list.iter().len() as f32;
    }
}

