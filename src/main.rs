use std::env;
use std::process;
use testgru_example_rust::Config;

fn main(){
    let args : Vec<String> = env::args().collect();
    let config = Config::build(&args).unwrap_or_else(|err|{
        println!("Problem parsing arguments: {}", err);
        process::exit(1);
    });

    println!("Searching for {}", config.query);
    println!("In file {}", config.file_path);

    if let Err(e) = testgru_example_rust::run(config) {
        println!("Application error: {e}");
        process::exit(1);
    }
}

#[cfg(test)]
mod test_main {
    use super::*;

    #[test]
    fn test_main_with_valid_args() {
        let args = vec![
            String::from("program"),
            String::from("test"),
            String::from("test.txt")
        ];
        let config = Config::build(&args).unwrap();
        assert_eq!(config.query, "test");
        assert_eq!(config.file_path, "test.txt");
    }

    #[test]
    #[should_panic(expected = "not enough arguments")]
    fn test_main_with_insufficient_args() {
        let args = vec![String::from("program")];
        Config::build(&args).unwrap();
    }

    #[test]
    fn test_main_error_handling() {
        let args = vec![
            String::from("program"),
            String::from("test"),
            String::from("nonexistent.txt")
        ];
        let config = Config::build(&args).unwrap();
        let result = testgru_example_rust::run(config);
        assert!(result.is_err());
    }
}
