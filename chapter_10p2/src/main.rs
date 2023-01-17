use std::fmt::{Display, Debug};

fn main() {
    println!("Hello, world!");
}
//traits. A collection of methods that different types sharing. 
//If a trait is added to a type then that type has to have the mentioned method defined

pub trait Summary {
    fn summarize(&self) -> String; //in a trait we just list the methods and separate with a ;. Then the compiler will check if the type has this method defined or not.
}

pub trait AnotherSummary {
    fn summarize(&self) -> String {
        String::from("Default method behavior here!") //we can define a default implementation for traits as well.
    }
}

pub struct NewsArticle {
    pub headline: String,
    pub location: String,
    pub author: String,
    pub content: String,
}

impl Summary for NewsArticle {
    fn summarize(&self) -> String { //the signature of the method and the name of the trait is matching to the trait definition
        format!("{}, by {} ({})", self.headline, self.author, self.location)
    }
}

pub struct Tweet { 
    pub username: String,
    pub content: String,
    pub reply: bool,
    pub retweet: bool,
}

impl Summary for Tweet { //although the struct is different but it will also implement the summary trait with its own definition of summarize
    fn summarize(&self) -> String {
        format!("{}: {}", self.username, self.content)
    }
}

pub fn notify(item: &impl Summary) {
    println!("We defined a function that implements a trait! {}", item.summarize());
}

pub fn notify_v2(item: &(impl Summary + Display)) {} //implementing multiple traits

pub fn notify_v3<T: Summary + Display>(item: &T){} //trait bound syntax. Achieves the same as above, but forces a generic type

fn some_function<T, U>(t: &T, u: &U) -> i32 //in case multiple trait bounds and parameters need to be defined the following syntax can be used
where
    T: Display + Clone,
    U: Clone + Debug,
{3}

fn returns_a_trait() -> impl Summary {
    Tweet {
        username: String::from("horse_ebooks"),
        content: String::from(
            "of course, as you probably already know, people",
        ),
        reply: false,
        retweet: false,
    }
} //this way the return type will automatically implement the Summary trait (only works with a single return type)