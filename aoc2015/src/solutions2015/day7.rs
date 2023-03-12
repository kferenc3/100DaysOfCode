use std::collections::HashMap;
use regex::Regex;

pub fn day7solver(i: &str) -> (String, String){
    (part_i(i, "a").to_string(), part_ii(i, "a").to_string())
}

fn part_i(i: &str, goal: &str) -> u16 {
    signal_calculator(i, goal, None)
}
fn part_ii(i: &str, goal: &str) -> u16 {
    let b = signal_calculator(i, goal, None);
    let mut v: HashMap<&str, Option<u16>> = HashMap::new();
    v.insert("b", Some(b));
    signal_calculator(i, goal, Some(v))
}

fn signal_calculator(i: &str, goal: &str, v: Option<HashMap<&str, Option<u16>>>) -> u16 {
    
    let mut valuemap = match v {
        Some(i) => i,
        None => HashMap::new(),
    };
    
    valuemap.insert(goal, None);

    

    let re5 = Regex::new(r"^(?P<var1>\w+)\s+(?P<op>[A-Z]+)\s+(?P<var2>\w+)\s+->\s+(?P<dest>\w+)").unwrap();
    let re4 = Regex::new(r"^(?P<op>[A-Z]+)\s+(?P<var1>\w+)\s+->\s+(?P<dest>\w+)").unwrap();
    let re3 = Regex::new(r"^(?P<var1>\w+)\s+->\s+(?P<dest>\w+)").unwrap();

    let mut var1 = "";
    let mut var2 = "";
    let mut var1_num: u16 = 0;
    let mut var2_num: u16 = 0;
    let mut op = "";
    let mut dest = "";

    while valuemap[goal].is_none() {
        for l in i.lines() {
            let line: Vec<&str> = l.split(' ').collect();

            if line.len() == 3 {
                let captures = re3.captures(l).unwrap();
                var1 = captures.name("var1").unwrap().as_str();
                dest = captures.name("dest").unwrap().as_str();
                if dest != "b" || !valuemap.contains_key(dest) {
                    match var1.parse::<u16>() {
                        Ok(i) => valuemap.insert(dest, Some(i)),
                        Err(_) => valuemap.insert(dest, None),
                    };    
                }; 
            } else if line.len() == 4 {
                let captures = re4.captures(l).unwrap();
                var1 = captures.name("var1").unwrap().as_str();
                dest = captures.name("dest").unwrap().as_str();
                op = captures.name("op").unwrap().as_str();
                valuemap.insert(dest, None);
            } else if line.len() == 5 {
                let captures = re5.captures(l).unwrap();
                var1 = captures.name("var1").unwrap().as_str();
                var2 = captures.name("var2").unwrap().as_str();
                dest = captures.name("dest").unwrap().as_str();
                op = captures.name("op").unwrap().as_str();
                valuemap.insert(dest, None);
            }

            var1_num += var1.parse::<u16>().unwrap_or(0);
            var2_num += var2.parse::<u16>().unwrap_or(0);

            if op.is_empty() && var2.is_empty() && valuemap.contains_key(var1) {
                match valuemap[var1] {
                    Some(i) => valuemap.insert(dest, Some(i)),
                    None => None,
                };

            } else if var2.is_empty() && valuemap.contains_key(var1) && op == "NOT" {
                match valuemap[var1] {
                    Some(i) => valuemap.insert(dest, Some(!i)),
                    None => None,
                };
            } else if var1_num != 0 && valuemap.contains_key(var2) && valuemap[var2].is_some() {
                valuemap = ops_match(op, dest, var1_num, valuemap[var2].unwrap(), valuemap)

            } else if valuemap.contains_key(var1) && valuemap[var1].is_some() && var2_num != 0 {
                valuemap = ops_match(op, dest, valuemap[var1].unwrap(), var2_num, valuemap)

            } else if valuemap.contains_key(var1) && valuemap[var1].is_some() && valuemap.contains_key(var2) && valuemap[var2].is_some() {
                valuemap = ops_match(op, dest, valuemap[var1].unwrap(), valuemap[var2].unwrap(), valuemap)

            };
            
            var1 = "";
            var2 = "";
            var1_num = 0;
            var2_num = 0;
            op = "";
            dest = "";
                
        }
        
    }
    valuemap[goal].unwrap()
}

fn ops_match<'a>(op: &'a str, d: &'a str, v1: u16, v2: u16,  mut v: HashMap<&'a str, Option<u16>>) -> HashMap<&'a str, Option<u16>> {
    match op {
        "AND" => v.insert(d, Some(v1 & v2)),
        "OR" => v.insert(d, Some(v1 | v2)),
        "LSHIFT" => v.insert(d, Some(v1 << v2)),
        "RSHIFT"  => v.insert(d, Some(v1 >> v2)),
        _ => None,
    };
    v
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn day7test1() {
        assert_eq!(65412,part_i(&String::from("123 -> x\n456 -> y\nx AND y -> d\nx OR y -> e\nx LSHIFT 2 -> f\ny RSHIFT 2 -> g\nNOT x -> h\nNOT y -> i\n"), "h"));
        //assert_eq!(1000000,part_ii(&String::from("turn on 0,0 through 999,999\n")));
    }

}
