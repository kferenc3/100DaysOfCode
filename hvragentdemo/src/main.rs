//day 37-39
//deviated from the book to practice a bit. A simple hvr agent plugin, I already wrote in python. I wanted to try it in Rust as well
//It was an interesting exercise to cross-compile the script to linux.

use std::{env, process};

#[derive(Debug)]
struct Config {
    script: String,
    mode: String,
    channel: String,
    location: String,
    arguments: Vec<String>,
    env_active: bool,
}

impl Config{
    fn build(
        mut args: impl Iterator<Item = String>,
    ) -> Result<Config, &'static str> {
        
        let script = match args.next() {
            Some(arg) => arg,
            None => return Err("Invalid arguments"),
        };
        
        let mode = match args.next() {
            Some(arg) => arg,
            None => return Err("Invalid argument"),
        };

        let channel = match args.next() {
            Some(arg) => arg,
            None => return Err("Invalid argument"),
        };

        let location = match args.next() {
            Some(arg) => arg,
            None => return Err("Invalid argument"),
        };

        let arguments: Vec<String> = args.collect();

        let env_active = match env::var("HVR_TESTENV") {
            Ok(s) if s == "1" => true,
            _ => false,
        };
        
        Ok(Config {
            script,
            mode,
            channel,
            location,
            arguments,
            env_active,
        })
    }
}

fn main() {
    let mut config = Config::build(env::args()).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {err}");
        process::exit(1);
    });
    println!("The script mode is: {:?}", config.mode);
    
    if config.env_active {
        println!("The environment variable is active!")
    } else {
        println!("The environment variable is not active!")
    }

    if config.mode == "refr_end" ||  config.mode == "integ_end" {
        //config.env_active = true;
        println!("{:?}", config);
    }

}
