fn main() {

    enum IpAddrKind { //defining an enum
        V4(u8, u8, u8, u8), //The enum variants themselves can hold values
        V6(String), //the enum types can differ. Also the enum data can be a struct or another enum even.
    }

    //enums can also have their own methods

    let home = IpAddrKind::V4(127, 0, 0, 1); //creating an instance of an enum
    let loopback = IpAddrKind::V6(String::from("::1"));

    //special enum: Option
    enum Option<i32> { //generic type
        None,
        Some(i32),
    }

    let some_num = Some(5); //we can define option variants directly
    let some_char = Some('c');

    let absent_num: Option<i32> = Option::None; //Since None is an absent value we have to define the overall option's type. Some can infer the type

    //Option<T> and T are different types! Option<i8> != i8

    //in "normal" types the variable ALWAYS has a value. There is no 'null' in Rust. So in case we need to handle absent values 
    //we need to opt for an Option<T> type to handle any possible null value scenarios

    enum Coin {
        Penny,
        Nickel,
        Dime,
        Quarter(UsState),
    }
    
    #[derive(Debug)]
    enum UsState {
        Alabama,
        Alaska,
        NorthVirgina,
        Hawaii,
    }

    fn value_in_cents(coin: Coin) -> u8 {  //match statements to handle different scenarios. This time different variants of an enum
        match coin{                        //while an if statement can return only a boolean, a match statement can return any type we define
            Coin::Penny => {
                println!("Lucky penny!");
                1
            }                              //a match arm's return part can have a curly bracket section if the code is more than one row. In this case the end comma is optional.   
            Coin::Nickel => 5,
            Coin::Dime => 10,
            Coin::Quarter(state) => {       //we can also extract an enum's value in a match arm
                println!("The state is {:?}", state);
                25
            }    
        }
    }

    fn plus_one(x: Option<i32>) -> Option<i32> {
        match x {
            Option::None => Option::None,
            Option::Some(i) => Option::Some(i + 1),
        }
    }

    let five = Option::Some(5);
    let six = plus_one(five);
    let none = plus_one(Option::None);

    //match arms have to be exhaustive and need to cover all options
}
