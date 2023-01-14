use std::collections::HashMap;

//these are implementations of some of the practice functions suggested at the end of chapter 8 of the book
//I intend to do the text interface one as well. That will be most likely day 13.

fn main() {
    let mut median_vec = vec![583,72,896,280,829,456,256,325,916,50,810,218,135,634,634,286,216,30,814];
    let mut my_string = String::from("hello world");
    let mode_vec = vec![1,1,2,2,2,2,2,23,3,3,4];

    println!("{}", median_calc(&mut median_vec));
    println!("{}",pig_latin(&mut my_string));
    println!("{}", mode_calc(&mode_vec));
}

fn median_calc(v: &mut Vec<i32>) -> i32 {
    let vec_len = v.len();
    v.sort();
    if vec_len%2 == 0 {
        let c = (v[(vec_len/2)-1] + v[vec_len/2])/2;
        c
    } else {
        let c = v[(vec_len/2)+1];
        c
    }
}

fn pig_latin(s: &mut String) -> String {
    let vowels  = ['a','e','i','o','u'];
    let mut new_string = String::from("");
    
    
    for word in s.split_whitespace() {
        let first = word.chars().next();
        let mut new_word = String::from("");
        if vowels.contains(&first.expect("Oh no!")) {
            new_word.push_str(format!("{word}hay ").as_str());
        } else {
            new_word.push_str(remove_first(word));
            new_word.push_str(format!("{}{}",&first.expect("Oh no!").to_string(),"ay").as_str());
        }
        new_string.push_str(format!("{new_word} ").as_str())
    }
    new_string
}

fn remove_first(w: &str) -> &str {
    let mut chars = w.chars();
    chars.next();
    chars.as_str()
}

fn mode_calc(v: &Vec<i32>) -> i32 {
    let mut map = HashMap::new();
    for n in v {
        let count = map.entry(n).or_insert(0);
        *count += 1
    }

    let max_value = map.iter().max_by_key(|k| k.1).unwrap();
    **max_value.0
}