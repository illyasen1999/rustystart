// Mean Median and Mode
// https://doc.rust-lang.org/book/ch08-03-hash-maps.html#summary
// https://www.khanacademy.org/math/statistics-probability/summarizing-quantitative-data/mean-median-basics/a/mean-median-and-mode-review

// Instructions:
/*
    Given a list of integers, use a vector and return the median (when sorted, the value in the middle position) and mode (the value that occurs most often; a hash map will be helpful here) of the list.
*/

use std::collections::HashMap;

// TODO: get the Mean and Mode of both even and odd vectors
// Mode - number the mostly occurs in the vector
pub fn mememo() {
    println!("Mean Median and Mode");

    let mut random_numbers_odd = vec![4, 6, 9, 4, 10, 6, 4, 4, 5];

    println!("Random Numbers Odd: {:?}", random_numbers_odd);

    random_numbers_odd.sort();

    println!("Random Numbers Odd sorted: {:?}", random_numbers_odd);

    // Mean
    let mut sum_all_odd = 0;

    for element in &random_numbers_odd {
        sum_all_odd += element;
    }

    println!("Sum: {}", sum_all_odd);

    let mean_odd = sum_all_odd / 2;

    println!("Odd Mean: {}", mean_odd);

    // Median
    let length_vec_odd = random_numbers_odd.len();
    println!("Length Odd: {}", length_vec_odd);

    let avg_odd = random_numbers_odd.len() / 2;
    println!("Odd Median: {}", random_numbers_odd[avg_odd]);

    // Mode
    let mut odd_map = HashMap::new();

    // let mut counter_map = 0;

    for num in &random_numbers_odd {
        // counter_map += 1;
        // TODO: Mode
        odd_map.insert(String::from("Four"), num);
        odd_map.insert(String::from("Five"), num);
        odd_map.insert(String::from("Six"), num);
        odd_map.insert(String::from("Nine"), num);
        odd_map.insert(String::from("Ten"), num);
        // println!("Map: {:?}", &odd_map);
        odd_map.entry(String::from("Four")).or_insert(&num);
        odd_map.entry(String::from("Five")).or_insert(&num);
        odd_map.entry(String::from("Six")).or_insert(&num);        
        odd_map.entry(String::from("Nine")).or_insert(&num);
        odd_map.entry(String::from("Ten")).or_insert(&num);        
    }

    println!("Map Edited: {:?}", &odd_map);

    println!("\n");

    let mut random_numbers_even = vec![1, 4, 7, 4, 7, 10, 4, 3, 7, 4];

    println!("Random Numbers Even: {:?}", random_numbers_even);

    random_numbers_even.sort();

    println!("Random Numbers Even sorted: {:?}", random_numbers_even);
    
    // Mean
    let mut sum_all_even = 0;

    for element in &random_numbers_even {
        sum_all_even += element;
    }

    println!("Sum: {}", sum_all_even);

    let mean_even = sum_all_even / 2;

    println!("Even Mean: {}", mean_even);

    // Median
    let length_vec_even = random_numbers_even.len();
    println!("Length Even: {}", length_vec_even);

    let mid_1 = length_vec_even / 2;
    let mid_2 = length_vec_even / 2 + 1;
    
    let avg_even = (random_numbers_even[mid_1 - 1] + random_numbers_even[mid_2 - 1]) / 2;

    println!("Even Median: {}", avg_even);

}