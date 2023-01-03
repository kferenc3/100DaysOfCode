//Chapter 3
//Variables, data types, functions, comments

//Immutability - as x is immutable the below code won't compile
//error[E0384]: cannot assign twice to immutable variable `x`

// fn main() {
//     let x = 5;
//     println!("The value of x is: {x}");
//     x = 6;
//     println!("The value of x is: {x}");
// }

//instead it should be declared as a mutable var as seen in fn mutability.
fn main() {
    mutability();
    shadowing();
    tuple_destructuring();
    another_function(5, 'h');
    let z = plusone(5);
    println!("5 plus 1 is: {z}")
}
    
fn mutability() {
    let mut x = 5;
    println!("The value of x is: {x}");
    x = 6;
    println!("The value of x is: {x}");

    //constants
    //all uppercase naming convention, always immutable and only constant expression, must annotate type
    const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;
}    

fn shadowing() {
    //shadowing
    let y = 5;

    let y = y + 1;

    {
        let y = y * 2;
        println!("The value of y in the inner scope is: {y}");
    }

    println!("The value of y is: {y}");

    let spaces = "    ";
    let spaces = spaces.len();
    println!("The string spaces have been shadowed and the type was also changed. The value is now: {spaces}");
    //This same thing wouldn't work with just simply declaring a mutable variable:
    //let mut spaces = "    ";
    //spaces = spaces.len();
}

//Data types:
//signed vs unsigned: e.g.: u8 = 0 to (2**n) - 1  vs i8 = -(2**n-1) to (2**n-1)-1
//visual separator: 1000 = 1_000
//type suffix: 57 = 57u8
// f32 single precision floating point, f64 is double precision floating point
// char uses single quotes('z') vs string ("zorro")

fn tuple_destructuring() {
    let tup: (i32, f64, u8) = (500, 6.4, 1);

    let (x,y,z) = tup;

    println!("The value of y is: {y}");
    //indexing also works
    let second = tup.1;
    println!("The same but with indexing: {second}")
}

//Arrays:
//let a: [i32, 5] = [1,2,3,4,5]
//let a = [3; 5]  is the same as let a = [3,3,3,3,3]
//array indexing works the same as in python --> a[0]

//function with a parameter
fn another_function(x: i32, y: char) {
    println!("The value of the function parameter is {x} and {y}")
}
//expressions don't need a semicolon. if it ends with a semicolon it will not return a value

//function with a return type (note that it contains only an expression, therefore it returns value)
fn plusone(x: i32) -> i32 {
    x + 1
}