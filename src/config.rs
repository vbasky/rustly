use std::error::Error;
use std::fs;
use std::{env, vec};

pub trait Configuration {
    fn build(args: std::env::Args) -> Result<Self, &'static str>
    where
        Self: Sized;
    fn run(&self) -> Result<(), Box<dyn Error>>;
    fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str>;
    fn search_case_insensitive<'a>(query: &str, contents: &'a str) -> Vec<&'a str>;
}

pub struct Config {
    pub query: String,
    pub filename: String,
    pub ignore_case: bool,
}

impl Configuration for Config {
    fn build(mut args: std::env::Args) -> Result<Config, &'static str> {
        args.next();

        let query = match args.next() {
            Some(arg) => arg,
            None => return Err("Didn't get a query string"),
        };

        let filename = match args.next() {
            Some(arg) => arg,
            None => return Err("Didn't get a file name"),
        };

        let ignore_case = env::var("IGNORE_CASE").is_ok();

        Ok(Config {
            query,
            filename,
            ignore_case,
        })
    }

    fn run(&self) -> Result<(), Box<dyn Error>> {
        let contents =
            fs::read_to_string(&self.filename).expect("Should have been able to read the file");

        let results = if self.ignore_case {
            Config::search_case_insensitive(&self.query, &contents)
        } else {
            Config::search(&self.query, &contents)
        };

        if vec![""].eq(&results) {
            println!("No results found for query: {}", self.query);
        }

        for line in results {
            println!("{line}");
        }

        Ok(())
    }

    fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
        contents
            .lines()
            .filter(|line| line.contains(query))
            .collect()
    }

    fn search_case_insensitive<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
        let mut results = Vec::new();
        let query = query.to_lowercase();

        for line in contents.lines() {
            if line.to_lowercase().contains(&query) {
                results.push(line);
            }
        }

        results
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case_sensitive() {
        let query = "duct";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Duct tape.";

        assert_eq!(
            vec!["safe, fast, productive."],
            Config::search(query, contents)
        );
    }

    #[test]
    fn case_insensitive() {
        let query = "rUsT";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Trust me.";

        assert_eq!(
            vec!["Rust:", "Trust me."],
            Config::search_case_insensitive(query, contents)
        );
    }
}
