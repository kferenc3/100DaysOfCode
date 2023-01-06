use day_six::slices::*;

fn main() {
    let mut s = String::from("hello world");
    
    let _word = first_word_v1(&s); //underscore signifies unused variable (so the compiler doesn't complain)
    let _hello = &s[0..5]; //0 can be omitted [..5]
    let _world = &s[6..11]; //similarly if slicing until the end of the string 11 can be omitted [6..]
    println!("{}",first_word_v2(&s));

    //s.clear(); <--this would empty the string
    //The problem with the first_word_v1 approach is that it works on a reference of s. It gives back the index, however if s changes
    //like on line 10 then the result will still be 5 making the operation invalid.
    //This can be solved with slicing
    let a = [1,2,3,4,5];
    let slice = &a[1..3]; //works on arrays as well!
    assert_eq!(slice, &[2, 3]);
    
}
