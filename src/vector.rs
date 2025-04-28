// linear_algebra_ws
// Author: Willy Sajbeni
// Website: https://www.sajbeni.com
// GitHub: https://github.com/willySajbeni
// LinkedIn: https://www.linkedin.com/in/willysajbeni/
// Email: willy@sajbeni.com


//Structure definition:
//A Vector will have a field called data, and this field is a Vec<f64>.
// when I use to insert the vector it will be the v1 that really exists, and v1.data has a Vec<f64> inside: [1.0, 2.0, 3.0] and there the let is created
#[derive(Debug)]
pub struct Vector {
    data: Vec<f64>,
}

//We are implementing methods for the Vector type.
impl Vector {
    pub fn new(data : Vec<f64>) -> Self {
        Vector { data }
    }
    //Vector addition : v1 + v2 = [a1​+b1 ​,a2​+b2 ​,a3​+b3​]
    //Creating a function called add. It receives:
    pub fn add(&self, other: &Self) -> Self { //- &self ➔ the vector that calls the method - &other ➔ another vector to add - Returns ➔ a new Vector
        assert_eq!(self.data.len(), other.data.len(), "Vectors must be the same size for addition."); //assert_eq! Check: vectors of the same size. If not, the program crashes and displays the message.

        let data = self.data.iter() //We get each element from self.data using an iterator (iter).
            .zip(&other.data) //We pack (zip) each element of the self with that of the other (pair by pair).
            .map(|(x,y)| x + y) // applies the operation to each pair.
            .collect();// Merge the result into a new vector (Vec<f64>).

        Vector::new(data) //Creates and returns a new Vector with the resulting data.
    }

    //Vector subtraction : v1 − v2 = [a1​−b1 ​,a2​−b2 ​,a3​−b3​]
    // Creating a function called subtract. It receives:
    pub fn subtract(&self, other: &Self) -> Self { //- &self ➔ the vector that calls the method - &other ➔ another vector to subtract - Returns ➔ a new Vector
        assert_eq!(self.data.len(), other.data.len(), "Vectors must be the same size for subtraction.");//assert_eq! Check: vectors of the same size. If not, the program crashes and displays the message.
        
        let data = self.data.iter() //We get each element from self.data using an iterator (iter).
            .zip(&other.data) //We pack (zip) each element of the self with that of the other (pair by pair).
            .map(|(x, y)| x - y) // applies the operation to each pair.
            .collect(); // Merge the result into a new vector (Vec<f64>).
        
        Vector::new(data) //Creates and returns a new Vector with the resulting data.
    }

    // multiplication of vectors element by element : v1 × v2 = [a1​×b1​, a2​×b2​, a3​×b3​]
    // Creating a function called multiply. It receives:
    pub fn multiply(&self, other: &Self) -> Self { //- &self ➔ the vector that calls the method - &other ➔ another vector to multiply - Returns ➔ a new Vector
        assert_eq!(self.data.len(), other.data.len(), "Vectors must be the same size for multiplication."); //assert_eq! Check: vectors of the same size. If not, the program crashes and displays the message.

        let data = self.data.iter() //We get each element from self.data using an iterator (iter).
            .zip(&other.data) //We pack (zip) each element of the self with that of the other (pair by pair).
            .map(|(x, y)| x * y) //applies the operation to each pair.
            .collect(); // Merge the result into a new vector (Vec<f64>).

        Vector::new(data) //Creates and returns a new Vector with the resulting data.
    }

    //divide to Vector (Multiplication by Inverse)
    //Vector division is not done as a traditional element-by-element division operation.
    //Instead, we divide vectors by multiplying one vector by the inverse of another vector, element by element.
    //So dividing v1 by v2 will be: v1/v2 = [a1​×(1/b1​​),a2​×(1/b2)​​,a3​×(1/b3)​​]
    //Check if no value of v2 is zero (since we cannot divide by zero!).
    //Then, we do the multiplication element by element.
    pub fn element_wise_division(&self, other: &Self) -> Self { //We created the divide method, which receives another vector other and returns a new Vector.
        assert_eq!(self.data.len(), other.data.len(), "Vectors must be the same size for division."); //We check if the vectors have the same size.
        assert!(!other.data.iter().any(|&x| x == 0.0), "Cannot divide by zero."); //Using assert!: If we encounter a zero, the code will crash and show the error message.

        let data = self.data.iter() //We get each element from self.data using an iterator (iter).
            .zip(&other.data) //We pack (zip) each element of the self with that of the other (pair by pair).
            .map(|(x, y)| x * (1.0 / y)) // Multiplying by 1/y, which is the inverse of y
            .collect();

        Vector::new(data)
    }
}