/*
    Nth Fibonacci Number
    Implement a function to calculate the `n`th Fibonacci number. 
    The Fibonacci sequence is defined as follows:
    F(0) = 0, F(1) = 1, F(n) = F(n-1) + F(n-2) for n > 1.

    You need to implement the function `fib(n: i32) -> i32` to return the `n`th Fibonacci number.
    
    Hint: Consider using matrix exponentiation to solve the problem in O(log n) time complexity.
*/

use std::fmt::{self, Display, Formatter};

pub fn fib(n: i32) -> i32 {
    if n == 0 {
        return 0; // Base case: F(0) = 0
    }
    if n == 1 {
        return 1; // Base case: F(1) = 1
    }

    // Define the transformation matrix for Fibonacci sequence
    let mut matrix = vec![vec![1, 1], vec![1, 0]];

    // Raise the matrix to the (n-1)th power
    matrix_power(&mut matrix, n - 1);

    // The top-left element of the matrix is F(n)
    matrix[0][0]
}

// Function to multiply two 2x2 matrices
fn multiply(a: &Vec<Vec<i32>>, b: &Vec<Vec<i32>>) -> Vec<Vec<i32>> {
    let mut result = vec![vec![0, 0], vec![0, 0]];
    for i in 0..2 {
        for j in 0..2 {
            for k in 0..2 {
                result[i][j] += a[i][k] * b[k][j];
            }
        }
    }
    result
}

// Function to raise a 2x2 matrix to a given power
fn matrix_power(matrix: &mut Vec<Vec<i32>>, power: i32) {
    let mut result = vec![vec![1, 0], vec![0, 1]]; // Identity matrix
    let mut base = matrix.clone();

    let mut p = power;
    while p > 0 {
        if p % 2 == 1 {
            result = multiply(&result, &base);
        }
        base = multiply(&base, &base);
        p /= 2;
    }

    *matrix = result;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_fib_1() {
        let result = fib(0);
        println!("Fibonacci of 0: {}", result);
        assert_eq!(result, 0);
    }

    #[test]
    fn test_fib_2() {
        let result = fib(1);
        println!("Fibonacci of 1: {}", result);
        assert_eq!(result, 1);
    }

    #[test]
    fn test_fib_3() {
        let result = fib(2);
        println!("Fibonacci of 2: {}", result);
        assert_eq!(result, 1);
    }

    #[test]
    fn test_fib_4() {
        let result = fib(3);
        println!("Fibonacci of 3: {}", result);
        assert_eq!(result, 2);
    }

    #[test]
    fn test_fib_5() {
        let result = fib(10);
        println!("Fibonacci of 10: {}", result);
        assert_eq!(result, 55);
    }

    #[test]
    fn test_fib_6() {
        let result = fib(20);
        println!("Fibonacci of 20: {}", result);
        assert_eq!(result, 6765);
    }
}