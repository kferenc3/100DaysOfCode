pub fn day3solver(i: &str) -> (String, String){
    (part_i(i).to_string(), part_ii(i).to_string())
}

fn part_i(i: &str) -> i32 { 
    let mut santa = (0,0);
    let mut visited: Vec<(i32,i32)> = Vec::new();
    visited.push(santa);
    for c in i.chars() {
        match c {
            '>' => santa.1 += 1,
            '<' => santa.1 -= 1,
            '^' => santa.0 += 1,
            'v' => santa.0 -= 1,
            _ => break

        }
        visited.push(santa);
    }
    visited.sort();
    visited.dedup();
    visited.len() as i32
}

fn part_ii(i: &str) -> i32 {
    let mut santa = (0,0);
    let mut robo_santa = (0,0);
    let mut visited: Vec<(i32,i32)> = Vec::new();
    visited.push(santa);
    visited.push(robo_santa);

    let mut it = i.chars();
    loop {

        let santa_move = it.next();
        let robo_move = it.next();
        
        match santa_move {
        Some('>') => santa.1 += 1,
        Some('<') => santa.1 -= 1,
        Some('^') => santa.0 += 1,
        Some('v') => santa.0 -= 1,
        _ => break,
        }

        match robo_move {
        Some('>') => robo_santa.1 += 1,
        Some('<') => robo_santa.1 -= 1,
        Some('^') => robo_santa.0 += 1,
        Some('v') => robo_santa.0 -= 1,
        _ => break,
        }
       
       visited.push(santa);
       visited.push(robo_santa);
    }

    visited.sort();
    visited.dedup();
    visited.len() as i32
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn day3test1() {
        assert_eq!(4,part_i(&String::from("^>v<\n")));
        assert_eq!(3,part_ii(&String::from("^>v<\n")));   
    }

    #[test]
    fn day3test2() {
        assert_eq!(2,part_i(&String::from("^v^v^v^v^v\n")));
        assert_eq!(11,part_ii(&String::from("^v^v^v^v^v\n")));    
    }

}