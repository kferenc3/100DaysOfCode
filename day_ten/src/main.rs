fn main() {
    //use std::fmt::Result;
    //use std::io::Result as IoResult; --> bringing in two types with the same name: renaming with 'as' works. Or we can just specify
    // fmt::Result() and io::Result respectively

    //pub use std::fmt::Result; --> re-export. the "use" will be available for others as well.

    //Vec<T> --> Vector, values of the same type, stored on heap, can change size
    let mut v: Vec<i32> = Vec::new(); //since we initialized an empty vector we need to specify what types it will hold
    //alternative: let v = vec![1,2,3];
    v.push(5); //adding elements to the vector
    v.push(6);
    v.push(7);
    v.push(8);

    let third: &i32 = &v[2]; //accessing an element
    println!("The third element is {}", third);

    let third: Option<&i32> = v.get(2); //get will always give back an Option<&T>
    match third {
        Option::Some(third) => println!("The third element is {third}"),
        Option::None => println!("There is no third element"),
    }

    //let does_not_exist: &i32 = &v[100]; //accessing an element that doesn't exits will panic like this
    //this however will run fine:
    let does_not_exist: Option<&i32> = v.get(100);
    match does_not_exist {
        Option::Some(does_not_exist) => println!("The 100th element is {}", does_not_exist),
        Option::None => println!("That doesn't exist, sorry."),
    }

    //iterating
    for i in &mut v {
        *i += 50;
        println!("{}",i);
    }

    enum SpreadsheetCell {
        Int(i32),
        Float(f64),
        Text(String),
    }

    let row = vec![  //storing the vector elements as variants of an enum so it will technically hold only the enum type, but in reality it can hold multiple types like this
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Text(String::from("blue")),
        SpreadsheetCell::Float(10.12),
    ];

    //String is a wrapper around a vector of bytes so 
    //let mut s = String::new();
    //or
    //let s = "some text".to_string(); to initialize new string. btw: let s = String::from("some text") is equivalent
    //UTF-8 encoded
    let mut s = String::from("foo");
    s.push_str(" bar"); //similar to vectors

    let s1 = String::from("foo");
    let s2 = String::from("bar");
    let s3 = s1 + &s2; //s1 is moved, so we are not able to use it anymore (how 'add' works internally)

    //alternatively:
    let s4 = String::from("some");
    let s5 = String::from("text");
    let s = format!("{s4}-{s5}"); //the format macro works with references in the backend so we'll not move s4 or s5

    //Stings are stored as bytes so indexing one them wouldn't work e.g hello[0] wouldn't return h, but the corresponding byte

    //iteration works thoug
    for c in "Зд".chars() {
        println!("{c}");
    }

    //same with the byte values
    for b in "Зд".bytes() {
        println!("{b}");
    }

}
