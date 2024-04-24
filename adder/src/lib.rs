// #[derive(Debug)]
// pub struct Rectangle {
//     width: i32,
//     height: i32,
// }

// impl Rectangle {
//     fn can_hold(&self, other: &Rectangle) -> bool {
//         return self.width > other.width && self.height > other.height
//     }
// }

pub fn add_two(n: i32) -> i32 {
    return n + 2
}

// pub fn greeting(name: &str) -> String {
//     // format!("Hello {}", name)
//     format!("Hello")
// }

// asserting that a function panics
pub struct Guess {
    value: i32,
}

impl Guess {
    pub fn new(value: i32) -> Guess {
        if value < 1 {
            panic!("Guess must be equal or greater than 1. Your value: {}", value);
        } 
        else if value > 100 {
            panic!("Guess must be equal or greater than 100. Your value: {}", value);
        }

        return Guess { value }
    }
}

// fn prints_and_returns_10(a: i32) -> i32 {
//     print!("Value: {}", a);
//     return 10
// }

// this function is public
pub fn add_two_v2(n: i32) -> i32 {
    return internal_adder(n, 2)
}

// this function is private
fn internal_adder(a: i32, b: i32) -> i32 {
    return a + b
}

#[cfg(test)]
mod tests {
    use super::*;

    // #[test] // functions are tests if they have the #[test] attribute
    // fn larger_can_hold_smaller() {
    //     let larger = Rectangle {
    //         width: 8,
    //         height: 7,
    //     };

    //     let smaller = Rectangle {
    //         width: 5,
    //         height: 2,
    //     };

    //     assert!(larger.can_hold(&smaller));
    // }

    // #[test]
    // fn smaller_cant_hold_larger() {
    //     let larger = Rectangle {
    //         width: 8,
    //         height: 7,
    //     };

    //     let smaller = Rectangle {
    //         width: 5,
    //         height: 2,
    //     };

    //     assert!(!smaller.can_hold(&larger));
    // }

    // #[test]
    // fn adds_two(){
    //     // assert macros
    //     // assert_eq!(4, add_two(2));
    //     assert_ne!(4, add_two(2));
    // }

    // tests with custom failure messages
    // #[test]
    // fn greeting_contains_name() {
    //     let res = greeting("carol");
    //     // using custom failure message in assert!
    //     assert!(
    //         res.contains("carol"),
    //         "Greeting did not contain name, value was {}",
    //         res,
    //     );
    // }

    #[test]
    #[should_panic(expected = "Guess must be equal or greater than 100")]
    fn greater_than_100(){
        Guess::new(200);
    }

    // test returning a Result type
    #[test]
    fn it_works() -> Result<(), String> {
        if 2 + 2 == 4 {
            Ok(())
        }
        else {
            Err(String::from("Error"))
        }
    }

    #[test]
    fn it_works_2() {
        assert_eq!(2 + 2, 4)
    }

    // on the cli use the "cargo test -- --show-ouput"
    // #[test]
    // fn test_pass() {
    //     let val = prints_and_returns_10(4);
    //     assert_eq!(10, val)
    // }

    // #[test]
    // fn test_fail() {
    //     let val = prints_and_returns_10(8);
    //     assert_eq!(5, val)
    // }

    // running a subset of tests
    // specify testing a function by passing it a part of its name ex. "cargo test add"
    #[test]
    fn add_two_and_two(){
        assert_eq!(4, add_two(2));
    }

    #[test]
    fn add_three_and_two(){
        assert_eq!(5, add_two(3));
    }

    // specify testing a function by passing it the name of the function ex. "cargo test hundred"
    #[test]
    fn hundred(){
        assert_eq!(102, add_two(100));
    }

    // ignoring test -  to run this test that is ignored write "cargo test -- --ignored" which only runs the ignored test
    #[test]
    #[ignore]
    fn expensive_test() {
        // some code that takes longer to run
    }

    // tests up to this point are all Unit Tests

    // Test Organization - 2 types: Unit tests, Integration tests
    // Unit Tests - small, focused, test one module in isolation and can test private interfaces
    // Integration Tests - completely external to your library and tests the public interface of your library

    #[test]
    fn internal() {
        assert_eq!(4, internal_adder(2, 2))
    }

}
