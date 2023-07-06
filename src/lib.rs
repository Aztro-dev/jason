use pyo3::prelude::*;

#[pyclass]
struct Jason {
    #[pyo3(get)]
    annoying_level: u64,
}

/// Sets jason's annoying level
#[pymethods]
impl Jason {
    #[new]
    pub fn new(_value: u64) -> Self {
        return Jason {
            annoying_level: 999_999_999,
        };
    }
}

/// Literally just println lmfao
#[pyfunction]
fn speak(message: &str) -> PyResult<()> {
    println!("{}", message);
    return Ok(());
}

/// A BLAZINGLY FAST Module
#[pymodule]
fn jason(_py: Python, module: &PyModule) -> PyResult<()> {
    module.add_class::<Jason>()?;
    module.add_function(wrap_pyfunction!(speak, module)?)?;
    Ok(())
}

