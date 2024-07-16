use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    let query = &args[1];
    let path_to_file = &args[2];

    println!("Looking for {query} in {path_to_file}");
}
