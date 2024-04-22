use std::vec;

// Generics in Structs
// the Generic type can be used in one or more fields in the struct
// adding a Generic type is the same as adding it in a function
struct Point<T> {
    x: T,
    y: T,
}

// Generics in Methods
// generics can also be added in methods for structs and the generic signature is placed after the "impl"
impl<T> Point<T> {
    fn x_point(&self) -> &T {
        return &self.x
    }
    fn y_point(&self) -> &T {
        return &self.y
    }
}

// specifiying cosnstraints on generic types
// methods here only apply to i32 integers
impl Point<i32> {
    fn raise_pow_two(&self) -> i32 {
        return self.x.pow(2)
    }
}


// Multiple Generic Types - this struct can use 2 different kinds of types
struct OtherPoint<T, U> {
    a: T,
    b: U,
}

impl<T, U> OtherPoint<T, U> {
    fn mixup<V, W>(self, other: OtherPoint<V, W>) -> OtherPoint<T, W> {
        OtherPoint {
            a: self.a,
            b: other.b
        }
    }
}

// Generics in Enums - its the same with Structs you can add generics in enums - best ex. Option enum
#[derive(Debug)]
enum TestOption<T> {
    TestSome(T),
    TestNone,
}

// enums can also have multiple generics best ex. Result enum
#[derive(Debug)]
enum TestResult<T, E> {
    TestOk(T),
    TestErr(E),
}

pub fn rust_generics() {
    println!("Rust Big Topic: Generics");

    // Sub-topic: Generics - abstract stand-ins for concrete types or other properties

    // functions can take parameters of some generic type, istead of a concrete type like i32 or String

    // ex. Option<T> Vec<T> HashMap<K, V> and Result<T, E>

    // Removing duplication by extracting a function
    // Generics allows us to replace specific types with a placeholder that represents multiple types to remove code duplication

    // first w/o Generics
    let number_list = vec![34, 50, 25, 100, 65];
    // let mut largest = &number_list[0];

    // for number in &number_list {
    //     if number > largest {
    //         largest = number
    //     }
    // }

    // println!("Largest: {}", largest);

    largest_i32(&number_list);

    // 2nd list to find the largest number
    let number_list = vec![102, 34, 6000, 89, 54, 2, 43, 8];
    // let mut largest = &number_list[0];

    // for number in &number_list {
    //     if number > largest {
    //         largest = number
    //     }
    // }

    // println!("Largest: {}", largest);

    // to avoid this we create a function that operates on any list of integers

    largest_i32(&number_list);

    let char_list = vec!['y', 'm', 'a', 'q'];

    largest_char(&char_list);

    // Now using Generics
    // Generics allow code to operate on abstract types

    println!("\nUsing Generics with <T>");
    let int_res = _largest(&number_list);
    println!("Largest item: {:?}", &int_res);

    let char_res = _largest(&char_list);
    println!("Largest item: {:?}", &char_res);

    // this works because the fields use the same type
    let _point_int = Point { x: 25, y: 50 };
    let _point_float = Point { x: 3.14, y: 1.99 };

    println!("Int X Point: {}", _point_int.x_point());
    println!("X to the power of 2: {}", _point_int.raise_pow_two());
    println!("Float Y Point: {}", _point_float.y_point());

    // this wont work because the fields are different types
    // let _point_broken = Point { x: 12, y: 1.12 };

    let _other_point_variable = OtherPoint { a: 23, b: 'B', };

    let _test_enum = TestOption::TestSome(String::from("Test Option"));

    println!("Test: {:?}", &_test_enum);

    let _test_enum_2 = vec![ TestResult::TestOk(true), TestResult::TestErr(String::from("Error")) ];

    println!("{:?}, {:?}", _test_enum_2[0], _test_enum_2[1]);

    let p1 = OtherPoint { a: 5, b: 10.4 };
    let p2 = OtherPoint { a: "Hello", b: 'c' };

    let p3 = p1.mixup(p2);

    println!("p3.x = {}, p3.y = {}", p3.a, p3.b);

    // Performance on using Generics
    /*
        - using generics doesnt make the program slower
        - Rust uses Monomorphization(turning generic code into specific code)

        https://doc.rust-lang.org/book/ch10-01-syntax.html#performance-of-code-using-generics
    */

}

fn largest_i32(list: &[i32]) -> &i32 {
    let mut largest = &list[0];

    for item in list {
        if item > largest {
            largest = item
        }
    }

    println!("Largest i32: {}", largest);
    return largest
}

fn largest_char(list: &[char]) -> &char {
    let mut largest = &list[0];

    for item in list {
        if item > largest {
            largest = item
        }
    }

    println!("Largest char: {}", largest);
    return largest
}

// Generics in function definitions
// to add a generic type to a function add the angle brackets(<>) after the function name and the letter "T" which indicates as a type, you can use any letter or name you want but the letter "T" for type is Rust's naming convention
// these traits make the generic restricted to any type that can be compared
fn _largest<T: PartialOrd + Copy>(list: &[T]) -> &T {
    let mut largest = &list[0];

    for item in list {
        if item > largest { // this wont compile because of an error here which requires a trait
            largest = item
        }
    }
    // println!("Largest item: {:?}", largest);
    return largest
}