//Now that I went through the book I will solve old Advent Of Code puzzles to practice and once I'm a bit more confident I might do some more complex projects

use aoc2015::helpers::*;
use std::{env, process};
use std::time::Instant;

fn main() {
    let start = Instant::now();
    let config = Config::build(env::args()).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {err}");
        process::exit(1);
    });
    let solution = match solver(config.year, config.day) {
        Ok(t) => t,
        Err(_) => {eprintln!("Unable to solve this day!");
                process::exit(1)}
    };
    println!("The solution for day {} is: \n - part I: \t{}\n - part II: \t{}", config.day, solution.0, solution.1);
    let duration = start.elapsed();
    println!("{:?}", duration);
}

