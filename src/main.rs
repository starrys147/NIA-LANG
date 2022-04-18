use std::env;
use std::process;

//
use nia_lang::Config;

fn main() {
    let config = Config::new(env::args()).unwrap_or_else(|err| {
        eprintln!("Error parsing arguments: {}", err);
        process::exit(1)
    });

    match nia_lang::run(config) {
        Ok(_) => (),
        Err(err) => {
            eprintln!("Error: {}", &err);
            process::exit(0)
        }
    };
}
