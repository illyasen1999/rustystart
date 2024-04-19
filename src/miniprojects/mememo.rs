// Mean Median and Mode
// https://doc.rust-lang.org/book/ch08-03-hash-maps.html#summary
// https://www.khanacademy.org/math/statistics-probability/summarizing-quantitative-data/mean-median-basics/a/mean-median-and-mode-review

// Instructions:
/*
    Given a list of integers, use a vector and return the median (when sorted, the value in the middle position) and mode (the value that occurs most often; a hash map will be helpful here) of the list.
*/

use std::collections::HashMap;

pub fn mememo() {
    println!("Mean Median and Mode");

    let mut random_numbers_odd = vec![4, 6, 9, 4, 10, 6, 4, 4, 5];

    println!("Random Numbers Odd: {:?}", random_numbers_odd);

    random_numbers_odd.sort();

    println!("Random Numbers Odd sorted: {:?}", random_numbers_odd);

    // Mean
    mean(String::from("Odd"), &random_numbers_odd);

    // Median
    median_odd(&random_numbers_odd);

    // Mode
    mode(String::from("Odd"), &random_numbers_odd);

    println!("\n");

    let mut random_numbers_even = vec![1, 4, 7, 4, 7, 10, 4, 3, 7, 4];

    println!("Random Numbers Even: {:?}", random_numbers_even);

    random_numbers_even.sort();

    println!("Random Numbers Even sorted: {:?}", random_numbers_even);
    
    // Mean
    mean(String::from("Even"), &random_numbers_even);

    // Median
    median_even(&random_numbers_even);

    // Mode
    mode(String::from("Even"), &random_numbers_even);

}

fn mean(op_name: String, given_vec: &Vec<i32>, ) {
    let mut sum_all = 0;

    for element in given_vec {
        sum_all += element;
    }

    println!("Sum: {}", sum_all);

    let mean = sum_all / 2;

    println!("{} Mean: {}", &op_name, mean);
}

fn median_odd(given_vec: &Vec<i32>, ) {
    let length_vec = given_vec.len();
    println!("Length Odd: {}", length_vec);

    let avg = given_vec.len() / 2;
    println!("Odd Median: {}", given_vec[avg]);
}

fn median_even(given_vec: &Vec<i32>) {
    let length_vec_even = given_vec.len();
    println!("Length Even: {}", length_vec_even);

    let mid_1 = length_vec_even / 2;
    let mid_2 = length_vec_even / 2 + 1;
    
    let avg_even = (given_vec[mid_1 - 1] + given_vec[mid_2 - 1]) / 2;

    println!("Even Median: {}", avg_even);
}

fn mode(op_name: String, given_vec: &Vec<i32>) {
    let mut map = HashMap::new();

    for &num in given_vec {
        *map.entry(num).or_insert(0) += 1;
    }

    println!("fn {} Map Edited: {:#?}", op_name, &map);

    // Find the element with the maximum occurrences
    let mut max_count = 0;
    let mut max_element = None;
    for (element, &count) in &map {
        if count > max_count {
            max_count = count;
            max_element = Some(element);
        }
    }
    
    // Output the element with the most occurrences
    if let Some(max_element) = max_element {
        println!("{} Mode: {}", op_name,  max_element);
        println!("{} Mode occurrences: {} times", op_name,  max_count);
    } else {
        println!("No {} elements found", op_name);
    }
}