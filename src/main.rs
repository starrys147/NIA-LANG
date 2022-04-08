use regex::Regex;
use std::error::Error;
use std::process;
use std::env;
use std::fs;

fn main() {
    let config = Config::new(env::args()).unwrap_or_else(|err| {
        eprintln!("Error parsing arguments: {}", err);
        process::exit(1)
    });

    config.run().unwrap_or_else(| err | {
        eprintln!("Error: {}", err);
        process::exit(1);
    });
}

struct Config {
    filename: String,
}

impl Config {
    fn new(mut args: env::Args) -> 
    Result<Config, &'static str> {
        if args.len() < 2 {
            return Err("not enough args");
        }
        args.next();

        let filename = match args.next() {
            Some(str) => str,
            None => return Err("Didn't get filename"),
        };

        Ok(Config { filename })
    }

    fn run(&self) -> Result<(), Box<dyn Error>>{
        let contents = fs::read_to_string(&self.filename)
            .unwrap()
            .to_lowercase();

        for line in contents.lines() {
            println!("line: {}", line);
        }   
        Ok(())
    }
}
