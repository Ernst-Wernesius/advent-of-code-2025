// Import necessary modules
use std::{fs}; // File system operations
mod day02_function;

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
        // loop through the values
        for value in interval.0..=interval.1 {
            // get digits of iterator
            let digits = (value as f64).log10() as u32 + 1;
            // loop through possible dividers
            for divider in (2..=(digits/2)).chain([digits]) {
                // check if divider viable
                if digits > 1
                && digits % divider == 0 
                // recursive function if ID is invalid
                && day02_function::is_invalid(value, digits, digits/divider).0 == true {
                    password += value; // add to password
                    break; // leave divider loop
                }
            }
        }
    }
    // print
    println!("Password: {}", password)
}
