use std::error::Error;
use std::fs;

#[derive(Debug)]
pub struct Config {
    pub file: String,
    pub query: String,
    pub case_sensitive: bool,
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.file)?;
    let fun = if config.case_sensitive {
        search
    } else {
        search_case_insensitive
    };
    for line in fun(&config.query, &contents) {
        println!("{}", line);
    }
    Ok(())
}

impl Config {
    pub fn new(mut args: std::env::Args) -> Result<Config, &'static str> {
        args.next(); // reject the first arg, always program name

        let query = match args.next() {
            Some(q) => q,
            None => return Err("Need a search query"),
        };

        let file = match args.next() {
            Some(f) => f,
            None => return Err("Need a valid path"),
        };

        let case_sensitive = args.next().is_some();

        Ok(Config {
            file,
            query,
            case_sensitive,
        })
    }
}

fn search<'a>(query: &str, content: &'a str) -> Vec<&'a str> {
    let mut result = vec![];
    for line in content.lines() {
        if line.contains(query) {
            result.push(line);
        }
    }
    result
}

fn search_case_insensitive<'a>(query: &str, content: &'a str) -> Vec<&'a str> {
    let mut result = vec![];
    let query = query.to_lowercase();
    for line in content.lines() {
        if line.to_lowercase().contains(&query) {
            result.push(line);
        }
    }
    result
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn one_result() {
        let query = "duct";
        let contents = "\
Rust
safe, fast, productive.
Pick three.
";

        assert_eq!(vec!["safe, fast, productive."], search(query, contents));
    }

    #[test]
    fn case_sensitive() {
        let query = "duct";
        let contents = "\
Rust
safe, fast, productive.
Pick three.
Duct tape";
        assert_eq!(vec!["safe, fast, productive."], search(query, contents));
    }

    #[test]
    fn case_insensitive() {
        let query = "rUsT";
        let contents = "\
Rust
safe, fast, productive.
Pick three.
Trust me.
Duct tape";
        assert_eq!(
            vec!["Rust", "Trust me."],
            search_case_insensitive(query, contents)
        );
    }
}
