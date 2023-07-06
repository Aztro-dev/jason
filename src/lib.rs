use pyo3::prelude::*;

/// Formats the sum of two numbers as string.
#[pyfunction]
fn annoying_level() -> PyResult<u64> {
    return Ok(999_999_999);
}

/// A Python module implemented in Rust.
#[pymodule]
fn jason(_py: Python, module: &PyModule) -> PyResult<()> {
    module.add_function(wrap_pyfunction!(annoying_level, module)?)?;
    Ok(())
}

