//Now that I went through the book I will solve old Advent Of Code puzzles to practice and once I'm a bit more confident I might do some more complex projects

use aoc2015::{
    solutions::day1::*,
    helpers::*,
};
use std::{env, process};

fn main() {
    let mut config = Config::build(env::args()).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {err}");
        process::exit(1);
    });

    println!("{:?}", config);
    let input = datagetter(2015, 1);
    println!("Part I solution: {}", part_i(&input));
    println!("Part II solution: {}", part_ii(&input));
}

