use std::fs;
use std::error::Error;

#[derive(Debug)]
pub struct Config {
    pub query: String,
    pub file_path: String,
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.file_path)?;

    println!("With text:\n{contents}");

    Ok(())
}

impl Config {
    pub fn build(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("not enough arguments");
        }

        let query = args[1].clone();
        let file_path = args[2].clone();

        Ok(Config { query, file_path })
    }
}

pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let mut results = Vec::new();

    for line in contents.lines() {
        if line.contains(query) {
            results.push(line);
        }
    }

    results
}

#[cfg(test)]
mod test_search {
    use super::*;

    #[test]
    fn one_result() {
        let query = "duct";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.";

        assert_eq!(vec!["safe, fast, productive."], search(query, contents));
    }

    #[test]
    fn no_result() {
        let query = "xyz";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.";

        assert_eq!(Vec::<&str>::new(), search(query, contents));
    }

    #[test]
    fn multiple_results() {
        let query = "the";
        let contents = "\
The quick brown fox
jumps over the lazy dog
The end.";

        assert_eq!(
            vec!["jumps over the lazy dog"],
            search(query, contents)
        );
    }

    #[test]
    fn case_sensitive() {
        let query = "Rust";
        let contents = "\
Rust is great
rust is awesome
RUST rocks";

        assert_eq!(vec!["Rust is great"], search(query, contents));
    }

    #[test]
    fn empty_query() {
        let query = "";
        let contents = "Some content";

        assert_eq!(vec!["Some content"], search(query, contents));
    }

    #[test]
    fn empty_contents() {
        let query = "test";
        let contents = "";

        assert_eq!(Vec::<&str>::new(), search(query, contents));
    }
}

#[cfg(test)]
mod test_config {
    use super::*;

    #[test]
    fn config_build_success() {
        let args = vec![
            String::from("program"),
            String::from("query"),
            String::from("file.txt"),
        ];

        let config = Config::build(&args).unwrap();
        assert_eq!("query", config.query);
        assert_eq!("file.txt", config.file_path);
    }

    #[test]
    fn config_build_not_enough_args() {
        let args = vec![String::from("program"), String::from("query")];

        let result = Config::build(&args);
        assert!(result.is_err());
        assert_eq!("not enough arguments", result.unwrap_err());
    }

    #[test]
    fn config_build_empty_args() {
        let args: Vec<String> = vec![];

        let result = Config::build(&args);
        assert!(result.is_err());
        assert_eq!("not enough arguments", result.unwrap_err());
    }
}
