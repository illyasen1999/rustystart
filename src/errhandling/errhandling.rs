use std::fs::File;
use std::io::ErrorKind;

pub fn errhandling() {
    println!("Topic: Error Handling");
    // 2 major error categories: recoverable and unrecoverable
    // Rust has the Result<T,E> in exchange for exceptions and the panic! macro for unrecoverable errors

    // https://doc.rust-lang.org/book/ch09-01-unrecoverable-errors-with-panic.html#unwinding-the-stack-or-aborting-in-response-to-a-panic

    // Sub-topic: Unrecoverable Errors with panic!

    // calling panic! and checking the error messages
    // panic!("Crash and burn 😈🔥");

    let _v = vec![1, 2, 3];
    // Backtrace - is a list of all the functions that have been called to get to this point
    // _v[99]; // checking the panic! Backtrace

    // Recoverable Errors with Result
    // structure of the Result enum
    // enum Result<T, E> {
    //     Ok(T),
    //     Err(E)
    // }

    // ex. using Result
    // this returns the Result enum and can run even though the file may or may not exist
    // if the file exists it will have the instance Ok() which in this case opens the file
    // if the file does not exists it will have the instance of Err() that contains more information about the kind of error that happened
    let _greeting_file_res = File::open("hello.txt");
    // println!("{:#?}", _greeting_file_res); // for logging purposes

    // take on different actions depending on the value of File::open returns
    // since "hello.txt" does not exist it will run the Err() instance
    // let _greeting_file = match _greeting_file_res {
    //     Ok(file) => file, // if exists return the file
    //     Err(error) => panic!("Problem opening the file {:?}", error),
    // }; // after this we can use the file handle for reading or writing
    
    // Matching on Different Errors
    // handle different actions based on different failure reasons
    let _greeting_file = match _greeting_file_res {
        Ok(file) => file,
        // here we want to execute different kinds of codes based on different kinds of errors by adding inner match cases
        Err(error) => match error.kind() {
            // here if the file does not exist we create that file from File::create which might also fail
            // and if it fails we still want to process a panic! for further info of the error
            ErrorKind::NotFound => match File::create("./src/errhandling/hello.txt") {
                Ok(fc) => fc,
                Err(e) => panic!("Problem creating the file {:?}", e),
            },
            other_error => {
                panic!("Problem opening the file {:?}", other_error);
            }
        }
    };

    // https://doc.rust-lang.org/book/ch09-02-recoverable-errors-with-result.html#alternatives-to-using-match-with-resultt-e

}