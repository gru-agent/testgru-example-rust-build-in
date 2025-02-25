use std::fs;
use std::error::Error;
pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.file_path)?;

    println!("With text:\n{contents}");

    Ok(())
}

#[derive(Debug)]
pub struct Config {
    pub query: String,
    pub file_path: String,
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
mod tests {
    use super::*;

    #[test]
    fn test_one_result() {
        let query = "duct";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.";

        assert_eq!(vec!["safe, fast, productive."], search(query, contents));
    }

    #[test]
    fn test_no_result() {
        let query = "xyz";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.";

        assert_eq!(Vec::<&str>::new(), search(query, contents));
    }

    #[test]
    fn test_multiple_results() {
        let query = "the";
        let contents = "\
The quick brown fox
jumps over the lazy
dog in the garden";

        assert_eq!(
            vec!["jumps over the lazy", "dog in the garden"],
            search(query, contents)
        );
    }

    #[test]
    fn test_config_build_success() {
        let args = vec![
            String::from("program"),
            String::from("query"),
            String::from("file.txt"),
        ];
        let config = Config::build(&args).unwrap();
        assert_eq!(config.query, "query");
        assert_eq!(config.file_path, "file.txt");
    }

    #[test]
    fn test_config_build_not_enough_args() {
        let args = vec![String::from("program"), String::from("query")];
        let result = Config::build(&args);
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), "not enough arguments");
    }

    #[test]
    fn test_case_sensitive_search() {
        let query = "rUsT";
        let contents = "\
Rust:
Trust me, this is safe.
RUST is awesome!";

        assert_eq!(Vec::<&str>::new(), search(query, contents));
    }

    #[test]
    fn test_empty_content() {
        let query = "test";
        let contents = "";

        assert_eq!(Vec::<&str>::new(), search(query, contents));
    }

    #[test]
    fn test_empty_query() {
        let query = "";
        let contents = "\
Line one
Line two";

        assert_eq!(
            vec!["Line one", "Line two"],
            search(query, contents)
        );
    }
}
