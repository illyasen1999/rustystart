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

    // https://doc.rust-lang.org/book/ch08-01-vectors.html#updating-a-vector
}