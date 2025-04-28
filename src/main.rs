// linear_algebra_ws
// Author: Willy Sajbeni
// Website: https://www.sajbeni.com
// GitHub: https://github.com/willySajbeni
// LinkedIn: https://www.linkedin.com/in/willysajbeni/
// Email: willy@sajbeni.com

mod submenu;
mod vector;
mod matrix_operations;
mod advanced_methods;
mod gradient_optimizers;
mod dimensionality_reduction;
mod input;

use crate::vector::*;
use crate::matrix_operations::*;
use crate::advanced_methods::*;
use crate::gradient_optimizers::*;
use crate::dimensionality_reduction::*;
use crate::input::*;
use crate::submenu::*;
use std::io;


fn main() {
    loop {
        println!("\n===== MAIN MENU =====");
        println!("1. Vector Operations");
        println!("2. Matrix Operations");
        println!("3. Advanced Numerical Methods");
        println!("4. Gradient Optimizers");
        println!("5. Dimensionality Reduction");
        println!("6. Exit");

        let mut choice = String::new();
        io::stdin().read_line(&mut choice).expect("Failed to read input.");
        let choice: u32 = match choice.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Please enter a valid number.");
                continue;
            }
        };

        match choice {
            1 => vector_menu(),
            2 => matrix_menu(),
            3 => advanced_methods_menu(),
            4 => gradient_optimizers_menu(),
            5 => dimensionality_reduction_menu(),
            6 => {
                println!("Leaving... See you soon!");
                break;
            },
            _ => {
                println!("Invalid choice. Please try again.");
            }
        }
    }
}
