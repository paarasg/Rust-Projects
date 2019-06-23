pub mod back_of_house {
    pub struct Breakfast {
        pub toast: String,
        pub fruit: String,
        pub cost: u32,
    }

    impl Breakfast {
        pub fn summer(toast: &str)->Breakfast {
            Breakfast {
                toast: String::from(toast),
                fruit: String::from("orange"),
                cost: super::meal_coast(toast, "orange"),
            }
        }
    }
}

fn meal_coast(toast: &str, fruit: &str)->u32 {
    let toast_cost;
    let fruit_cost;

    match toast {
        rye =>      {
                        toast_cost = 9;
                    },

        wheat =>    {
                        toast_cost = 10;
                    },
    }

    match fruit {
        orange =>   {
                        fruit_cost = 11;
                    },
        apple =>    {
                        fruit_cost = 15;
                    },
    }

    toast_cost + fruit_cost
}

fn main() {
    let mut my_meal = back_of_house::Breakfast::summer("rye");

    my_meal.toast = String::from("wheat");
    println!("{} {} {}", my_meal.toast, my_meal.fruit, my_meal.cost);
}
