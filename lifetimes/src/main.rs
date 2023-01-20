fn main() {
    println!("Hello, world!");
}

fn _longest<'a>(x: &'a str, y: &'a str) -> &'a str{ //lifetime specifier. Doesn't change how long the references live, but rather specifies the relationship of the return value and parameters
    if x.len() > y.len() {               //here the lifetime annotation tells that the return reference will be valid as long as both parameters are valid
        x
    } else {
        y
    }
}
//"The function signature now tells Rust that for some lifetime 'a, the function takes two parameters, 
//both of which are string slices that live at least as long as lifetime 'a."

//'static --> special lifetime notation