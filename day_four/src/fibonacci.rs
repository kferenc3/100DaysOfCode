//1, 2, 3, 4, 5, 6, 7, 8,  9,  10, 11, 12, 13,  14 
//0, 1, 1, 2, 3, 5, 8, 13, 21, 34, 55, 89, 144, 233
pub fn nth_value(n: i32) {
    let mut n_min_two: i32 = 0;
    let mut n_min_one: i32 = 1;
    let mut res: i32 = 0;

    for i in 1..n + 1 {
        if i <= 2 {
            res = 1
        } else {
            res = n_min_two + n_min_one;
            n_min_two = n_min_one;
            n_min_one = res;
        }
    }
    println!("{res}")
}