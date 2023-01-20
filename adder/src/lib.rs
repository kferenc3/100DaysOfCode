pub fn add(left: usize, right: usize) -> usize {
    left + 3//right
}

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]       //test annotation with this rust will know to use this as a test when we run "cargo test"
    fn exploration() {
        //let result = add(2, 2);
        assert_eq!(add(2,2), 4);
    }

    //#[test]
    //fn another() {
    //    panic!("Make this test fail!"); //we can specify explicitly the test to panic in certain results
    //}
    #[test]
    fn larger_can_hold_smaller() {
        let larger = Rectangle {
            width: 8,
            height: 7,
        };
        let smaller = Rectangle {
            width: 5,
            height: 1,
        };

        assert!(larger.can_hold(&smaller));  //assert will translate the expresion to a boolean. True will pass the test, false will panic
    }

     #[test]
    fn smaller_cannot_hold_larger() {
        let larger = Rectangle {
            width: 8,
            height: 7,
        };
        let smaller = Rectangle {
            width: 5,
            height: 1,
        };

        assert!(!smaller.can_hold(&larger));
    }

    //other useful test macros: assert_eq!(), assert_ne!(). these are also displaying the values if the test fails
    // assert macros take custom error messages as an argument so the test failures can be customized as well
    // by adding #[should_panic] (goes after #[test]) we can test for error handling and make sure that our code indeed panics if it should
    // should_panic also accepts an expected argument to specify the scenario:
    // #[should_panic(expected = "less than or equal to 100")]
}
