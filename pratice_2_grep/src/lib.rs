use std::error::Error;
use std::{env, fs};

pub struct Config {
    pub query: String,
    pub filename: String,
    pub ignore_case: bool,
}

impl Config {
    pub fn build(mut args: impl Iterator<Item=String>) -> Result<Config, &'static str> {
        args.next();
        let query = match args.next() {
            Some(arg) => arg,
            None => return Err("didn't get a query string"),
        };
        let filename = match args.next() {
            Some(arg) => arg,
            None => return Err("didn't get a filename string"),
        };
        let ignore_case = env::var("IGNORE_CASE").is_ok();
        Ok(Config { query, filename, ignore_case })
    }
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.filename)?;
    let contains_vec = search(&config.query, &contents, config.ignore_case);
    for result in contains_vec {
        println!("{}: {}", result.line_number, result.line);
    }
    Ok(())
}

#[derive(Debug)]
struct SearchResult<'a> {
    line: &'a str,
    line_number: usize,
}

impl PartialEq for SearchResult<'_> {
    fn eq(&self, other: &Self) -> bool {
        self.line == other.line && self.line_number == other.line_number
    }
}

fn search<'a>(query: &'a str, contents: &'a str, ignore_case: bool) -> Vec<SearchResult<'a>> {
    let mut results = Vec::new();
    let mut line_number = 0;
    for line in contents.lines() {
        line_number += 1;
        if check_contains(query, line, ignore_case) {
            results.push(SearchResult {
                line,
                line_number,
            });
        }
    }
    results
}

fn check_contains(query: &str, line: &str, ignore_case: bool) -> bool {
    if ignore_case {
        line.to_lowercase().contains(&query.to_lowercase())
    } else {
        line.contains(query)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn one_result() {
        let query = "duct";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.";
        assert_eq!(vec![SearchResult { line: "safe, fast, productive.", line_number: 2 }], search(query, contents, false));
    }

    #[test]
    fn ignore_case_search() {
        let query = "rUsT";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Trust me.";
        assert_eq!(vec![SearchResult { line: "Rust:", line_number: 1 }, SearchResult { line: "Trust me.", line_number: 4 }], search(query, contents, true));
    }
}