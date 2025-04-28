# linear_algebra_ws

**Author:** [Willy Sajbeni](https://www.sajbeni.com)  
**Email:** [willy@sajbeni.com](mailto:willy@sajbeni.com)  
**GitHub:** [https://github.com/willySajbeni](https://github.com/willySajbeni)  
**LinkedIn:** [https://www.linkedin.com/in/willysajbeni/](https://www.linkedin.com/in/willysajbeni/)


A minimal, transparent, and Open Source linear algebra library written in Rust.

## Why?

Most machine learning libraries today (TensorFlow, PyTorch, etc.) involve heavy tracking, telemetry, or hidden behaviors.  
**LinearAlgebra-RS** is designed to be completely open, simple, educational, and 100% under your control — **no tracking, no spyware**.

This project starts from pure **linear algebra operations** and will evolve into a full **numerical computation engine**.

---

## Current Features

✅ **Vector operations**
- Sum
- Subtraction
- Multiplication (element-wise)
- Division (element-wise)

✅ **User input for vectors and matrices**

---

## Upcoming Features (Roadmap)

- **Matrix operations**
  - Sum of Matrices
  - Subtraction of Matrices
  - Multiplication of Matrices
  - Matrix Inversion
  - Matrix Division (via Multiplication by Inverse)
  - LU Decomposition
  - Eigenvalues and Eigenvectors

- **Advanced Numerical Methods**
  - Iterative Methods for Linear Systems (Jacobi, Gauss-Seidel)
  - Direct Methods (Gaussian Elimination, LU Factorization)
  - Conjugate Gradient Method (for sparse systems)

- **Gradient Optimizers**
  - Simple Gradient Descent
  - Stochastic Gradient Descent (SGD)
  - Adam Optimizer
  - RMSProp

- **Dimensionality Reduction**
  - PCA (Principal Component Analysis)
  - SVD (Singular Value Decomposition)
  - LLE (Locally Linear Embedding)
  - t-SNE (t-Distributed Stochastic Neighbor Embedding)

---

## How to Run

Make sure you have **Rust** installed.

Clone this repository:

```bash
git clone https://github.com/willySajbeni/linear_algebra_ws.git
cd linear_algebra_ws
cargo run
