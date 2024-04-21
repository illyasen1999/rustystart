// src: https://www.youtube.com/watch?v=wM6o70NAWUI&list=PLai5B987bZ9CoVR-QEIN9foz4QCJ0H2Y8&index=9
use std::fs::{self, File};
use std::io::{self, ErrorKind, Read};
use std::net::IpAddr;

pub fn errhandling2(){
    println!("Topic: Error Handling 2");

    let _file_path = "./src/errhandling/hello.txt";

    // let f = File::open(file_path); // File::open uses the Result enum by which this function might fail

    // handling the Success and the Error case using match
    // let f = match f {
    //     Ok(file) => file,
    //     Err(error) => panic!("Problem opening the file. {:?}", error)
    // };

    // Error Handling with Pattern Matching
    // let f = match f {
    //     Ok(file) => {
    //         println!("File opened");
    //         file
    //     },
    //     // create a new file instead of panicing
    //     Err(err) => match err.kind() {
    //         // creating a file still can fail so it also needs a matching case
    //         ErrorKind::NotFound => match File::create(file_path) {
    //             Ok(fc) => {
    //                 println!("File created");
    //                 fc
    //             },
    //             Err(e) => panic!("Problem creating the file {:?}", e)
    //         }
    //         // handling all other cases
    //         other_error => panic!("Problem opening the file. {:?}", other_error)
    //     }
    // };

    // using Closures(Chapter 13)
    // .unwrap_or_else() attempts to open a file/ gives back the file or passing in an error
    // if the file is not found attempt to create the file which can still result in an error
    // another .unwrap_or_else() is passed on the File::create which can give back the file or result to an error

    // let f = File::open(file_path)
    // .unwrap_or_else(|error| {
    //     if error.kind() == ErrorKind::NotFound {
    //         File::create(file_path).unwrap_or_else(|error| {
    //             panic!("Problem creating the file {:?}", error);
    //         })
    //     }
    //     else {
    //         panic!("Problem opening the file. {:?}", error);
    //     }
    // });

    // using .unwrap()
    // works the same with the code below which returns a file if successful or if it doesnt succeed it will panic!
    // let f = File::open(file_path).unwrap();

    // using .expect()
    // allows us to add an error message
    // let f = File::open(file_path).expect("Failed to open file hello.txt");

    // let f = match f {
    //     Ok(file) => file,
    //     Err(error) => panic!("Problem opening the file. {:?}", error)
    // };

    // When to use Error Handling
    /*
        - by defult use Result enum and Error Propegation
        - use panic! in non-recoverable errors
        - you can also use panic! in example code like unwrap and expect
        - you can also use unwrap and expect in prototype code and dont want to handle the error and if its moving past the prototype phase you can remove the unwraps and expects and introduce error handling
        - unwrap and expect can also be used in test code
        - you can also use unwrap or expect when you know the code will succeed
    */

    // ex. of succeeding code to call unwrap()
    // this code parses the string and turn it into an IpAddress enum
    let _home: IpAddr = "127.0.0.1".parse().unwrap(); // since this code is hardcoded we know that it will run successfully and its safe to use unwrap()

    // if the _home code is dynamic and the input is not hardcoded then it has the possibility to error and it needs to be handled instead of calling unwrap()

}

// Error Propegation
// calling something that might fail you want to return that error back to the caller who can decide what to do with the error
pub fn _read_username_from_file() -> Result<String, io::Error> {
    let file_path = "./src/errhandling/hello.txt";
    // let mut s = String::new();

    // the question mark(?) indicates that if successful it will return the file and if not it will end early and return the error
    // let mut f = File::open(file_path)?;

    // this code can be simplified using the question mark(?) to the end of the call and its similar to the .unwrap_or() function
    // let mut f  = match f {
    //     Ok(file) => file,
    //     Err(e) => return Err(e),
    // };

    // match f.read_to_string(&mut s)  {
    //     Ok(_) => Ok(s),
    //     Err(e) => return Err(e),
    // }

    // if this fails it will end early and return an error and if succeeds then we know that it read the string in the file
    // f.read_to_string(&mut s)?; // this can be simplified further with chaining method calls

    // chaining method calls
    // File::open(file_path)?.read_to_string(&mut s)?; // this can still be simplier

    fs::read_to_string(file_path) // the fs module has the read_to_string() function that can either return a string containing the contents of the file or return an error
    // this line doesnt need a semi-colon(;) because it is the  code returned

    // Ok(s)
}
