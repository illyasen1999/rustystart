pub fn ownborrow(){
    println!("Topic: Ownership");

    // Sub-topic: The Stack and The Heap
    // Stack - faster, organized, LIFO, data have a fixed size
    // Heap - slower, less organized, needs to search for a spot on the memory to store the data, returns a pointer(address of the data's location)

    /*
     * Ownership Rules:
     * 
     * Each value in Rust has an owner.
     * There can only be one owner at a time.
     * When the owner goes out of scope, the value will be dropped.
     * 
     * */

    // Variable Scope
    let _outer_s = "Letter S";

    {
        // variable "_inner_s" is declared and is valid 
        let _inner_s = "Letter S";
        // do things with "_inner_s"
    } // out of scope, the variable "_inner_s" inside is dropped and is no longer valid

    // Sub-topic: The String Type
    // a great example of data that is stored on the heap
    // String type can be mutated
    let mut _hello = String::from("Hello");
    println!("{}", _hello);
    _hello.push_str(" User");
    println!("String was edited and added another String: {}", _hello);
    _hello = "Hi User".to_string();
    println!("str was converted to String: {}", _hello);

    // Sub-topic: Memory and Allocation
    {
        // inside this scope({}) _s is allocated and is of valid use
        let _s = String::from("Test");
    } // at the end of the scope Rust automatically calls the drop() function and _s is no longer valid

    // Variables and Data with Move
    let _x = 5;
    let _y = _x; // _y copies the data of _x because Integers are of known and fixed size
    println!("_x: {}, _y: {}", _x, _y);

    let _s1 = String::from("S1");
    let _s2 = _s1; // _s2 copies the pointer, length and capacity of _s1 and not the data that is stored on the heap that _s1 has
    // after _s2 is declared _s1 is no longer valid

    // Variables and Data with Clone
    let _ss1 = String::from("SS1");
    let _ss2 = _ss1.clone(); // _ss2 clones all the data of _ss1 and _ss1 is still valid for use
    println!("_ss1: {}, _ss2: {}", _ss1, _ss2);

    // Ownership and Functions
    let _val_string = String::from("string value"); // _val_string gets initialized
    take_ownership(_val_string); // since _val_string is of String type and has  the Drop trait then the function now owns _val_string and after this _val_string is no longer valid
    // println!("{}", _val_string); // this errors because _val_string is dropped

    let _val_int = 32; // _val_in gets initialized
    copies_only(_val_int); // since _val_int is of Integer type and has the Copy trait then the function doesnt need to take ownershio of _val_int and is still valid for usage
    println!("_val_int is still valid: {}", _val_int);

    // Return Values and Scope
    let _val_owner = give_ownership(); // _val_owner now owns the returned value of give_ownership

    let _old_val_owner = String::from("New value");
    let _new_val_owner = take_and_give_ownership(_old_val_owner);



    // Sub-topic: References and Borrowing
    let _new_string = String::from("Hello");
    // calc_length is now "borrowing" the value of _new_string
    let _string_len = calc_length(&_new_string); // the ampersand(&) is used to denote that it will only take a reference of the variable 
    println!("Val: {} | Length: {}", _new_string, _string_len); // _new_string can still be used because the calc_length function only wants the reference and doesnt want to take ownership of _new_string

    // Mutable References
    let mut _s_string = String::from("New");
    let changed_string = change_data(&mut _s_string); // borrowed variable must also be mutable
    println!("{}", &changed_string);

    // you can have many immutable references
    let mut _some_new_string = String::from("Some new string");
    let _ref_a = &_some_new_string;
    let _ref_b = &_some_new_string;

    println!("ref a: {} | ref b: {}", _ref_a, _ref_b);

    // but you can only use one mutable reference at a time
    let mut _another_new_string = String::from("Another new string");
    let _another_ref_a = &mut _another_new_string;
    // let _another_ref_b = &mut _another_new_string;

    // println!("{} {}", _another_ref_a, _another_ref_b);

    let mut _s_new_string = String::from("One reference only");

    // mutable references can only be borrowed one at a time, this prevents "data races"
    let _x_string = &mut _s_new_string;
    // let _y_string = &mut _s_new_string; // this errors because _x_string hast finished using the _s_new_string
    // println!("X: {} | Y: {}", _x_string, _y_string);

    // Dangling References
    // let ref_to_nothing = dangle(); // there is no value to be borrowed

    /*
        Rules of References
        - At any given time, you can have either one mutable reference or any number of immutable references.
        - References must always be valid.
    */

    // Sub-topic: The Slice Type
    // - contiguous sequence of elements in a collection rather than the whole collection. 

    let _my_sentence = String::from("Hello World");
    let _words_res = first_word(&_my_sentence);
    println!("Word sliced: {}", &_words_res);

    let _word_res_2 = second_word(&_my_sentence);
    println!("Word sliced 2: {}", &_word_res_2);

    // string slice
    let _slice_hello = &_my_sentence[0..5];
    let _slice_world = &_my_sentence[6..11];
    println!("{} {}", &_slice_hello, &_slice_world);

    // String Literals as Slices & String Slices as Parameters
    let _test_words = "Rust, Language";
    let _test_res = test_words(&_test_words);
    println!("{}", &_test_res);

    // Other Slices
    // Array slice
    let _array = [1,2,3,4,5];
    println!("Original Array: {:?}", &_array);
    let _array_slice = &_array[2..];
    println!("Array Sliced: {:?}", &_array_slice);

}

// FUNCTION SPACE

fn take_ownership(val: String){
    println!("{}", val);
} // val is dropped since val is of type String

fn copies_only(val: i32){
    println!("Copy: {}", val);
} // val is still valid after being moved to this function because val is of type Integer

fn give_ownership() -> String { // this function returns a value to be owned
    return String::from("Mine");
}

fn take_and_give_ownership(val: String) -> String { // takes ownership of a variable
    return val; // return gives out the value of a variable so it can now be owned by another variable
}

// creating a reference of a value is called "Borrowing"
fn calc_length(val: &String) -> usize { // this function takes in a "reference" of a variable by adding the ampersand(&) before the type and wont take ownership of it and doesnt drop the variable
    let value_len = val.len();
    return value_len;
}

// w/o "&mut" you cant change data from a reference
fn change_data(val: &mut String) -> &String { // takes in a mutable reference for the value to be manipulated
    val.push_str(", World"); // reference data is being manipulated
    return val;
}

// this function errors because there is no value to be borrowed from
// fn dangle() -> &String{ // returns a ref to a String
//     let s = String::from("Dangle");

//     return &s;
// } // "s" goes out of scope, dropped, and its memory goes away leading to the error

fn first_word(s: &String) -> &str {
    let bytes = s.as_bytes();

    for(i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i]; // returns a slice of the word if it detects a space
        }
    }

    return &s[..];
}

fn second_word(s: &String) -> &str {
    let bytes = s.as_bytes();

    for(i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[i..]; // returns a slice of the word if it detects a space
        }
    }

    return &s[..];
}

fn test_words(s: &str) -> &str {
    let bytes = s.as_bytes();

    for(i, &item) in bytes.iter().enumerate() {
        if item == b',' {
            return &s[..i]; // returns a slice of the word if it detects a space
        }
    }

    return &s[..];
}