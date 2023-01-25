use std::thread;

#[derive(Debug, PartialEq, Copy, Clone)]
enum ShirtColor {
    Red,
    Blue,
}

struct Inventory {
    shirts: Vec<ShirtColor>,
}

impl Inventory{
    fn giveaway(&self, user_preference: Option<ShirtColor>) -> ShirtColor {
        user_preference.unwrap_or_else(|| self.most_stocked())
    }

    fn most_stocked(&self) -> ShirtColor {
        let mut num_red = 0;
        let mut num_blue = 0;

        for color in &self.shirts{
            match color {
                ShirtColor::Red => num_red += 1,
                ShirtColor::Blue => num_blue += 1,
            }
        }
        
        if num_red > num_blue {
            ShirtColor::Red
        } else {
            ShirtColor::Blue
        }
    }
}


fn main() {
    let store = Inventory {
        shirts: vec![ShirtColor::Blue, ShirtColor::Red, ShirtColor::Blue],
    };

    let user_pref1 = Some(ShirtColor::Red);
    let giveaway1 = store.giveaway(user_pref1);
    println!(
        "The user with preference {:?} gets {:?}",
        user_pref1, giveaway1
    );

    let user_pref2 = None;
    let giveaway2 = store.giveaway(user_pref2);
    println!(
        "The user with preference {:?} gets {:?}",
        user_pref2, giveaway2
    );

    //closures will decide ownership based on the context and what operation they need to perform
    //to force ownership of the parameter we can use 'move' e.g: thread::spawn(move || println!("something"))

    let list = vec![1, 2, 3];
    println!("Before defining closure: {:?}", list);

    thread::spawn(move || println!("From thread: {:?}", list))
        .join()
        .unwrap();
}

//The below two are more or less equals in functionality. Closure captures its environment so doesn't need type annotation (can have, but not necessary).
//fn add_one(x: u32) -> u32 {x+1}
//let add_one_c = |x| x+1;

//closures will decide ownership based on the context and what operation they need to perform
//to force ownership of the parameter we can use 'move' e.g: thread::spawn(move || println!("something"))

//iterators:
//can be created with .iter / .iter_mut / .into_iter (moves owenership)

//.sum .next are consuming the iterator

// .map .filter --> creating new iterators from iterators
