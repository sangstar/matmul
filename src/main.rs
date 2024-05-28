use std::fmt;
use std::fmt::Formatter;

struct Matrix {
    data: Vec<Vec<i32>>,
}

impl Matrix {
    fn new(data: Vec<Vec<i32>>) -> Matrix {
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

impl fmt::Display for Matrix {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        println!("\n");
        for row in &self.data {
            for elem in row {
                println!("{}", elem);
            }
        }
        Ok (())
    }
}

fn main() {
    let a1: Vec<i32> = vec![1,2];
    let a2: Vec<i32> = vec![4,5];
    let b1: Vec<i32> = vec![7,8];
    let b2: Vec<i32> = vec![10,11];
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
    for i in 0..a.data.len() {
        let mut c_row: Vec<i32> = vec![];
        for j in 0..a.data[i].len() {
            let b_col: Vec<i32> = b.get_column_vector(j as i32);
            c_row.push(inner_product(&a.data[i], &b_col));
        }
        c_data.push(c_row);
    }
    let matrix = Matrix { data: c_data };
    matrix
}


fn inner_product(a: &Vec<i32>, b: &Vec<i32>) -> i32 {
    let mut c: i32 = 0;
    for i in 0..a.len() {
        c += a[i] * b[i]
    }
    c
}

