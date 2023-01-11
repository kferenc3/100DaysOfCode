use std::collections::HashMap;

fn main() {
    //Hash Maps
    let mut scores = HashMap::new();

    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);
    //also stored on the heap, similar to vectors
    let team_name = String::from("Blue");
    let score = scores.get(&team_name).copied().unwrap_or(0);  //extracting a value based on key

    //iterating over a hash map
    for (key, value) in &scores {
        println!("{key}: {value}")
    }

    //the ownership of items will depend on the trait of the type. e.g i32 will be copied, but a String will be moved into the hash map

    //updating. 3 options: discard old value, discard new value (leave old in place), combine values

    //simply updating:
    scores.insert(String::from("Blue"), 25); //we overwrote the previous 10

    //check if a value exist against a key and if not insert a value:
    scores.entry(String::from("Red")).or_insert(50); //entry checks whether there is a value or not (returns Entry enum)

    //updating a value based on old value
    let text = "hello world wonderful world";
    let mut map = HashMap::new();

    for word in text.split_whitespace() {
        let count = map.entry(word).or_insert(0);
        *count += 1;
    }

    println!("{:?}", map);


}
