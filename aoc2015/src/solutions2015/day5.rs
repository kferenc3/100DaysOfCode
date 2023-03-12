use fancy_regex::Regex;

pub fn day5solver(i: &str) -> (String, String){
    (part_i(i).to_string(), part_ii(i).to_string())
}

fn part_i(i: &str) -> i32 {
    
    let lines = i.lines();
    
    let vowels = Regex::new(r"[aeiou]").unwrap();
    let nono = Regex::new(r"ab|cd|pq|xy").unwrap();
    let dupes = Regex::new(r"(.)\1").unwrap();
    let mut nice = 0;
    
    for line in lines {
        let vowel_count = vowels.captures_iter(line).count() >= 3;
    
        let nono_count = nono.captures_iter(line).count() == 0;
    
        let duplicate_count = dupes.captures_iter(line).count() >= 1;

        if vowel_count && nono_count && duplicate_count {
            nice += 1;
        }

    }

    nice

}

fn part_ii(i: &str) -> i32 {
        
    let lines = i.lines();
    
    let r1 = Regex::new(r"(..).*\1").unwrap();
    let r2 = Regex::new(r"(.).\1").unwrap();
    let mut nice = 0;
    
    for line in lines {
        let r1_count = r1.captures_iter(line).count() >= 1;
    
        let r2_count = r2.captures_iter(line).count() >= 1;

        if r1_count && r2_count {
            nice += 1;
        }

    }

    nice

}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn day3test1() {
        assert_eq!(1,part_i(&String::from("ugknbfddgicrmopn\n")));
        assert_eq!(1,part_ii(&String::from("qjhvhtzxzqqjkmpb\n")));   
    }

    #[test]
    fn day3test2() {
        assert_eq!(0,part_i(&String::from("jchzalrnumimnmhp\n")));
        assert_eq!(0,part_ii(&String::from("uurcxstgmygtbstg\n")));    
    }

}