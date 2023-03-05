use ndarray::Array2;

pub fn day6solver(i: &String) -> (String, String){
    (part_i(i, 1000).to_string(), part_ii(i, 1000).to_string())
}

#[derive(Debug)]
struct Instruction {
    toggle: u8,
    row_ft: (i32, i32),
    col_ft: (i32, i32),
}

impl Instruction {
    fn new(l: &str) -> Instruction {
        
        let toggle: u8 = match l.find("toggle") {
            Some(_) => 2,
            None => match l.find("on") {
                Some(_) => 1,
                None => 0,
            }
        };
        let line_content: Vec<&str> = l.split(' ').collect();

        let c1: Vec<&str> = if toggle == 2 {
            line_content[1].split(',').collect()
        } else {
            line_content[2].split(',').collect()
        };

        let c2: Vec<&str> = if toggle == 2 {
            line_content[3].split(',').collect()
        } else {
            line_content[4].split(',').collect()
        };
        
        
        Instruction {
            toggle: toggle,
            row_ft: (c1[0].parse::<i32>().unwrap(),c2[0].parse::<i32>().unwrap()),
            col_ft: (c1[1].parse::<i32>().unwrap(),c2[1].parse::<i32>().unwrap()),
        }
    }

    fn switch_lights(&self, v: Array2<i32>) -> Array2<i32> {

        let mut lights_arr = v;
        for ((r,c), x) in lights_arr.indexed_iter_mut() {
            if self.col_ft.0<=r as i32 && self.row_ft.0<=c as i32 && r as i32<=self.col_ft.1 && c as i32 <= self.row_ft.1 {
                if self.toggle == 1 {
                    *x = 1;
                } else if self.toggle == 0 {
                    *x = 0;
                } else if self.toggle == 2 {
                    if x == &1 {
                        *x = 0;
                    } else {
                        *x = 1;
                    }
                }
            }
        }

        lights_arr

    }

    fn adjust_brightness(&self, v: Array2<i32>) -> Array2<i32> {

        let mut lights_arr = v;
        for ((r,c), x) in lights_arr.indexed_iter_mut() {
            if self.col_ft.0<=r as i32 && self.row_ft.0<=c as i32 && r as i32<=self.col_ft.1 && c as i32 <= self.row_ft.1 {
                if self.toggle == 1 {
                    *x += 1;
                } else if self.toggle == 0 {
                    if x != &0 {
                        *x -= 1;
                    }
                } else if self.toggle == 2 {
                    *x += 2;
                }
            }
        }

        lights_arr

    }


}

fn part_i(i: &String, gridsize: usize) -> i32 {
    
    let mut grid = Array2::<i32>::zeros((gridsize,gridsize)); 

    for line in i.lines() {
        let instruction = Instruction::new(line);
        grid = instruction.switch_lights(grid);
    }
    grid.sum()
}


fn part_ii(i: &String, gridsize: usize) -> i32 {

    let mut grid = Array2::<i32>::zeros((gridsize,gridsize)); 

    for line in i.lines() {
        let instruction = Instruction::new(line);
        grid = instruction.adjust_brightness(grid);
    }
    grid.sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn day6test1() {
        assert_eq!(1000000,part_i(&String::from("turn on 0,0 through 999,999\n"),1000));
        assert_eq!(1000000,part_ii(&String::from("turn on 0,0 through 999,999\n"),1000));
    }

    #[test]
    fn day6test2() {
        assert_eq!(999000,part_i(&String::from("turn on 0,0 through 999,999\ntoggle 0,0 through 999,0\n"),1000));
        assert_eq!(1002000,part_ii(&String::from("turn on 0,0 through 999,999\ntoggle 0,0 through 999,0\n"),1000));
    }

     #[test]
    fn day6test3() {
        assert_eq!(999996,part_i(&String::from("turn on 0,0 through 999,999\nturn off 499,499 through 500,500\n"),1000));
        assert_eq!(999996,part_ii(&String::from("turn on 0,0 through 999,999\nturn off 499,499 through 500,500\n"),1000));
    }

}