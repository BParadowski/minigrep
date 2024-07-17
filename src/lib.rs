use std::error::Error;
use std::{env, fs};
pub struct Config {
    pub query: String,
    pub path: String,
    pub ignore_case: bool,
}

impl Config {
    pub fn build(args: &[String]) -> Result<Config, &str> {
        if args.len() < 3 {
            return Err("Fewer than 2 arguments provided.");
        }
        let query = args[1].clone();
        let path = args[2].clone();
        let ignore_case = env::var("IGNORE_CASE").is_ok();

        Ok(Config {
            query,
            path,
            ignore_case,
        })
    }
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    println!("Looking for {} in {}", config.query, config.path);
    let contents = fs::read_to_string(config.path)?;

    let result = if config.ignore_case {
        search_insensitive(&config.query, &contents)
    } else {
        search(&config.query, &contents)
    };

    for line in result {
        println!("{}", line);
    }

    Ok(())
}

fn search<'a>(target: &str, source: &'a str) -> Vec<&'a str> {
    let mut result: Vec<&'a str> = Vec::new();

    for line in source.lines() {
        if line.contains(target) {
            result.push(line);
        }
    }

    result
}

fn search_insensitive<'a>(target: &str, source: &'a str) -> Vec<&'a str> {
    let mut result: Vec<&'a str> = Vec::new();

    for line in source.lines() {
        if line.to_lowercase().contains(target) {
            result.push(line);
        }
    }

    result
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn one_result() {
        let contents = "hello
I love you
very much hewoo tooo
boomcykcyk";
        let query = "hewo";

        assert_eq!(vec!["very much hewoo tooo"], search(query, contents));
    }

    #[test]
    fn multiple_results() {
        let contents = "I'm nobody! Who are you?
Are you nobody, too?
Then there's a pair of us - don't tell!
They'd banish us, you know.

How dreary to be somebody!
How public, like a frog
To tell your name the livelong day
To an admiring bog!
";
        let query = "og";

        assert_eq!(
            vec!["How public, like a frog", "To an admiring bog!"],
            search(query, contents)
        );
    }
}
