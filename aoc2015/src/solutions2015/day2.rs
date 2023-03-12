pub fn day2solver(i: &str) -> (String, String){
    (part_i(i).to_string(), part_ii(i).to_string())
}

#[derive(Debug)]
struct Present {
    l: i32,
    w: i32,
    h: i32,
}

impl Present {
    fn new(length: i32, width: i32, height: i32) -> Present {
            Present{
                l: length,
                w: width,
                h: height,
            }
    }

    fn area(&self) -> i32 {
        2*(self.l * self.w) + 2*(self.w * self.h) +2*(self.h * self.l) 
    }

    fn smallest_side(&self) -> i32 {
        let sides = vec![self.l * self.w, self.w * self.h, self.h * self.l];
        
        let min_val = sides.iter().min();

        match min_val {
            Some(min) => *min,
            None => 0,
        }
    }

    fn short_edges(&self) -> i32 {
        let mut sides = Vec::new();
        sides.push(self.l); 
        sides.push(self.w);
        sides.push(self.h);

        let index = sides.iter().position(|y| y == sides.iter().max().unwrap()).unwrap();
        sides.swap_remove(index);

        sides.iter().sum::<i32>() * 2

    }

    fn volume(&self) -> i32 {
        self.l*self.w*self.h
    }

}

fn construct_cubes(i: &str) -> Vec<Present> {
    let nums = i.replace('\n', "x");
    let mut nums_iter = nums.split('x').into_iter().peekable();
    let mut cubes: Vec<Present> = Vec::new();

    loop {
    
        let l = match nums_iter.next() {
           Some(n) => n.parse::<i32>().unwrap(),
           None => 0,
        };
        let w = match nums_iter.next() {
            Some(n) => n.parse::<i32>().unwrap(),
            None => 0,
        };
        let h = match nums_iter.next() {
            Some(n) => n.parse::<i32>().unwrap(),
            None => 0,
        };
        
        cubes.push(Present::new(l, w, h));
        
        match nums_iter.peek() {
            Some(n) if n.is_empty() => break,
            Some(_) => {},
            None => break,
        }
    }

    cubes

}


fn part_i(i: &str) -> i32 {

    let c: Vec<Present> = construct_cubes(i);
    let mut wrapping_paper = 0;
   
        for p in c {
            wrapping_paper += p.area() + p.smallest_side()
        }
        wrapping_paper
        
    }

fn part_ii(i: &str) -> i32 {
    let c: Vec<Present> = construct_cubes(i);
    let mut ribbon = 0;

    for p in c {
        ribbon += p.volume() + p.short_edges()
    }
    ribbon
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn day2test1() {
        assert_eq!(58,part_i(&String::from("2x3x4\n")));
        assert_eq!(34,part_ii(&String::from("2x3x4\n")));   
    }

    #[test]
    fn day2test2() {
        assert_eq!(43,part_i(&String::from("1x1x10\n")));
        assert_eq!(14,part_ii(&String::from("1x1x10\n")));    
    }

}