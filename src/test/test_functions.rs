use std::io;

fn test_function() -> String {
    let mut my_name = String::new();

    println!("Enter your name: ");
    io::stdin()
    .read_line(&mut my_name)
    .expect("Reading failed");

    my_name.make_ascii_uppercase();

    // what???
    if my_name.contains("MIKHAIL") || my_name.contains("MIK") {
        println!("Your name is Mikhail");
    }
    else {
        println!("Your name is {}", &my_name);
    }

    println!("Testing function...");
    return my_name;
}