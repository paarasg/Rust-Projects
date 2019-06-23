use std::fs;
use std::error::Error;
use std::env;

pub struct Config {
    pub query: String,
    pub file_name: String,
    pub case_insensitive: bool,
}

impl Config {
    pub fn new(mut args: std::env::Args)->Result<Config, &'static str> {
        args.next();
        
        let query = match args.next() {
            Some(arg) => arg,
            None => return Err("Didn't get query string"),
        };

        let file_name = match args.next() {
            Some(arg) => arg,
            None => return Err("Didn't get file_name string"),
        };

        let case_insensitive = env::var("CASE_SENSITIVE").is_err();

        Ok(Config {query, file_name, case_insensitive})
    }
}

pub fn run(config: Config)->Result<(), Box<dyn Error>> {
    let content = fs::read_to_string(config.file_name)?;
   
    let result = if config.case_insensitive {
        search_case_insensitive(&config.query, &content)
    } else {
        search_case_sensitive(&config.query, &content)
    };

    for line in result {
        println!("{}", line);
    }

    Ok(())
}


pub fn search_case_sensitive<'a, 's>(query: &str, content: &'a str)->Vec<&'a str> {

    content.lines()
        .filter(|line| line.contains(query))
        .collect()
}

pub fn search_case_insensitive<'a>(query: &str, content: &'a str)->Vec<&'a str> {

    content.lines()
        .filter(|line| line.to_lowercase().contains(&query.to_lowercase()))
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case_sensitive_search() {
        let query = "duct";
        let content = "\
Rust:
safe, fast, productive.
Pick three.
DUCT my hell.";
        assert_eq!(
            vec!["safe, fast, productive."], search_case_sensitive(query, content)
        );
    }

    #[test]
    fn case_insensitive_search() {
        let query = "rUST";
        let content = "\
Rust:
safe, fast, productive.
Pick three.
Trust me.";

        assert_eq!(
            vec!["Rust:", "Trust me."], search_case_insensitive(query, content)
        );
    }
}
