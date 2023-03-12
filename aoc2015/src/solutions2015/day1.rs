pub fn day1solver(i: &str) -> (String, String){
    (part_i(i).to_string(), part_ii(i).to_string())
}

fn part_i(i: &str) -> i32 {
    let mut f = 0;

    for c in i.chars() {
        match c {
        '(' => f += 1,
        ')' => f -= 1,
        _ => f += 0,
    };}

    f
}

fn part_ii(i: &str) -> i32 {
    let mut f = 0;
    let mut counter = 1;

    for c in i.chars() {
        match c {
        '(' => f += 1,
        ')' => f -= 1,
        _ => f += 0,
        };
        if f == -1 {
            return counter
        }
        counter += 1;
    }

    counter
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn day1test1() {
        assert_eq!(0,part_i(&String::from("(())")));
        assert_eq!(1,part_ii(&String::from(")")));   
    }

    #[test]
    fn day1test2() {
        assert_eq!(3,part_i(&String::from("(()(()(")));
        assert_eq!(5,part_ii(&String::from("()())")));    
    }

    #[test]
    fn day1test3() {
        assert_eq!(-3,part_i(&String::from(")())())")));
        assert_eq!(1,part_ii(&String::from(")())())")));    
    }
}