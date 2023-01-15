use std::fs::File;
use std::io::ErrorKind;
use std::io::{self, Read};

fn main() {
    //panic!("crash an burn"); //panic can be called as a macro to stop execution when we intend with an error message.
    //by default rust cleans up memory at panic, however adding panic='abort' will simply abort and let the OS do the cleanup (faster)
    //let v = vec![1,2,3];
    //v[99];
    //using RUST_BACKTRACE=1 we can see the entire trace of the error

    let greeting_file_result = File::open("hello.txt");
    let greeting_file = match greeting_file_result {
        Ok(file) => file,
        Err(error) => match error.kind() {
            ErrorKind::NotFound => match File::create("hello.txt") {
                Ok(fc) => fc,
                Err(e) => panic!("Problem creating the file: {:?}", e),
            },
            other_error => {
                panic!("Problem opening the file: {:?}", error);
            } 
        }
    };
    //Result enums are useful to handle recoverable error. However we can decide to panic instead as well.
    //Also we can use ErrorKind to differentiate between errors and choose different actions (like if the file doesn't exist, create it instead of panicing).
    // .unwrap, unwrap_or_else, expect are ways to simplify code instead of lengthy match statements

    //propagating:
}

fn _read_username_from_file_v1() -> Result<String, io::Error> {
    let username_file_result = File::open("hello.txt");
    let mut username_file = match username_file_result {
            Ok(file) => file,
            Err(e) => return Err(e),
    };

    let mut username = String::new();

    match username_file.read_to_string(&mut username) {
        Ok(_) => Ok(username),
        Err(e) => Err(e), //here we are returning the entire error. Since the return type of the function itslef is a Result<> the function caller
        //can simply decide what to do with the error message
    }
}

fn _read_username_from_file_v2() -> Result<String, io::Error> {
    let mut username_file = File::open("hello.txt")?; //the ? is a shortcut to propagate/return the error message emmediately
    let mut username = String::new();
    username_file.read_to_string(&mut username)?; //this way the code is much shorter and easier to read
    Ok(username)
}

fn _read_username_from_file_v3() -> Result<String, io::Error> {
    let mut username = String::new();

    File::open("hello.txt")?.read_to_string(&mut username)?; //even more compact version of the same code

    Ok(username)

    //v4 of this code would be a one-liner: fs::read_to_string("hello.txt")  --> the exact same functionality as the previous versions

    //Note: the ? operator can only be used in functions that return Return, Option types or types that have trait FromResidual
}
