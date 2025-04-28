// linear_algebra_ws
// Author: Willy Sajbeni
// Website: https://www.sajbeni.com
// GitHub: https://github.com/willySajbeni
// LinkedIn: https://www.linkedin.com/in/willysajbeni/
// Email: willy@sajbeni.com

use std::io;

// Function to read a vector
pub fn read_vector() -> Vec<f64>{
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read vector.");
    input
        .trim()
        .replace(",", " ")
        .split_whitespace()
        .map(|x| x.parse::<f64>().expect("Failed to read vector."))
        .collect()
}

// Function to read a scalar
pub fn read_scalar() -> f64 {
    println!("Enter a scalar number:");
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read scalar.");
    input.trim().parse().expect("Please enter a valid number.")
}

// Function to read an matrix
pub fn read_matrix() -> Vec<Vec<f64>> {
    println!("Enter the lines of the matrix, one at a time. Press ENTER after each line. Blank line to end:");
    let mut matrix = Vec::new();
    loop{
        let mut line = String::new();
        io::stdin().read_line(&mut line).expect("Failed to read array row.");
        let trimmed = line.trim();
        if trimmed.is_empty() {
            break;
        }
        let how: Vec<f64> = trimmed.replace(",", " ").split_whitespace()
            .map(|x| x.parse::<f64>().expect("Invalid value in array"))
            .collect();
        matrix.push(how);
    }
    matrix
}

// Function to read a cost function (just as a string for now)
pub fn read_function() -> String {
    println!("Enter the cost function (e.g. x.powi(2) + 3.0 * x + 2.0):");
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read function.");
    input.trim().to_string()
}