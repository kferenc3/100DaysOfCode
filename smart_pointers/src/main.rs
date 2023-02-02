use crate::List::{Cons, Nil};
use std::ops::Deref;
use std::rc::Rc;

//Rc<T> reference counter -> multiple concurrent owner

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
    hello(&(*m)[..]); //without deref coercion this would be the correct syntax

    let c = CustomSmartPointer {
        data: String::from("my stuff"),
    };

    let d = CustomSmartPointer {
        data: String::from("other stuff"),
    };
    println!("CustomSmartpointers created,");
    //std::mem::drop(c) --> function to force a drop. we can't call the drop method directly

    //let a = Rc::new(Cons(5, Box::new(Cons(10, Box::new(Nil)))));
    //let b = Cons(3, Box::new(&a));
    //let c = Cons(4, Box::new(&a));
    //in the above example with Rc the reference ownership will be shared between b and c and a will only go out of scope once both b and c released it

    //Rc::clone --> doesn't create a deep copy, only increases the reference count.
    //Rc::strong_count --> displays the reference count.

    //RefCell<T>
    //Normally either 1 mutable reference or any number of immutable reference, references must always be valid
    //With RefCell the borrowing rules will be only enforced at runtime not at compile time. However if the rules are broken at runtime the program will panic


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

struct CustomSmartPointer {
    data: String,
}

impl Drop for CustomSmartPointer { //implementing the drop trait on smart pointers we can customize the behavior of the code when and how the variable goes out of scope
    fn drop(&mut self) {
        println!("Dropping CustomSmartPointer with data '{}' !", self.data);
    }
}
