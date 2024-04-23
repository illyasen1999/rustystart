// Lifetimes in structs
// in the past we use Structs with owned data but when using a reference we use Lifetimes
// this Struct means that it cannot outlive the reference
struct ImportantExcerpt<'a> {
    // 
    part: &'a str,
}

// Lifetime Elision Rule 3
impl<'a> ImportantExcerpt<'a> {
    fn return_part(&self, announcment: &str) -> &str {
        println!("Attention please: {}", announcment);
        return &self.part
    }
}

// All of Chapter 10
use std::fmt::Display;

fn _longest_with_an_announcement<'a, T> (
    x: &'a str,
    y: &'a str,
    ann: T,
) -> &'a str where T: Display, {
    println!("Announcement: {}", ann);
    if x.len() > y.len() {
        return x
    }
    else {
        return y
    }
}

pub fn rust_lifetimes() {
    println!("Rust Big Topic: Lifetimes");

    // Sub-topic: Lifetimes

    // 1st example
    // both the lifetimes of string1 and string2 are equal so this is valid
    let string1 = String::from("abcd");
    let string2 = String::from("xyz");

    let res = longest(string1.as_str(), string2.as_str());
    println!("Longest: {}", res); // "res" wont be a dangling reference since we told the borrow checker that what ever is returned from "longest" will have the same lifetime as the thing that is being passed in the "longest" function

    // 2nd example
    // the lifetime of string2 and res is only until the final curly bracket in which string1 still has the longest lifetime
    let string1 = String::from("abcd");
    {
        let string2 = String::from("xyz");
        let res = longest(string1.as_str(), string2.as_str());
        println!("Longest: {}", res);
    }

    // 3rd example
    // the "res" inside the brackets only lasts with the smallest lifetime which is string2 and after that string2 is no longer valid
    // since the function "longest" only cares about the lifetime of its parameter "x" which is the string1 where its lifetime lasts until the end of the main function
    let string1 = String::from("abcd");
    let res;
    {
        let string2 = String::from("xyz");
        res = longest(string1.as_str(), string2.as_str());
    }
    println!("Longest: {}", res);

    let novel = String::from("Sword Art Online. Some years ago...");
    let first_sentence = novel.split('.').next().expect("Could not find sentence");
    let _i = ImportantExcerpt {
        part: first_sentence,
    };

    // static lifetime - the reference lives as long as the program
    let _s: &'static str = "I have a static lifetime";

}

// fn dangle() -> &String {
//     let s = String::from("Dangle");

//     return &s // this is a dangling reference
// }

// Generic lifetime annotations - are added by the <'a> syntax
// here it tells that "x", "y", and the return type are related to each other, it means here that the lifetime of the returned value is the same as the smallest lifetime of the arguments
// fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
//     if x.len() > y.len() {
//         return x
//     }
//     else {
//         return y
//     }
// }

// since this only returns "x" this function is valid
// fn longest<'a>(x: &'a str, y: &str) -> &'a str {
//     return x
// }

// can return a refernce that has been created internally because when the function ends, all its local variables are destroyed, which means you can only use a parameter casted into the function or an owned type
// fn longest<'a>(x: &str, y: &str) -> &'a str {
//     let res = String::from("really long string");
//     return res.as_str()
// }

// using a owned type which its returned variable ownership is transferablle
fn longest<'a>(x: &str, y: &str) -> String {
    let res = String::from("really long string");
    return res
}

// Lifetime Elision - the compiler can deterministically infer the lifetime annotations based on 3 rules
/*
    1. each param that is a refernce gets its own lifetime parameter

    2. if there is exactly one input lifetime param, that lifetime is assigned to all output params
    
    3. if there are multiple input lifetime parameters, but one of them is &self or &mut self the lifetime of self is assigned to all output lifetime parameters | this rule applies in methods of structs
*/

// lifetimes of arguments being passed are input lifetimes
// and lifetimes of returned values are output lifetimes
fn first_word<'a>(s: &'a str) -> &'a str { 
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    return &s[..]
}

// this can work even without lifetimes
fn _first_word(s: &str) -> &str { 
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    return &s[..]
}
