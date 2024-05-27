use std::fmt;
use std::fmt::Formatter;

struct Matrix {
    data: Vec<Vec<i32>>,
}

impl Matrix {
    fn new(data: Vec<Vec<i32>>) -> Matrix {
        if !has_uniform_row_length(&data) {
            panic!("vectors must have same length");
        }
        let matrix = Matrix { data };
        matrix
    }
}

impl fmt::Display for Matrix {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        for row in &self.data {
            for elem in row {
                println!("{}", elem);
            }
        }
        Ok (())
    }
}

fn main() {
    let a1: Vec<i32> = vec![1,2,3];
    let a2: Vec<i32> = vec![4,5,6];
    let b1: Vec<i32> = vec![7,8,9];
    let b2: Vec<i32> = vec![10,11,12];
    let a_mat = Matrix::new(vec![a1, a2]);
    let b_mat = Matrix::new(vec![b1, b2]);
    println!("Matmul is {}", matmul(&a_mat, &b_mat));
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

fn matmul(a: &Matrix, b: &Matrix) -> Matrix {
    let mut c_data: Vec<Vec<i32>> = vec![];
    for i in 0..a.data.len()-1 {
        let mut c_row: Vec<i32> = vec![];
        for j in 0..a.data[i].len()-1 {
            c_row.push(a.data[i][j] * b.data[j][i]);
        }
        c_data.push(c_row);
    }
    let matrix = Matrix { data: c_data };
    matrix
}


// Possibly not needed
fn inner_product(a: &Vec<i32>, b: &Vec<i32>) -> i32 {
    let mut c: i32 = 0;
    for i in 0..a.len() {
        c += a[i] * b[i]
    }
    c
}

