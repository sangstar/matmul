use pyo3::prelude::*;
use pyo3::wrap_pyfunction;

mod linalg;
use linalg::Matrix;

#[pymodule]
fn rublas(py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(matmul, m)?)?;
    Ok(())
}

#[pyfunction]
fn matmul(a: Vec<Vec<i32>>, b: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
    let mat_a: Matrix = Matrix::new(a);
    let mat_b: Matrix = Matrix::new(b);

    let c = mat_a * mat_b;
    c.data
}