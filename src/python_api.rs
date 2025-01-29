use pyo3::prelude::*;
use pyo3::wrap_pyfunction;

#[pyfunction]
fn transform_data(input: &str) -> PyResult<String> {
    // Placeholder transformation logic
    Ok(format!("Transformed: {}", input))
}

#[pymodule]
fn volto(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(transform_data, m)?)?;
    Ok(())
}
