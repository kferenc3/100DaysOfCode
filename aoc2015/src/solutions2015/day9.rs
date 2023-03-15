use std::{str, collections::{HashMap, HashSet}};
use itertools::Itertools;

pub fn day9solver(i: &str) -> (String, String){
    (part_i(i).to_string(), part_ii(i).to_string())
}

fn part_i(i: &str) -> i32 {

    let distances = dist_calc(routesbuilder(i), cityvec(i));
    distances[0]

}

fn part_ii(i: &str) -> i32 {
    
    let mut distances = dist_calc(routesbuilder(i), cityvec(i));
    distances.pop().unwrap()
    
    }

fn dist_calc(r: HashMap<Route, i32>, c: Vec<&str>) -> Vec<i32> {
    let mut dist: Vec<i32> = vec![];
    let win = c.len();
    for p in c.into_iter().permutations(win) {
        let mut d = 0;
        let rt = p.windows(2).collect::<Vec<&[&str]>>();
        for route in rt {
            d += r.get(&Route{from: route[0], to: route[1]}).unwrap_or(&1000);
        }
        dist.push(d);
    }

dist.sort();
dist
}


fn routesbuilder(i: &str) -> HashMap<Route, i32> {
    let mut routes: HashMap<Route, i32> = HashMap::new();
    
    let v = i.lines().map(|l| l.split(" to ").collect::<Vec<&str>>());
    
    for x in v {
        let start = x[0];
        let end = x[1].split(" = ").collect::<Vec<&str>>()[0];
        let distance = x[1].split(" = ").collect::<Vec<&str>>()[1].parse::<i32>().unwrap_or(0);
        routes.insert(Route{from: start, to: end}, distance);
        routes.insert(Route{from: end, to: start}, distance);
    }

    routes    

}

fn cityvec(i: &str) -> Vec<&str>{
    let mut cities: Vec<&str> = vec![];

    let v = i.lines().map(|l| l.split(" to ").collect::<Vec<&str>>());
    for x in v {
        let start = x[0];
        let end = x[1].split(" = ").collect::<Vec<&str>>()[0];
        cities.push(start);
        cities.push(end);
    }

    let set: HashSet<_> = cities.drain(..).collect();
    cities.extend(set.into_iter());
    cities
}


#[derive(Debug, PartialEq, Eq, Hash)]
struct Route<'a> {
    from: &'a str,
    to: &'a str,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn day9test1() {
        assert_eq!(605,part_i(&std::fs::read_to_string("inputdata_day9test").expect("errorka")));
        assert_eq!(982,part_ii(&std::fs::read_to_string("inputdata_day9test").expect("errorka")));
    }

}