pub trait Summery {
    fn summerize(&self)->String {
        String::from("The more to read..")
    }

    fn summerize_author(&self)->String;
}

struct Tweet {
    name: String,
    info: String,
}


impl Summery for Tweet{
    fn summerize_author(&self)->String {
        format!("{} {}", self.summerize(), self.name)
    }
}

fn notify<T: Summery>(item: T) {
    println!("{}", item.summerize_author());
}

fn main() {
    let my_tweet = Tweet {
                        name: String::from("Paaras"),
                        info: String::from("a tweet for politics"),
                    };
    println!("{}", my_tweet.summerize_author());

    notify(my_tweet);
}
