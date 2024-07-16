use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();
    let query = &args[1];
    let path_to_file = &args[2];

    println!("Looking for {query} in {path_to_file}");

    let contents = fs::read_to_string(path_to_file).expect("File should exist!");

    println!("{contents}");

}
