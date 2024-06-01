use std::fmt;
use std::fmt::Formatter;
use std::ops::Mul;
use std::ops::Add;
use pyo3::prelude::*;

// TODO: Check if docstrings are not idiomatic 
// TODO: Add Python-side docstrings
#[derive(Debug)]
#[derive(Clone)]
#[pyclass]
pub struct Matrix {
    pub data: Vec<Vec<i32>>,
    num_rows: i32,
    num_cols: i32
}


// Define methods for Matrix
#[pymethods]
impl Matrix {
    fn __mul__(&self, rhs: Matrix) -> Self {
        let mat: Matrix = self.clone();
        mat.mul(rhs)
    }

    fn __add__(&self, rhs: Matrix) -> Self {
        let mat: Matrix = self.clone();
        mat.add(rhs)
    }

    fn __repr__(&self) -> String {
        let mut output = String::from("");
        for row in &self.data {
            output += "[";
            for &value in row {
                output += &value.to_string();
                output += " ";
            }
            output += "]\n";
        }
        output
    }

    #[new]
    fn init(data: Vec<Vec<i32>>) -> Matrix {
       Matrix::new(data)
    }
}

impl Matrix {
    pub fn new(data: Vec<Vec<i32>>) -> Self {
        if !has_uniform_row_length(&data) {
            panic!("Vectors must have same length");
        }
        let num_rows = get_num_rows(&data);
        let num_cols = get_num_cols(&data);
        let matrix = Matrix { data, num_rows, num_cols };
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


fn get_num_rows(data: &Vec<Vec<i32>>) -> i32 {
    data.len() as i32
}

fn get_num_cols(data: &Vec<Vec<i32>>) -> i32 {
    data[0].len() as i32
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
    fn mul(self, rhs: Self) -> Self {
        let mut c_data: Vec<Vec<i32>> = vec![];
        if !is_matmul_compatible(&self, &rhs) {
            panic!("Matrices are not compatible for multiplication")
        }
        for i in 0..self.num_rows {
            let mut c_row: Vec<i32> = vec![];
            for j in 0..rhs.num_cols {
                let b_col: Vec<i32> = rhs.get_column_vector(j as i32);
                c_row.push(inner_product(&self.data[i as usize], &b_col));
            }
            c_data.push(c_row);
        }
        let matrix = Matrix::new(c_data);
        matrix
    }
}

impl Add for Matrix {
    type Output = Self;
    fn add(self, rhs: Self) -> Self {
        let mut c_data: Vec<Vec<i32>> = vec![];
        if !is_add_compatible(&self, &rhs) {
            panic!("Matrices must have same dimension for adding")
        }
        for i in 0..self.num_rows {
            let mut c_row: Vec<i32> = vec![];
            for j in 0..self.num_cols {
                c_row.push(&self.data[i as usize][j as usize] + &rhs.data[i as usize][j as usize])
            }
            c_data.push(c_row);
        }
        let matrix = Matrix::new(c_data);
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

fn is_add_compatible(a: &Matrix, b: &Matrix) -> bool {
    for i in 0..a.num_rows {
        if a.data[i as usize].len() != b.data[i as usize].len() {
            return false;
        }
    }
    true
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
        let compat_a = Matrix::new(vec![vec![1,2],vec![3,4]]);
        let compat_b = Matrix::new(vec![vec![1,2,3,4],vec![3,4,5,7]]);
        assert_eq!(true, is_matmul_compatible(&compat_a, &compat_b));

        let incompat_a = Matrix::new(vec![vec![1,2],vec![3,4]]);
        let incompat_b = Matrix::new(vec![vec![1,2],vec![3,4], vec![5,6]]);
        assert_ne!(true, is_matmul_compatible(&incompat_a, &incompat_b));

    }

    #[test]
    fn test_matmul_correctness() {
        let a1: Vec<i32> = vec![1,2];
        let a2: Vec<i32> = vec![4,5];
        let a3: Vec<i32> = vec![1,3];
        let b1: Vec<i32> = vec![7,8,9,46];
        let b2: Vec<i32> = vec![10,11,3,12];
        let a_mat = Matrix::new(vec![a1, a2, a3]);
        let b_mat = Matrix::new(vec![b1, b2]);
        let c: Matrix = Matrix::new(vec![vec![27, 30, 15, 70], vec![78, 87, 51, 244], vec![37, 41, 18, 82]]);
        assert_eq!(a_mat * b_mat, c);
    }
}