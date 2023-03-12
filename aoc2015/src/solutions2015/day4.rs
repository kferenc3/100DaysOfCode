//original solution with just the MD5 crate
//alternative solution using rust-crypto. Performance-wise both are quite bad, however the alternative solution look a bit nicer at least due to the built in methods.
extern crate crypto;

//use md5;
use crypto::md5::Md5;
use crypto::digest::Digest;


pub fn day4solver(i: &str) -> (String, String){
    (part_i_alter(i, Some(5)).to_string(), part_i_alter(i, Some(6)).to_string())
}

// fn part_i(i: &String, n: Option<usize>) -> i32 {
//     let nulls = match n {
//         Some(n) => n,
//         _ => 5,
//     };

//     let  mut d = 0;
//     loop {
//         let hashval = md5::compute(format!("{}{}", i.trim_end(), d.to_string()));
//         let hash_string = format!("{:X}", hashval);
//         if hash_string.starts_with(&"0".repeat(nulls)) {
//             return d
//         } else {
//             d += 1;
//         }
//     }
// }

fn part_i_alter(i: &str, n: Option<usize>) -> i32 {
    let nulls = match n {
        Some(n) => n,
        _ => 5,
    };

    let mut d = 0;
    let mut hash = Md5::new();
    loop {
        hash.input_str(&format!("{}{}", i.trim_end(), d));

        if hash.result_str().starts_with(&"0".repeat(nulls)) {
            return d;
        } else {
            hash.reset();
            d += 1;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn day4test1() {
        assert_eq!(609043,part_i_alter(&String::from("abcdef\n"), Some(5)));
    }

    #[test]
    fn day4test2() {
        assert_eq!(1048970,part_i_alter(&String::from("pqrstuv\n"), Some(5)));
    }

}