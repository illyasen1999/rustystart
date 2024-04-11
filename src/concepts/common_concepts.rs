// The Book: Chapter 3
use std::io;
// use std::{thread, time};

#[warn(unused_variables)]
pub fn common_concepts(){
    println!("Topic: Common Concepts");

    // Sub-topic: VARIABLES AND MUTABILITY

    // normal variable can have the type annotated automatically
    // immutable - cant be modified
    let a = 3;
    // a = 4; // error
    println!("a is {}", a);

    // mutable - can be modified
    let mut b = 3;
    println!("b is {}", b);
    b = 4; // with "mut" b can change value
    println!("b is {}", b);

    // constants - cant be changed and names must be uppercase and has the type annotated
    const PIE_VALUE: f32 = 3.14;
    println!("PIE: {}", PIE_VALUE);

    const SPEED_OF_LIGHT: i32 = 299_792_458;
    println!("Speed of Light: {}", SPEED_OF_LIGHT);

    // shadowing - using the name of the previous variable to be turned into something else ex. String to Integer or other things and can create a new variable
    let c = 3;
    let c = c + 10;
    println!("C: {}", c);
    {
        let c = c * 20;
        println!("Inner scope C: {}", c);
    }
    println!("Outer scope C: {}", c);

    // shadowing can be used to transform "immutable" variables to another type unlike "mut"
    let space = "    ";
    let space = space.len();
    println!("Spaces: {}", space);

    // let mut mut_space = "    ";
    // mut_space = mut_space.len(); // returns a usize type instead of the &str type

    // Sub-topic: DATA TYPES
    // Scalar and Compound types

    // Scalar - single value (int, float, boolean, characters)
    // Integer - number w/o fraction
    // - can be signed(i): positive and negative nums
    // - can be unsigned(u): only positive nums 
    // Length	Signed	Unsigned
    // 8-bit	i8	    u8
    // 16-bit	i16	    u16
    // 32-bit	i32	    u32
    // 64-bit	i64	    u64
    // 128-bit	i128	u128
    // arch	isize	usize

    // NOTE: The underscore(_) before the name tells the compiler to ignore even if its not used
    let _some_int: i32 = 32;
    
    // Float - floating-point numbers all floats are signed
    // f32(single-percision) and f64(double-percision)
    let _some_float_default = 3.144;
    let _some_float: f32 = 6.4;

    // Numeric Operators: add(+), sub(-), mult(*), div(/), remainder(%)
    let _add = 1 + 2;
    let _sub = 2 - 1;
    let _mult = 2 * 4;
    let _div = 10 / 5;
    let _remainder = 43 % 5;

    // Boolean - true or false, usefull for conditions, 1 byte size
    let _is_online = false;
    let _is_offline: bool = true;

    // Character - a single character, can be letters, or emojis and is 4 bytes size
    let _small_a = 'a';
    let _large_b: char = 'B';
    let _heart_emoji = '❤';
    println!("Heart: {}", _heart_emoji);

    // Compound - group multiple values into one type
    // tuples and arrays

    // Tuple - general way of grouping things, and have a fixed size, can have values of the same and different types, tuples cant grow or shrink in size
    let _tup: (i32, f64, bool) = (32, 32.5, false);
    let _tup2 = ("first", 'A', true);
    // the "{:?}" syntax can give the values of a collection, to output them individually you need to destructure the collection
    println!("{:?}", _tup);

    // values in a tuple can also be accessed with out destructuring them by using their indexes accessed through the dot(.) syntax
    println!("_tup first value: {}", _tup.0);

    // Destructuring - getting individual values inside a collection
    let (_x, _y, _z) = _tup;
    println!("x: {}, y: {}, z: {}", _x, _y, _z);

    const PHIL_COORDS: (f64, f64) = (12.8797, 121.7740);
    let (north, east) = &PHIL_COORDS;
    println!("Philippine Coordinates: {}(N), {:.4}(E)", &north, &east); // the "{:.4} format ensures that the 4 digits after the decimal point is printed"

    // Array - another way of grouping things, every element has the same type, arrays in Rust have a fixed length, values are stored in square brackets([])
    let _list_of_numbers = [1, 4, 10, 20, 33, 800];

    println!("{:?}", &_list_of_numbers);

    // arrays are more usefull when you know the number of elements inside the array
    let _months = ["January", "February", "March", "April", "May", "June", "July", "August", "September", "October", "November", "December"];
    println!("List of months: {:?}", _months);

    let _nums: [i32; 5] = [1, 2, 3, 4, 5]; // explicit type and size
    let _all_threes = [3; 5]; // this list has 5 units of the number 3

    // accessing array elements - access an element in an array using indexes that start at 0
    let _first_el = _nums[0];
    let _second_el = _nums[1];
    println!("first el: {}, second el: {}", &_first_el, &_second_el);

    // Sub-topic: Functions

    let res = run_function(32); // a function being called to be executed
    println!("Returned value: {}", res);

    // the "main" function is the most important function in Rust because its the entry point of Rust

    // Statements and Expressions
    // Statements - are instructions that perform some action and do not return a value.
    // Expressions - evaluate to a resultant value.
    let _aa = 3; // this is both a statement and an expression

    // Sub-topic: Control Flow
    // run code based on conditions

    // if expressions
    let is_online = true;
    if is_online { // conditions must be booleans(true or false)
        println!("User is Online");
    }
    else {
        println!("User is Offline");
    }

    // else-if expressions - for multiple conditions
    let number = 6;

    if number % 4 == 0 {
        println!("number is divisible by 4");
    } else if number % 3 == 0 {
        println!("number is divisible by 3");
    } else if number % 2 == 0 {
        println!("number is divisible by 2");
    } else {
        println!("number is not divisible by 4, 3, or 2");
    }

    // let-if statement - assign a value based on the outcome
    // in "let-if" potential values returned must be the same as each arm of the "if" statement else it will error
    let status = if !is_online { "Im online🟢" } else { "Im offline🔴" };
    println!("My internet status: {}", &status);

    // loops - continue executing code until you declare a break point or meet a condition
    // - loop, while, for 

    // loop
    let mut counter = 0;
    // let seconds = time::Duration::from_millis(1000);
    // loop {
    //     counter += 1;
    //     println!("Count: {}", &counter);
    //     if counter == 10 {
    //         break; // breaks out of the loop
    //     }
    // }

    // return values from loops

    let value_doubled_in_ten_seconds = loop {
        counter += 1;
        // thread::sleep(seconds);
        println!("Counter: {}", &counter);
        if counter == 3 {
            // break can return a value but only when break is used in the "loop" and not other loops
            break counter * 2;
        }
    };
    
    println!("Value: {}", &value_doubled_in_ten_seconds);
    
    // while loops - while a condition is not met continue to execute code
    let mut eta = 3;
    while eta != 0 {
        println!("ETA: {}", &eta);
        // thread::sleep(seconds);
        eta -= 1;
    }
    println!("Liftoff");

    // for loop - looping through a collection without hardcoding the size of the collection 
    for element in _nums {
        println!("Element: {}", element);
    }

    range_based_for_loop(15);
}

