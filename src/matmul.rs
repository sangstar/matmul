use std::fmt;
use std::fmt::Formatter;
use std::ops::Mul;


// TODO: Check if docstrings are not idiomatic 
#[derive(Debug)]
pub struct Matrix {
    pub data: Vec<Vec<i32>>,
}

impl Matrix {
    pub fn new(data: Vec<Vec<i32>>) -> Matrix {
        if !has_uniform_row_length(&data) {
            panic!("Vectors must have same length");
        }
        let matrix = Matrix { data };
        matrix
    }

    fn get_column_vector(&self, col_index: i32) -> Vec<i32> {
        let mut col: Vec<i32> = vec![];
        for i in 0..self.data.len() {
            col.push(self.data[i][col_index as usize]);
        }
        col
    }
}

// Define binary `==` operator for Matrix
impl PartialEq for Matrix {
    fn eq(&self, other: &Self) -> bool {
        self.data == other.data
    }
}

// Implement `fmt::Display` to define effect of `println` for Matrix
impl fmt::Display for Matrix {
    fn fmt(&self, _f: &mut Formatter) -> fmt::Result {
        println!("\n");
        for row in &self.data {
            for elem in row {
                println!("{}", elem);
            }
        }
        Ok (())
    }
}

impl Mul for Matrix {
    type Output = Self;
    fn mul(self, rhs: Self) -> Self::Output {
        let mut c_data: Vec<Vec<i32>> = vec![];
        if !is_matmul_compatible(&self, &rhs) {
            panic!("Matrices are not compatible for multiplication")
        }
        for i in 0..self.data.len() {
            let mut c_row: Vec<i32> = vec![];
            for j in 0..rhs.data[i].len() {
                let b_col: Vec<i32> = rhs.get_column_vector(j as i32);
                c_row.push(inner_product(&self.data[i], &b_col));
            }
            c_data.push(c_row);
        }
        let matrix = Matrix { data: c_data };
        matrix
    }
}


// TODO: Optimize later
fn has_uniform_row_length(data: &Vec<Vec<i32>>) -> bool {
    let first_len: usize = data[0].len();
    for vec in data {
        if vec.len() != first_len {
            return false;
        }
    }
    true
}

// Check if matrix A and B are able to be multiplied.
// This means the column rank of A must equal the row rank of B
fn is_matmul_compatible(a: &Matrix, b: &Matrix) -> bool {
    let num_cols_a: usize = a.data[0].len();
    let num_rows_b: usize = b.data.len();

    num_cols_a == num_rows_b
}


fn inner_product(a: &Vec<i32>, b: &Vec<i32>) -> i32 {
    let mut c: i32 = 0;
    for i in 0..a.len() {
        c += a[i] * b[i]
    }
    c
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_matmul_compatible() {
        let compat_a = Matrix { data:  vec![vec![1,2],vec![3,4]] };
        let compat_b = Matrix  { data:  vec![vec![1,2,3,4],vec![3,4,5,7]] };    
        assert_eq!(true, is_matmul_compatible(&compat_a, &compat_b));

        let incompat_a = Matrix { data: vec![vec![1,2],vec![3,4]] };
        let incompat_b = Matrix { data: vec![vec![1,2],vec![3,4], vec![5,6]] };
        assert_ne!(true, is_matmul_compatible(&incompat_a, &incompat_b));

    }

    #[test]
    fn test_matmul_correctness() {
        let a1: Vec<i32> = vec![1,2];
        let a2: Vec<i32> = vec![4,5];
        let b1: Vec<i32> = vec![7,8,9,46];
        let b2: Vec<i32> = vec![10,11,3,12];
        let a_mat = Matrix::new(vec![a1, a2]);
        let b_mat = Matrix::new(vec![b1, b2]);
        let c: Matrix = Matrix::new(vec![vec![27, 30, 15, 70], vec![78, 87, 51, 244]]);
        assert_eq!(a_mat * b_mat, c);
    }
}