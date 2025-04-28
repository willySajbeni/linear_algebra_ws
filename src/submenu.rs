// linear_algebra_ws
// Author: Willy Sajbeni
// Website: https://www.sajbeni.com
// GitHub: https://github.com/willySajbeni
// LinkedIn: https://www.linkedin.com/in/willysajbeni/
// Email: willy@sajbeni.com

use std::io; // to read user input
use crate::vector::*; // Make sure the vector module was imported correctly
use crate::matrix_operations::*; // Matrix operations: addition, subtraction, multiplication, inversion, LU, eigenvalues/eigenvectors
use crate::advanced_methods::*;  // Advanced Numerical Methods: iterative, direct, conjugate gradient
use crate::gradient_optimizers::*; // Gradient Optimizers: GD, SGD, Adam, RMSProp
use crate::dimensionality_reduction::*; // Dimensionality Reduction: PCA, SVD, LLE, t-SNE
use crate::input::*; // Helper functions: read_scalar, read_vector, read_matrix, read_function


// Function for the vector operations menu
pub fn vector_menu() {
    loop {
        println!("\nChoose vector operation:");
        println!("1. Sum");
        println!("2. Subtraction");
        println!("3. Multiplication");
        println!("4. Element-Wise Division");
        println!("5. Back");

        let mut choice = String::new();
        io::stdin().read_line(&mut choice).expect("Failed to read input.");
        let choice: u32 = choice.trim().parse().expect("Please enter a valid number.");

        if choice == 5 {
            break;
        }

        // Here we put the operations for vectors
        println!("Enter the first vector (example: 1.0 2.0 3.0):");
        let v1 = read_vector();
        println!("Enter the second vector (example: 1.0 2.0 3.0):");
        let v2 = read_vector();

        let v1 = Vector::new(v1);
        let v2 = Vector::new(v2);

        match choice {
            1 => {
                let result = v1.add(&v2);
                println!("Sum result: {:?}", result);
            },
            2 => {
                let result = v1.subtract(&v2);
                println!("Subtraction result: {:?}", result);
            },
            3 => {
                let result = v1.multiply(&v2);
                println!("Multiplication result: {:?}", result);
            },
            4 => {
                let result = v1.element_wise_division(&v2);
                println!("element-wise Division result: {:?}", result);
            },
            _ => {
                println!("Invalid choice. Please try again.");
            }
        }
    }
}

// Matrix menu function
pub fn matrix_menu() {
    loop {
        println!("\nChoose matrix operation:");
        println!("1. Sum of Matrices");
        println!("2. Subtraction of Matrices");
        println!("3. Multiplication of Matrices");
        println!("4. Matrix Inversion");
        println!("5. Divisão de Matrizes (Multiplicação por Inversa)");
        println!("6. LU decomposition");
        println!("7. Eigenvalues and Eigenvectors");
        println!("8. Back");

        let mut choice = String::new();
        io::stdin().read_line(&mut choice).expect("Failed to read input.");
        let choice: u32 = choice.trim().parse().expect("Please enter a valid number.");

        if choice == 7 {
            break;
        }

        match choice {
            1 => {
                println!("Enter the first matrix:");
                let a = read_matrix();
                println!("Enter the second matrix:");
                let b = read_matrix();
                println!("Sum Matrix not implemented yet!");
            },
            2 => {
                println!("Enter the first matrix:");
                let a = read_matrix();
                println!("Enter the second matrix:");
                let b = read_matrix();
                println!("Subtraction Matrix not implemented yet!");
            },
            3 => {
                println!("Enter the first matrix:");
                let a = read_matrix();
                println!("Enter the second matrix:");
                let b = read_matrix();
                println!("Multiplication of Matrices not implemented yet!");
            },
            4 => {
                println!("Enter the matrix to invert:");
                let a = read_matrix();
                println!("Matrix inversion not implemented yet!");
            },
            5 => {
                println!("Enter the first matrix (M1):");
                let m1 = read_matrix();
                println!("Enter second matrix (M2):");
                let m2 = read_matrix();
                println!("Matrix Division not implemented yet!");
            },
            6 => {
                println!("Enter the matrix for LU decomposition:");
                let a = read_matrix();
                println!("LU decomposition not implemented yet!");
            },
            7 => {
                println!("Enter the matrix for eigenvalues ​​and eigenvectors:");
                let a = read_matrix();
                println!("Calculation of eigenvalues ​​and eigenvectors not implemented yet!");
            },
            _ => {
                println!("Invalid choice. Please try again.");
            }
        }
    }
}


