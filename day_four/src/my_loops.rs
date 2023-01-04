pub fn looooops(x: i32) {
    let mut number = x;
    let mut counter = 0;
    let result = loop {
        counter += 1;

        if counter == x {
            break counter * 2;
        }
    };

    println!("The result is {result}");

    //loop labels and nested loops
    let mut count = 0;
    'counting_up: loop { //loop label with which we can define which loop to break/continue in case of nested loops
        println!("count = {count}");
        let mut remaining = 10;

        loop {
            println!("remaining = {remaining}");
            if remaining == 9 {
                break;
            }
            if count == 2 {
                break 'counting_up; //normally this would break the inner loop, but with the label it wil break the "main"
            }
            remaining -= 1;
        }

        count += 1;
    }
    println!("End count = {count}");

    //while
    while number != 0 {
        println!("{number}!");

        number -= 1;
    }

    println!("LIFTOFF!!!");

    let a = [10, 20, 30, 40, 50];

    for element in a {
        println!("the value is: {element}");
    }

    //the above while loop implemented with for
    for number in (1..4).rev() {
        println!("{number}");
    }
    println!("LIFTOFF!!!");
}