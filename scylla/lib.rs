use pyo3::prelude::*;

#[pyfunction]
pub fn tryme() -> PyResult<String> {
    Ok("Rust works!".to_string())
}

#[pymodule]
fn scylla(_py: Python, module: &PyModule) -> PyResult<()> {
    module.add_function(wrap_pyfunction!(tryme, module)?)?;
    Ok(())
}