// Advanced Numerical Methods menu function
pub fn advanced_methods_menu() {
    loop {
        println!("\nChoose Advanced Numerical Methods operation:");
        println!("1. Iterative Solution of Linear Systems (Jacobi, Gauss-Seidel)");
        println!("2. Direct Methods (Gaussian Elimination, LU Factorization)");
        println!("3. Conjugate Gradient Method (sparse systems)");
        println!("4. Back");

        let mut choice = String::new();
        io::stdin().read_line(&mut choice).expect("Failed to read input.");
        let choice : u32 = choice.trim().parse().expect("Please enter a valid number.");
    

        if choice == 4 {
            break;
        }

        match choice {
            1 => {
                println!("Enter the matrix A:");
                let a = read_matrix();
                println!("Enter vector b:");
                let b = read_vector();
                println!("Iterative method not implemented yet!");
            },
            2 => {
                println!("Enter matrix A:");
                let a = read_matrix();
                println!("Enter vector b:");
                let b = read_vector();
                println!("Direct Methods not implemented yet!");
            },
            3 => {
                println!("enter matrix A:");
                let a = read_matrix();
                println!("Enter vector b:");
                let b = read_vector();
                println!("Conjugate Gradient not implemented yet!");
            },
            _ => {
                println!("Invalid choice. Please try again.");
            }
        }
    }
}

// Gradient Optimizers menu function
pub fn gradient_optimizers_menu() {
    loop {
        println!("\nChoose Gradient Optimizers operation:");
        println!("1. Simple Gradient Descent");
        println!("2. Stochastic Gradient (SGD)");
        println!("3. Adam Optimizer");
        println!("4. RMSProp");
        println!("5. Back");

        let mut choice = String::new();
        io::stdin().read_line(&mut choice).expect("Failed to read input.");
        let choice : u32 = choice.trim().parse().expect("Please enter a valid number.");

        if choice == 5 {
            break;
        }

        match choice {
            1 => {
                println!("Enter cost function:");
                let rf = read_function();
                println!("Enter starting point:");
                let x0 = read_scalar();
                println!("Gradient Descent not implemented yet!");
            },
            2 => {
                println!("Enter cost function:");
                let rf = read_function();
                println!("Enter starting point:");
                let x0 = read_scalar();
                println!("Stochastic Gradient not implemented yet!");
            },
            3 => {
                println!("Enter cost function:");
                let rf = read_function();
                println!("Enter starting point:");
                let x0 = read_scalar();
                println!("Adam Optimizer not implemented yet!");
            },
            4 => {
                println!("Enter cost function:");
                let rf = read_function();
                println!("Enter starting point:");
                let x0 = read_scalar();
                println!("RMSProp not implemented yet!");
            },
            _ => {
                println!("Invalid choice. Please try again.");
            }
        }
    }
}


// Dimensionality Reduction menu function
pub fn dimensionality_reduction_menu() {
    loop{
        println!("\nChoose Dimensionality Reduction operation:");
        println!("1. PCA (Principal Component Analysis)");
        println!("2. SVD (Singular Value Decomposition)");
        println!("3. LLE (Locally Linear Embedding)");
        println!("4. t-SNE (t-Distributed Stochastic Neighbor Embedding)");
        println!("5. Back");

        let mut choice = String::new();
        io::stdin().read_line(&mut choice).expect("Failed to read input.");
        let choice : u32 = choice.trim().parse().expect("Please enter a valid number.");

        if choice == 5 {
            break;
        }

        match choice {
            1 => {
                println!("Enter the data array:");
                let data = read_matrix();
                println!("PCA not implemented yet!");
            },
            2 => {
                println!("Enter the data array:");
                let data = read_matrix();
                println!("SVD not implemented yet!");
            },
            3 => {
                println!("Enter the data array:");
                let data = read_matrix();
                println!("LLE not implemented yet!");
            },
            4 => {
                println!("Enter the data array:");
                let data = read_matrix();
                println!("t-SNE not implemented yet!");
            },
            _ => {
                println!("Invalid choice. Please try again.");
            }
        }
    }
}