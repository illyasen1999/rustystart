// FIXME: this "use" doesnt work
// use restaurant::front_of_house::hosting;

// FIXME: 
// this works but its not printing the output of add_to_waitlist()
use restaurant::hosting;

pub fn manage(){
    println!("Topic: Managing Growing Projects with Packages, Crates, and Modules");
    
    // Packages: A Cargo feature that lets you build, test, and share crates - a bundle of one or more crates that provides a set of functionality

    // Crates: A tree of modules that produces a library or executable - smallest amount of code
    /*
        Crate Roots:
        src/lib.rs - library crate
        src/main.rs - binary crate
    */ 

    // Modules and use: Let you control the organization, scope, and privacy of paths

    // Paths: A way of naming an item, such as a struct, function, or module

    hosting::add_to_waitlist(); 
}