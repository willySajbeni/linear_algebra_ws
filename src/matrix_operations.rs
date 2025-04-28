// linear_algebra_ws
// Author: Willy Sajbeni
// Website: https://www.sajbeni.com
// GitHub: https://github.com/willySajbeni
// LinkedIn: https://www.linkedin.com/in/willysajbeni/
// Email: willy@sajbeni.com

use crate::input::*;
use std::io;


pub struct Matrix {
    data : Vec<Vec<f64>>,
}

impl Matrix {
    pub fn new(data : Vec<Vec<f64>>) -> Self{
        Matrix { data }
    }

 


    //E na divisão de matrizes: Não é dividir elemento a elemento.
    // É: A / B = A × B⁻¹ Ou seja, multiplicar a matriz A pela inversa da matriz B.
    // Fórmula da inversa de uma matriz 2x2: B^−1 = 1/det(B) * [d,​−b -c, a​] 
    // a = 2, b = 0, c = 1, d = 2
    // Primeiro, o determinante de B: det(B) = (2×2) − (0×1) = 4
    // Então: B^−1 = 1/4 * [2, ​0, -1, 2​]
    // agora Multiplicação de matrizes: Multiplicação linha × coluna:

}