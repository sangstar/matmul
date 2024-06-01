use pyo3::prelude::*;
use pyo3::wrap_pyfunction;

mod linalg;
use linalg::Matrix;

#[pymodule]
fn rublas(py: Python, m: &PyModule) -> PyResult<()> {
    m.add_class::<Matrix>()?;
    Ok(())
}
