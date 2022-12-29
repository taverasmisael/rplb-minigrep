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
    pub fn new(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("Not enough arguments.");
        }
        let query = args[1].clone();
        let file = args[2].clone();
        let case_sensitive = args.get(3).unwrap_or(&"".to_string()).to_string();
        let case_sensitive = !case_sensitive.is_empty();
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
