// Mean Median and Mode
// https://doc.rust-lang.org/book/ch08-03-hash-maps.html#summary
// https://www.khanacademy.org/math/statistics-probability/summarizing-quantitative-data/mean-median-basics/a/mean-median-and-mode-review

// Instructions:
/*
    Given a list of integers, use a vector and return the median (when sorted, the value in the middle position) and mode (the value that occurs most often; a hash map will be helpful here) of the list.
*/

// TODO: get the Mean and Mode of both even and odd vectors
// Mean - sum of all items / number of items
// Mode - number the mostly occurs in the vector
pub fn mememo() {
    println!("Mean Median and Mode");

    let mut random_numbers_odd = vec![4, 6, 9, 4, 10, 6, 4, 4, 5];

    println!("Random Numbers Odd: {:?}", random_numbers_odd);

    // Median
    random_numbers_odd.sort();

    println!("Random Numbers Odd sorted: {:?}", random_numbers_odd);

    let mean = random_numbers_odd.iter().min();
    match mean {
        Some(min) => println!("Mean {:?}", min),
        None => println!("No Mean"),
    }

    let length_vec_odd = random_numbers_odd.len();
    println!("Length Odd: {}", length_vec_odd);

    let avg_odd = random_numbers_odd.len() / 2;
    println!("avg_odd Median: {}", random_numbers_odd[avg_odd]);

    println!("\n");

    let mut random_numbers_even = vec![1, 4, 7, 4, 7, 10, 4, 3, 7, 4];

    println!("Random Numbers Even: {:?}", random_numbers_even);

    // Median
    random_numbers_even.sort();

    println!("Random Numbers Even sorted: {:?}", random_numbers_even);
    
    let length_vec_even = random_numbers_even.len();
    println!("Length Even: {}", length_vec_even);

    let mid_1 = length_vec_even / 2;
    let mid_2 = length_vec_even / 2 + 1;
    
    let avg_even = (random_numbers_even[mid_1 - 1] + random_numbers_even[mid_2 - 1]) / 2;

    println!("avg_even Median: {}", avg_even);

}