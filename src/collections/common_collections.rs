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

    // https://doc.rust-lang.org/book/ch08-01-vectors.html#reading-elements-of-vectors

}