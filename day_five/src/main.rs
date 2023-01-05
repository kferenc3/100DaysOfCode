use day_five::memory_allocation::*;

fn main() {
    my_strings();
    //ownership
    let s = String::from("helló röfi"); //variable s is valid here

    takes_ownership(s); //since we pass s to the function it takes the ownership
                                    //Here as is no longer valid as the variable is stored on the heap
                                    //The same with a stack type would only create a copy and the variable would still be valid.

    let s1 = pass_ball();   //the function retunrs the String and passes ownership to the main

    let s2 = String::from("ball2"); //comes into scope

    let s3 = catch_and_pass_ball(s2); //here since s2 is moved it is now out of scope

    println!("{}", referencing_example(&s3)); //since we only passed a reference of s3 it still remained valid. 
                                            //Otherwise we would need to return from the function
    
    let mut s4 = String::from("ball"); //can only borrow mutable reference once, but go for multiple immutable
    change_with_ref(&mut s4);
    
    println!("{}",s4)

}


