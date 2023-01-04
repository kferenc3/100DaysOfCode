pub fn ifs(x: i32) {
    let number = x;

    //simple if
    if number < 5 {
        println!("Condition was true!");
    } else {
        println!("Condition was false!");
    }

    //else if
    if number % 4 == 0 {
        println!("number is divisible by 4");
    } else if number % 3 == 0 {
        println!("number is divisible by 3");
    } else if number % 2 == 0 {
        println!("number is divisible by 2");
    } else {
        println!("number is not divisible 4, 3, or 2");
    }

    //assigning a variable with a conditional
    let condition = true;
    let number = if condition { 5 } else { 6 }; //types are important!
    println!("The value of number is: {number}");
}