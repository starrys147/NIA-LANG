use std::env;
use std::fs;

//
mod buffer;
mod command;
mod stack;

pub struct Config {
    filename: String,
}

pub fn run(cfg: Config) -> Result<(), String> {
    let nias = fs::read_to_string(cfg.filename).unwrap();

    let mut executer = command::Executer::from(nias).unwrap();
    executer.run()
}

impl Config {
    pub fn new(mut args: env::Args) -> Result<Config, &'static str> {
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
}