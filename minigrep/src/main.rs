
//cargo run -- [args] - to test run the binary with arguments


//day 21: main part of today's learning was finishing chapter 11 regarding tests, 
//but I also wanted to start with chapter 12s project and since the latter is better suited to github that is what is visible here. 
//Also I figure this will span out for the next couple of days as well.

//day 22: added file reading and started some refactoring

//day 23: error handling, separation of concerns, lib.rs

//day 24: Developing with tests, environment variables, stderr

use std::{env, process};
use minigrep::Config;

fn main() {
    let args: Vec<String> = env::args().collect(); //env::args creates an iterator of the cmd args. collect turns them into a collection (in this case a vec)
    
    let config = Config::build(&args).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {err}"); //erpintln prints to stderr instead of stdout
        process::exit(1);
    });
    
    println!("Searching for {}", config.query);
    println!("In file {}", config.file_path);

   if let Err(e) = minigrep::run(config) {
    eprintln!("Application error: {e}");
    process::exit(1);
   }
}