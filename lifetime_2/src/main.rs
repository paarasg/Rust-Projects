use std::fmt::Display;

struct Tweet <'a> {
    tweet1: &'a str,
    tweet2: &'a str,
}

fn largest<'a, T>(tweet1: &'a str, tweet2: &'a str, item: &[T])->&'a str
    where T: Display 
{
    println!("{}", item[0]);
    if tweet1.len() > tweet2.len() {
        tweet1
    } else {
        tweet2
    }
}

fn main() {
   let my_str1 = String::from("Hi this is Paaras");
   let my_str2 = String::from("HI this is mehak");

   let item = vec![10];

   let my_tweet = Tweet{tweet1: &my_str1, tweet2: &my_str2};

   let largest_tweet = largest(my_tweet.tweet1, my_tweet.tweet2, &item);
   println!("{}", largest_tweet);

    println!("{}", item[0]);
}
