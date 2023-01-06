pub fn first_word_v1(s: &String) -> usize { //borrowing a reference
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return i; //returning an index only
        }
    }

    s.len()
}

pub fn first_word_v2(s: &str) -> &str { //signature str: more useful to write it like that as now it works with String and string literals as well without losing functionality
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i]; //returns the slice reference instead. 
            //And since the slice is linked to the original string the compiler will not allow clearing the string after (as that would make the reference invalid)
        }
    }

    &s[..] //returns the entire string
}