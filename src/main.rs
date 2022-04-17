use std::process;
use std::env;

// 
use nia_lang::Config;

fn main() {
    let config = Config::new(env::args()).unwrap_or_else(|err| {
        eprintln!("Error parsing arguments: {}", err);
        process::exit(1)
    });

    nia_lang::run(config).unwrap();
}