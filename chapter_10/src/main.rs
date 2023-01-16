fn main() {
    
    //first finding the largest number without any generics. This way we would need to duplicate the code if we wanted to reproduce the result
    //so step 2 would be to extract this code into a function
    let number_list = vec![34, 50, 25, 100, 65];
    let char_list = vec!['q', 'z', 'a', 'h', 'm'];

    let mut lrgst = &number_list[0];

    for number in &number_list {
        if number > lrgst {
            lrgst = number;
        }
    }

    println!("The largest number calculated without a function {}", lrgst);
    println!("The largest number, but with a function: {}", largest(&number_list));
    println!("Largest number with generic types: {}", largest_gen(&number_list));
    println!("Largest char with generic types: {}", largest_gen(&char_list));

}

fn largest(list: &[i32]) -> &i32 { //however similarly to generic repeated code a function with a given type is also restrictive. 
    let mut largest = &list[0];    //E.g we need a similar function but it should take a list of chars.
    for item in list {             //In that case we would need the same function, but with different types. That is where generics come handy.
        if item > largest {
            largest = item;
        }
    }
    largest
}

fn largest_gen<T: std::cmp::PartialOrd>(list: &[T]) -> &T { //this syntax tells rust to accept a list (no matter the type in them) and return the type reference
    let mut largest = &list[0]; //this way it both accepts char and integer lists as well
    for item in list {
        if item > largest {
            largest = item;
        }
    }
    largest
}

struct Point<T> { //structs can also have generic types. However this still doesn't mean that we can mix the types inside the struct
    x: T,
    y: T,
}

enum Result<T, E> { //same with enum. btw T and E are placeholders. I can use anything e.g A and B. But T and E are convention.
    Ok(T),
    Err(E),
}

//a method with a generic type on the previously defined struct
impl<T> Point<T> {
    fn x(&self) -> &T {
        &self.x
    }
}

impl Point<f32> { //here we restricted the method to a specific type. The struct still accepts generict, but this method only works on f32
    fn distance_from_origin(&self) -> f32 {
        (self.x.powi(2) + self.y.powi(2)).sqrt()
    }
}
