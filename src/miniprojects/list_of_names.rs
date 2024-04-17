use std::io;

pub fn list_of_names() {
    // Add names in a vector
    println!("A vector demo maybe?");
    let mut list: Vec<String> = Vec::new();
    let mut count = 0;

    loop {
        let mut user_input = String::new();

        let output = name_input(&mut user_input);

        // println!("{}", output); // for logging purposes
        list.push(output);

        count += 1;

        if count < 5 {
            continue;
        }
        else {
            break;
        }
    }

    println!("List of names: {:?}", list);


}

// TODO: Maybe just write the code in the loop and not require a function?
fn name_input(name: &mut String) -> String {
    let mut output = name;

    println!("Enter a name: ");
    io::stdin().read_line(&mut output).expect("Cant seem to read your value");

    let output: String = match output
    .trim()
    .parse() {
        Ok(string) => string,
        Err(_) => String::from("Not"),
    };

    return output;
}