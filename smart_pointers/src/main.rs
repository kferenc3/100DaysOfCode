use crate::List::{Cons, Nil};
use std::ops::Deref;

//Vec<T> and String are also smart pointers
//Box<T> --> we store something on the heap instead of the stack to spare copy cost
fn main() {
    let b = Box::new(5); //since 5 is an int it would be stored on the stack, but as a box it will be on heap
    println!("b = {}", b);
//Box is useful e.g when we would like to define a recursive type, like a cons list
//indirection: don't store the data directly on the stack, but rather indirectly (i.e only a pointer)
    let list = Cons(1, Box::new(Cons(2, Box::new(Cons(3, Box::new(Nil))))));

    let x = 5;
    let y = MyBox::new(x);

    assert_eq!(5,x);
    assert_eq!(5,*y); //with the deref trait implemented we can access the actual value of the reference

    let m = MyBox::new("Rust");
    hello(&m); //since deref is implemented Rust uses deref coercion to return a valid type here
    hello(&(*m)[..]) //without deref coercion this would be the correct syntax
}
fn hello(name: &str) {
    println!("Hello, {name}!");
}

enum List {
    Cons(i32, Box<List>), //without the box syntax the compiler would fail with "recursive type 'List' has infinite size"
    Nil,
}

struct MyBox<T>(T); //building an own smart pointer which is similar to a Box<T>

impl<T> MyBox<T> {
    fn new(x: T) -> MyBox<T> {
        MyBox(x)
    }
}

impl<T> Deref for MyBox<T> { //implementing the Deref trait on our type
    type Target = T; //the associated type for the trait to be used

    fn deref(&self) -> &Self::Target {
        &self.0 //since technically MyBox is a tuple we access the first value of the reference to get back the actual value
    }
}
