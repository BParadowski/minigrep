use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();
    let config = parse_config(&args);

    println!("Looking for {} in {}", config.query, config.path);

    let contents = fs::read_to_string(config.path).expect("File should exist!");

    println!("{contents}");
}

struct Config {
    query: String,
    path: String,
}

fn parse_config(args: &[String]) -> Config {
    let query = args[1].clone();
    let path = args[2].clone();

    Config { query, path }
}
