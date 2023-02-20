//Now that I went through the book I will solve old Advent Of Code puzzles to practice and once I'm a bit more confident I might do some more complex projects

use std::fs;
use aoc2015::solutions::day1::*;

fn main() {
    let input = fs::read_to_string("inputdata").expect("Unable to read file");
    println!("Part I solution: {}", part_i(&input));
    println!("Part II solution: {}", part_ii(&input));
}