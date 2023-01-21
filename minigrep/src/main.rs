//main part of today's learning was finishing chapter 11 regarding tests, 
//but I also wanted to start with chapter 12s project and since the latter is better suited to github that is what is visible here. 
//Also I figure this will span out for the next couple of days as well.
use std::env;

//cargo run -- [args] - to test run the binary with arguments
fn main() {
    let args: Vec<String> = env::args().collect(); //env::args creates an iterator of the cmd args. collect turns them into a collection (in this case a vec)
    
    let query = &args[1];
    let file_path = &args[2]; 
    
    println!("Searching for {}", query);
    println!("In file {}", file_path);
}
