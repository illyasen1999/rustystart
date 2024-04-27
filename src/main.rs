use std::error::Error;
use std::fs::File;

// mod test;
// mod miniprojects;
// mod concepts;
// mod ownership;
// mod structure;
// mod manage;
// mod collections;
// mod errhandling;
mod bigtopics;

// Box<> is a trait object(Chapter 17) for now in this case it means that "return any kind of Error"
fn main() -> Result<(), Box<dyn Error>> {
    // miniprojects::guessing_game::guessing_game();
    // concepts::common_concepts::common_concepts();
    // ownership::ownborrow::ownborrow()
    // structure::structure::structures();
    // miniprojects::tempconv::tempconv();
    // manage::manage::manage();
    // collections::common_collections::common_collections();
    // miniprojects::list_of_names::list_of_names();
    // miniprojects::mememo::mememo();
    // errhandling::errhandling::errhandling();
    // errhandling::errhandling2::errhandling2();
    // bigtopics::rustgenerics::rust_generics();
    // bigtopics::rusttraits::rust_traits();
    bigtopics::rustlifetimes::rust_lifetimes();

    // the question mark(?) operator cannot be used in the "main" function due to restrictions like it can only return Nothing or it can return a Result type, it only works if the function returns a Result type 
    let _f = File::open("hello.txt")?;

    // TODO: https://rust-cli.github.io/book/index.html
    Ok(())
}
