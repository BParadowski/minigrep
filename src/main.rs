use minigrep::Config;
use std::env;
use std::process;

fn main() {
    let args: Vec<String> = env::args().collect();
    let config = Config::build(&args).unwrap_or_else(|err| {
        println!("Encountered an error: {err}");
        process::exit(1);
    });

    if let Err(e) = minigrep::run(config) {
        println!("Encountered an error: {:?}", e);
        process::exit(1);
    }
}
