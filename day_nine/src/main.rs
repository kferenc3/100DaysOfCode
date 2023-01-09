pub mod dices; //modules can be declared directly in the binary

fn main() {
    //Continuing the match syntax from yesterday.
    //Since the match has to be exhaustive we can define catch-all patterns. E.g
    let dice_roll = 9;
    match dice_roll {
        3 => dices::add_fancy_hat(),
        7 => dices::remove_fancy_hat(),
        other => dices::move_player(other), //catch-all for all other possibilities. This is not a key-word, but rather we just named a variable called other that will contain anything else other that 3 and 7
    }
//if we don't want to use the other values to anything we could simply use _. e.g:
//_ => reroll_dice(),

let config_max = Option::Some(3u8);
if let Option::Some(max) = config_max { //if let is a more concise way to handle matches where we only care about one arm of the match
    println!("The maximum is configured to be {}", max);
}

//The above is equivalent to this:
//match config_max {
//    Option::Some(max) => println!("The maximum is configured to be {}", max),
//    _ => (),
//}

//a public struct fields are still private and has to be made public explicitly
//a public enum's variants are going to be also public

}
