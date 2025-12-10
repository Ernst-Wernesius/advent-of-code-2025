// Import necessary modules
use std::fs; // File system operations

fn main() {
    // Read the contents of the file "inputs/day01.txt"
    let content = fs::read_to_string("inputs/day03.txt")
        .expect("Failed to read the file");

    // Process each line
    // Store the results in a vector of vectors
    let data : Vec<Vec<u8>> = content
        .lines()
        .map(|line|{ // For each line:
            // Initializing new Vector (list of the digits)
            let mut digits: Vec<u8> = Vec::new();
            for char in line.chars() { // For every character in string
                // Using Rust char to digit function to get every digit as integer
                digits.push(char.to_digit(10).expect("That's no Digit") as u8);
            }
            digits // Return digits vector for every line
        })
        .collect(); // Collect the digit vectors in one vector

    // Initalizing variable for the result
    let mut maximum_joltage = 0;

    // Loop through every line
    for line in &data {
        // Get highest digit of vector except last element
        let highest = getting_highest_digit(&line[..line.len()-1]);
        // Get highest in list *after* the highest one
        let second_highest= getting_highest_digit(&line[(highest.0+1)..]);
        // Fuse the digits together to 2 digit number and add to max joltage
        maximum_joltage += highest.1 as u32 * 10 + second_highest.1 as u32;
    }

    // Print result
    println!("Maximum joltage is: {}", maximum_joltage)

}

fn getting_highest_digit(digits: &[u8]) -> (usize, u8) {
    // Initialize variables to first element of list
    let mut highest_digit = digits[0];
    let mut highest_index= 0;
    // Loop through the rest and check for higher digits
    for (index, digit) in digits.iter().enumerate().skip(1) {
        // Save higher digit
        if *digit > highest_digit {
            highest_digit = *digit;
            highest_index = index;
        }
    };
    // return index (needed for 2nd highest digit) and digit
    return(highest_index, highest_digit)
}