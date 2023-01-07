//structs are custom data types like tuples, but with named fields (kind of json like or dicts in python)

pub struct User {
    pub active: bool,
    pub username: String,
    pub email: String,
    pub sign_in_count: u64
}

pub fn build_user(email: String, username: String) -> User {
    User {
        email, //this means the same as : email: email, --> if the fn param has the same name as the struct field 
        username,
        active: true,
        sign_in_count: 1,
    }
}

// () --> unit (technically an empty tuple)
// struct Something; --> unit-like Struct

pub struct Point(i32, i32, i32); //tuple struct. no named fields, just types with a custom naming. This will be a type on it's own, i.e this 
//would be a different type then Colo(i32, i32, i32)