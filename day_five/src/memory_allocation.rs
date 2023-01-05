pub fn my_strings() {
    let mut s = String::from("hello"); //creating a string type (!= to a string literal, i.e simply "hello") - also scpope for s begins

    s.push_str(", world!"); //String is stored on the heap, but string literals can be pushed to the stack

    println!("{}", s);
} //scope ends, s is no longer valid. Going out of scope also means its memory is freed up with 'drop'

pub fn memory_move() {
    let s1 = String::from("hello");
    let s2 = s1; //this doesn't create a copy. It only copies the heap references of s1 AND invalidates s1. 
                //So s1 is now out of scope. Called "move"
}

pub fn takes_ownership(some_string: String) {
    println!("{}", some_string);
}

pub fn pass_ball() -> String {
    let some_string = String::from("ball");
    some_string
}

pub fn catch_and_pass_ball(a_string: String) -> String {
    a_string //just simply returning the recieved value
}

pub fn referencing_example(s: &String) -> usize { //&String indicates we only pass a reference, so ownership is not moved
    s.len()
}

pub fn change_with_ref(s: &mut String) { //this function requires a mutable string reference so it can modify without taking ownership
    s.push_str(" game");
}