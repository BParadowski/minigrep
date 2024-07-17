use std::env;
use std::error::Error;
use std::fs;
use std::process;

fn main() {
    let args: Vec<String> = env::args().collect();
    let config = Config::build(&args).unwrap_or_else(|err| {
        println!("Encountered an error: {err}");
        process::exit(1);
    });

    if let Err(e) = run(config) {
        println!("Encountered an error: {:?}", e);
        process::exit(1);
    }
}

struct Config {
    query: String,
    path: String,
}

impl Config {
    fn build(args: &[String]) -> Result<Config, &str> {
        if args.len() < 3 {
            return Err("Fewer than 2 arguments provided.");
        }
        let query = args[1].clone();
        let path = args[2].clone();

        Ok(Config { query, path })
    }
}

fn run(config: Config) -> Result<(), Box<dyn Error>> {
    println!("Looking for {} in {}", config.query, config.path);
    let contents = fs::read_to_string(config.path)?;
    println!("{contents}");

    Ok(())
}
