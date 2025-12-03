// Import necessary modules
use std::fs; // File system operations

fn main() {
    // Read the contents of the file "inputs/day01.txt"
    let content = fs::read_to_string("inputs/day01.txt")
        .expect("Failed to read the file");

    // Process each line to extract the action character and the integer value
    // Store the results in a vector of tuples
    let data: Vec<(char, i32)> = content
        .lines() // Iterate over each line of content
        // Map each line to a tuple of (action, value)
        .map(|line| { // For each line:
            let (action, value) = line.split_at(1); // Split at the first character into action and value strings
            let value: i32 = value.parse().expect("Failed to parse number"); // Convert the value to an integer
            (action.chars().next().unwrap(), value) // Create a tuple of (action, value); convert action to char
        })
        .collect(); // Collect all tuples into a vector

    // Initialize password and dial variables
    let mut password = 0;
    let mut dial = 50;
    
    for item in &data {
        if item.0 == 'R' {
            if dial + item.1 > 99{
                password += (dial + item.1) / 100;
            }
            dial = (dial + item.1) % 100;
        } else {
            if dial - item.1 < 1{
                if dial != 0 {
                    password += (dial - item.1) / -100 + 1;
                } else {
                    password += (dial - item.1) / -100;
                }
            }
            dial = (dial-item.1).rem_euclid(100);
        }
    }

    println!("Password: {}", password);
}
