// Import necessary modules
use std::{fs}; // File system operations

fn main() {
    // Read the contents of the file "inputs/day01.txt"
    let content = fs::read_to_string("inputs/day02.txt")
        .expect("Failed to read the file");

    // Process each interval
    // Store the results in a vector of tuples
    let data: Vec<(u64, u64)> = content
        .split(',')// Split the content by commas
        .map(|interval| { // For each interval:
            let (start, end) = interval.split_once('-').unwrap(); // get start and end value of interval
            let start: u64 = start.parse().expect("Failed to parse number"); // Convert the value to an big enough integer
            let end: u64 = end.parse().expect("Failed to parse number"); // Convert the value to an big enough integer
            (start, end) // Create a tuple of (start, end)
        })
        .collect(); // Collect all tuples into a vector

    // set dynamic password variable
    let mut password: u64 = 0;
    
    // loop through the intervals
    for interval in &data {
        for i in interval.0..=interval.1 {
            // get digits of iterator
            let digits = (i as f64).log10() as u32 + 1;
            // check if number can be split in half
            if digits.is_multiple_of(2)
            // check if first half of number is the same as second
            // Logic: xy -> x,y -> x check if xy-y same as xy-x
                && (i / 10_u64.pow(digits/2) * 10_u64.pow(digits/2)) == (i - i / 10_u64.pow(digits/2)){
                    // add to password
                    password += i;
                }
        }
    }
    // print
    println!("Password: {}", password)
}
