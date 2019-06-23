fn main() {
    let string1 = String::from("Hi this is Paaras");
    let result;
    {
        let string2 = String::from("Hi this is Mehak");
        result = largest(string1.as_str(), string2.as_str());
        println!("The largest string is {}", result);
    }
}

fn largest<'a>(string1: &'a str, string2: &'a str)->&'a str {
    if string1.len() > string2.len() {
        string1
    } else {
        string2
    }
}
