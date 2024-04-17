use std::vec;

pub fn common_collections() {
    println!("Topic: Common Collections");

    // Collections - can contain multiple values, and are stored on the heap
    // - vector, string, hash map
    /*
        - A vector allows you to store a variable number of values next to each other.
        
        - A string is a collection of characters. We’ve mentioned the String type previously, but in this chapter we’ll talk about it in depth.
        
        - A hash map allows you to associate a value with a particular key. It’s a particular implementation of the more general data structure called a map.
    */

    // Sub-topic: Storing lists of values with Vectors
    // creating a vector, an empty vector needs type annotations to be created
    let _some_vec: Vec<i32> = Vec::new();

    // Rust has a vec![] macro that creates a new vector that holds the values you give it.
    let _value_vec = vec![1, 2, 3];

    // Updating a vector
    // initializng a vector and giving it values
    // by making the vector mutable we can add values in the vector
    // vectors are index base so the first element is starting at 0
    let mut _new_vec = Vec::new();

    // pushing new values in a vectord
    _new_vec.push(5);
    _new_vec.push(6);
    _new_vec.push(7);
    _new_vec.push(8);

    // outputting values of a vector
    println!("Vector: {:#?}", &_new_vec);

    for i in &_new_vec {
        println!("Vec Item: {:?}", &i);
    }

    // Reading Elements of Vectors
    // "indexing" - one of the ways of referencing an element in a vector, this works best if you want the program to panic if you are accessing an element past the length of the vector
    let _another_vec = vec![1, 2, 3, 4, 5];
    let third_element: &i32 = &_another_vec[2]; // getting the ref
    println!("Third element is {}", third_element);

     // ".get()" -the other way of referencing a value in a vector, this works best if you want your code to have the logit to handle having either "Some(&element)" or "None"
    let third_element: Option<&i32> = _another_vec.get(2); // the .get() returns an Option enum
    match third_element {
        // running some code if there is a third element
        Some(third_element) => println!("Third element is {:?}", third_element),
        // handling the situation where the element that you are lookign for is out-of-bounds of the vector
        None => println!("There is no third element"),
    }

    // trying to access elements outside of the range of the vector / referencing a none existent element
    let _some_vector = vec![1, 2, 3, 4, 5];

    // let _does_not_exist = &_some_vec[100]; // this panics and gives an error message
    let _does_not_exist = &_some_vec.get(100); // while this does nothing and not panic since you are not defining the "None" logic

    let mut _other_vec = vec![1, 2, 3, 4, 5];
    let first = &_other_vec[0];
    // _other_vec.push(6); // the "first" variable still holds the referenced "_other_vec" vector and still hasnt used the vecor therefore it cannot push a value inside the vector
    println!("First: {}", &first);

    // iterating over values in a vector
    // using a for loop
    let an_immutable_vec = vec![20, 50, 33, 116];
    for item in &an_immutable_vec {
        println!("Some New Vec Item: {}", item);
    }

    // iterating over a mutable vector and making some changes to all elements

    let mut _a_mutable_vec = vec![88, 300, 17, 2413];
    for element in &mut _a_mutable_vec {
        *element *= 2; // to change values in a referenced vector you need to dereference it buy using the star(*) symbol to get its values
        println!("Element x 2 = {}", element);

        // _a_mutable_vec.push(2); // you cannot add or remove an item in an iterating vector as the iterator is still in use of the vector
    }
    _a_mutable_vec.push(2); // only after you can add or remove items in the vector

    // Using Enums to store Muliple Types
    // a normal vector can only store 1 type
    // enums allow vectors to hold data of multiple types
    #[derive(Debug)]
    enum SpreadsheetCell {
        Int(i32),
        Float(f32),
        Text(String),
    }

    let _row = vec![
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Text(String::from("Blue")),
        SpreadsheetCell::Float(3.14),
    ];

    println!("Row: {:?}", _row);

    // Dropping a vector drops its elements
    {
        let _v = vec![1, 2, 3, 4, 5];
        // "_v" is valid to use
    } // "_v" goes out-of-scope and now cannot be used outside of this scope
}