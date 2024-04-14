// Explenation Video: https://www.youtube.com/watch?v=5RPXgDQrjio&list=PLai5B987bZ9CoVR-QEIN9foz4QCJ0H2Y8&index=7
// The Book Chapter: https://doc.rust-lang.org/book/ch07-00-managing-growing-projects-with-packages-crates-and-modules.html

// This is a library crate
// w/o the "pub" keyword the variables, functions, structs etc. are considered private

// bringing 2 items with the same name
// ex. theses modules have the same Result type
use std::fmt;
use std::io;

// bringing 2 items with the same name but renaming the other one
use std::fmt::Result;
use std::io::Result as IoResult; // renaming using the "as" keyword

// using the same result type but sepecifing the parent module
// using parent modules fmt:: and io::
fn _func1() -> fmt::Result { Ok(() )}
fn _funct2() -> io::Result<()> { Ok(()) } 

// using different names
fn _funct3() -> Result { Ok(()) } 
fn _funct4() -> IoResult<()> { Ok(()) } 

// defining modules using the "mod" keyword
pub mod front_of_house; // since all the things related to front_of_house is moved to another file we can add a semi-colon(;) to the end of the syntax since its refering to the file of the same name

mod back_of_house {
    pub struct Breakfast {
        // if you mark a struct public only the struct itself is public and not the fields you need to also specify the fields you wish to be public by using the "pub" keyword
        pub toast: String,
        _seasonal_fruit: String,
    }

    impl Breakfast {
        // even though the struct is public all methods of the struct is still considered private unless you make them public the same way as its fields
        pub fn _summer(toast: &str) -> Breakfast {
            return Breakfast {
                toast: String::from(toast),
                _seasonal_fruit: String::from("peaches"),
            }
        }
    }

    pub enum Appetizer {
        // if you mark an enum as public all the varialbes inside it is also public
        Soup,
        Salad,
    }
}

pub fn eat_at_restaurant_one() {
    // Absolute Path
    crate::front_of_house::hosting::add_to_waitlist();

    // Relative Path
    front_of_house::hosting::add_to_waitlist();
}

pub fn eat_at_restaurant_two() {
    let mut _meal = back_of_house::Breakfast::_summer("Brunt");

    _meal.toast = String::from("Wheat");
}

pub fn eat_at_restaurant_three() {
    let _order1 = back_of_house::Appetizer::Soup;
    let _order2 = back_of_house::Appetizer::Salad;
}

// bringing a module to scope by using the "use" keyword
// use crate::front_of_house::hosting; // Absolute Path
// use self::front_of_house::hosting; // Relative Path

// external code can reference this crate using the "pub" keyword
pub use self::front_of_house::hosting;
// since these traits have the same parent module we are going to use nested paths to further minimize the code and specially bringing many things from a module
use rand::Rng;
// use rand::CryptoRng;

// nested paths
// use rand::{ Rng, CryptoRng };
// use std::io::{ Self, Write }; // self refers to io:: path itself
// use std::io::*; // the star symbol(*) is calling all public items in the std::io module that can be used

pub fn eat_at_restaurant_four() {
    // w/o bringing the module into scope
    // front_of_house::hosting::add_to_waitlist();
    // front_of_house::hosting::add_to_waitlist();
    // front_of_house::hosting::add_to_waitlist();

    let secret_number = rand::thread_rng().gen_range(1..=10);
    println!("Random number: {}", secret_number);

    // w/ bringing the module into scope
    hosting::add_to_waitlist();
    hosting::add_to_waitlist();
    hosting::add_to_waitlist();
}