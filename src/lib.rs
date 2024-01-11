use std::{fs, error::Error, env};

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.file_path)?;

    let results = if config.ignore_case {
        search_case_insensive(config.query.as_str(), contents.as_str())
    } else {
        search(config.query.as_str(), contents.as_str())
    };

    for result in results {
        println!("{}", result);
    }
  
    Ok(())
}

pub struct Config {
    query: String,
    file_path: String,
    ignore_case: bool,
}

impl Config {
    pub fn build(mut args: impl Iterator<Item = String>) -> Result<Config, &'static str> {

        args.next();
        
        let query = match args.next() {
            Some(s) => s,
            None => return Err("Cannot match query parameter from the command"),
        };

        let file_path = match args.next() {
            Some(s) => s,
            None => return Err("Cannot match file_path parameter from the command"),
        };

        let ignore_case_cmd: Option<String> = args.next();

        let ignore_case = get_ignore_case(ignore_case_cmd);
    
        Ok(Config { query, file_path, ignore_case })
    }
}

// Valid command line option take precedence over env var for ignoring case
fn get_ignore_case(ignore_case_cmd: Option<String>) -> bool {
    let ignore_case_cmd = match ignore_case_cmd {
        None => None,
        Some(s) => s.parse::<bool>().ok()
    };

    let ignore_case_env = env::var("IGNORE_CASE").is_ok();

    match ignore_case_cmd {
        Some(b) => b,
        None => ignore_case_env
    }
}

fn search<'a>(query: & str, contents: &'a str) -> Vec<&'a str> {

    contents
        .lines()
        .filter(|line| line.contains(query))
        .collect()
}

fn search_case_insensive<'a>(query: & str, contents: &'a str) -> Vec<&'a str> {
    
    let query = query.to_uppercase();

    contents
        .lines()
        .filter(|line| line.to_uppercase().contains(&query))
        .collect()
}

#[cfg(test)]
mod tests{
    use super::*;

    #[test]
    fn test_search() {
        let query = "duct";
        //tells Rust not to put a newline character at the beginning of the contents of this string literal
        let text = "\
Rust:
safe, fast, productive.
Pick three.";

        assert_eq!(search(query, text), vec!["safe, fast, productive."])
    }

    #[test]
    fn test_search_case_insensive() {
        let query = "rUsT";
        //tells Rust not to put a newline character at the beginning of the contents of this string literal
        let text = "\
Rust:
safe, fast, productive.
Pick three.
Trust me";

        assert_eq!(search_case_insensive(query, text), vec!["Rust:", "Trust me"])
    }    
}