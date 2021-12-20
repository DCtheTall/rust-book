use std::env;
use std::process;

use::minigrep::Config;

fn main() {
    // Refactor config to take 
    let config = Config::new(env::args()).unwrap_or_else(|err| {
        // `eprintln!` macro prints to stderr instead of stdout
        eprintln!("Problem parsing arguments: {}", err);
        process::exit(1);
    });
    println!("Searching for {}", config.query);
    println!("In file {}", config.path);
    if let Err(e) = minigrep::run(config) {
        eprintln!("Application error: {}", e);
        process::exit(1);
    }
}
