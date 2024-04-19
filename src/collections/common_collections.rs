use unicode_segmentation::UnicodeSegmentation;
use std::{collections::HashMap, hash::Hash};

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

    // Sub-topic: Storing UTF-8 Encoded Text with Strings
    // Strings - is a growable, mutable, owned, UTF-8 encoded string type

    // Creating a new String
    let mut _new_string = String::new(); // a new instance of a String

    // w/ initial data
    let initial_data = "Initial Data"; // this is a string slice or &str
    let _s = initial_data.to_string(); // converts the string slice data to a String type
    let _s = "Initial Data".to_string(); // this is the same as the one at the top as the string slice have methods 

    // creating a string from a string literal
    let _new_s = String::from("Initial Data"); // this is also the same with the .to_string() function 

    // Strings are UTF-8 encoded ex.
    let _kr_lang = String::from("안녕하세요");
    let _ukranian_lang = String::from("Здравствуйте");
    let _jp_lang = String::from("こんにちは");

    // Sub-topic: Updating a String

    // Append to a string using push_str and push
    // grow a string by using .push_str that appends a string slice
    let mut _pushed_string = String::from("Hello ");
    _pushed_string.push_str("World"); // output: Hello World

    let mut _push_string = String::from("foo");
    let _push_string_2 = "bar";
    _push_string.push_str(_push_string_2); // does not take ownership of _push_string_2, output: foobar
    
    // push() takes a single char and adds it to the String
    let mut _push_s = String::from("LO");
    _push_s.push('L'); // output: LOL

    // using plus(+) operator
    let _s1 = String::from("New");
    let _s2 = String::from("World");
    let _s3 = _s1 + &_s2; // since s3 owns s1, s1 cannot be used after this line
    
    // using the format! macro
    let _new_s1 = String::from("One");
    let _new_s2  = String::from("Piece");
    // format! macro doesnt take ownership so _new_s1 and _new_s2 is still valid
    let _new_s3 = format!("{} {}", _new_s1, _new_s2); // output: One Piece

    // indexing into a string
    let _hello_kr = String::from("안녕하세요");
    // let c: char = _hello[0]; // using a number doesnt work on a String as String has 3 ways a word is represented
    // Bytes - unit data of a character
    // accessing the bytes of a string using the .bytes() method
    for byte in _hello_kr.bytes() {
        println!("{}", byte);
    }
    
    // Scalar Values - are characters or part of the characters
    // accessing the chars of a string using the .chars() method
    for ch in _hello_kr.chars() {
        println!("{}", ch);
    }  
    
    // Grapheme Clusters - we consider as characters
    // Rust doesnt have a default function for Grapheme Clusters for that we need an external crate using the unicode-segmentation crate and use the .graphemes() method
    for g in _hello_kr.graphemes(true) {
        println!("{}", g);
    }

    // Sub-topic: Hashmaps
    // - are a collection of key-value pairs
    // Hashmaps are a part of the standard library and has to be imported from the collections library, Hasmaps store their data in the heap like Vectors, and all keys must have the same data type

    // creating a new Hashmap
    let mut scores = HashMap::new();

    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);

    // accessing a value in a hashmap using the .get() method and providing it a key, .get() returns an Option enum
    // .copied() method handles the "None" in the Option enum
    // .unwrap_or() returns the value of the hashmap and if there is no set value it will use the default value that you provided
    let _blue_team = scores.get("Blue").copied().unwrap_or(0);

    // iterating in a hashmap
    for (k,v) in &scores {
        println!("Team: {}, Score: {}", k, v);
    }

    // Hashmaps and Ownership
    let _num: i32 = 200; // primitive types have the Copy trait are copied into the hashmap
    let _my_string = String::from("String"); // types like String are owned so the values will be moved and the hashmap will be the owner of those values

    let field_name = String::from("Fav Color");
    let field_value = String::from("Blue");

    let mut owned_map = HashMap::new();
    owned_map.insert(field_name, field_value); // map now owns the field name and value so after this point they are not valid to use

    // println!("{} {}", field_name, field_value); // cannot run this

    let letter_a = 'A';
    let letter_in_bytes = letter_a as i32;
    
    let mut copied_map = HashMap::new();
    copied_map.insert(letter_a, letter_in_bytes);

    println!("{}: {}", letter_a, letter_in_bytes); // this is valid because primitive types are copied in the hashmap

    // Sub-topic: Updating a Hashmap
    // Overwriting a value

    let mut new_scores = HashMap::new();

    new_scores.insert(String::from("Blue"), 10);
    new_scores.insert(String::from("Blue"), 50); // this overwrites the previous value of "Blue"

    // Adding K,V if K isnt present
    new_scores.entry(String::from("Blue")).or_insert(20); // "Blue" is already in the Hashmap and has already have a Value so its value wont change into 20
    new_scores.entry(String::from("Yellow")).or_insert(30); // .entry() function adds a Key to the hashmap since Yellow is not currently in the hashmap and adds a Value with the .or_insert() function if it doesnt have a Value already, .or_insert() returns a mutable reference to the value of the entry key if that key exists

    println!("Teams and Scores: {:?}", new_scores);

    // Updating a value based on the old value
    let text = "Hello world New world";

    let mut text_map = HashMap::new();

    for word in text.split_whitespace() {
        // grabs individual words as a Key and counting how many times it is mentioned in the sentence as a Value
        let count = text_map.entry(word).or_insert(0);
        *count += 1; // dereferencing the count because  .or_insert() function returns a mutable reference
    }

    println!("{:?}", text_map);

    // Hashing Functions for Security of software
    // https://doc.rust-lang.org/book/ch08-03-hash-maps.html#hashing-functions
}