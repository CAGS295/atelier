
// lib.rs : Final script where to place wrapped functionality

pub mod data_types;
use pyo3::prelude::*;

/// Specific functionality to be exposed to python usage
/// within the definition of the exposed MODULE
#[pyfunction]
fn sum(a: usize, b: usize) -> PyResult<String> {
    Ok((a + b).to_string())
}

/// Definition of the exposed MODULE
/// should be the same as 
/// pyproject.toml/tool.setuptools-rust.ext-modules[target]
#[pymodule]
fn rs_atelier(m: &Bound<'_, PyModule>) -> PyResult<()> {
    
    m.add_function(wrap_pyfunction!(sum, m)?)?;
    
    Ok(())
}