// an example of a function, declared by using the "fn" keyword and giving it a name
// a function can have a "return" type(ex. fn number() -> i32 {}) meaning it can produce an output or just run some code
// a function can also have parameters/arguments(values) inside parenthesis() that you need to provide in order for the function to run
fn run_function(number: i32) -> i32 {
    println!("This code is ran by a function");
    let returned_number = number;
    return returned_number
}

fn range_based_for_loop(_n: i32){
    println!("FizzBuzz");
    // range..based for loop
    for _n in 1..=15{
        if _n % 5 == 0 && _n % 3 == 0 {
            println!("{}: fizzbuzz", _n);
        }
        else if _n % 5 == 0 {
            println!("{}: buzz", _n);
        }
        else if _n % 3 == 0 {
            println!("{}: fizz", _n);
        }
        else {
            println!("{}", _n);
        }
    }
}

fn _invalid_array_access(){
    let my_list = [64, 10, 30, 5, 98];
    println!("My List: {:?}", &my_list);

    println!("Enter an index(0-4): ");
    let mut access_element = String::new();

    io::stdin()
        .read_line(&mut access_element)
        .expect("Cant read input");

    let access_element: usize = access_element
    .trim()
    .parse()
    .expect("Input was no a number");

    let element = &my_list[access_element];
    println!("You've accessed {}", element);
    println!("If you tried to access a 6th element and beyond it will give you an out-of-bounds error");
}