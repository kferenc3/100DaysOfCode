use std::str;

pub fn day8solver(i: &str) -> (String, String){
    (part_i(i).to_string(), part_ii(i).to_string())
}

//this is stolen from stackoverflow in great shame...
fn replace<T>(source: &[T], from: &[T], to: &[T]) -> Vec<T>
where
    T: Clone + PartialEq
    {
        let mut result = source.to_vec();
        let from_len = from.len();
        let to_len = to.len();

        let mut i = 0;
        while i + from_len <= result.len() {
            if result[i..].starts_with(from) {
                result.splice(i..i + from_len, to.iter().cloned());
                i += to_len;
            } else {
                i += 1;
            }
        }

        result
    }

fn part_i(i: &str) -> usize {
    let mut bytelength = 0;
    let mut charlen = 0;

    for line in i.lines() {
        
        let bytes = line.bytes().len();
        let mut b = line.as_bytes().to_owned();
        
        b = b.split_last().unwrap().1.to_vec();
        b = b.split_first().unwrap().1.to_vec();
        b = replace(&b, &[92u8, 92u8], &[35u8]);
        b = replace(&b, &[92u8, 34u8], &[35u8]);
        b = replace(&b, &[92u8, 120u8], &[33u8]);
        
        let mut cl = b.len();
        cl -= b.iter().filter(|x| x == &&33u8).count() * 2;
        bytelength += bytes;
        charlen += cl;
    }

    bytelength - charlen
}

fn part_ii(i: &str) -> usize {
    let mut bytelength = 0;
    let mut charlen = 0;

    for line in i.lines() {
        
        let bytes = line.bytes().len();
        let b = line.as_bytes();
        let mut cl = b.len();
        let quotes= b.iter().filter(|x| x == &&34).count() -2;
        let backslash = b.iter().filter(|x| x == &&92).count();

        cl += 4 + quotes + backslash;
        
        bytelength += bytes;
        charlen += cl;
    }
    charlen - bytelength}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn day8test1() {
        assert_eq!(6,part_i(&String::from(r#""\\xm\xay""#))); //20 - (20-2-3)
        assert_eq!(4,part_ii(&String::from(r#""""#)));
    }

    #[test]
    fn day8test2() {
        assert_eq!(12,part_i(&std::fs::read_to_string("inputdata_day8test").expect("Unable to read file")));
        assert_eq!(6,part_ii(&String::from(r#""aaa\"aaa""#)));
    }
}