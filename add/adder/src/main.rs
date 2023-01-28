//day 28 - Using workspaces
//this is  project showcasing how to use multiple library crates depending on each other. See top level cargo.toml 
//1) create top level directory
//2) crate cargo.toml with the binary as a member
//3) cargo new pkg --> create the binary crate
//4) then we can create additional library crates with cargo new pkg --lib
//additional cargo commands:
//cargo run -p pkg --> run a specific crate
//cargo test -p pkg --> test a specific crate

use add_one;
use add_two;

fn main() {
    let num = 10;
    let num2 = 10;
    println!("Hello, world! {num} plus 1 is {}", add_one::add_one(num));
    println!("Hello, world! {num} plus 2 is {}", add_two::add_two(num2));
}
