use pyo3::prelude::*;

#[pyfunction]
fn greet(name: &str) -> PyResult<String> {
    Ok(format!("Hello, {}!", name))
}

#[pymodule]
fn volto(py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(greet, m)?)?;
    Ok(())
}
