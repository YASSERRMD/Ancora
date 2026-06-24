use pyo3::prelude::*;
use pyo3::exceptions::PyException;

/// Exception raised by the Ancora Python SDK.
#[pyclass(name = "AncorError", extends = PyException)]
#[derive(Debug)]
pub struct AncorError {}

#[pymethods]
impl AncorError {
    #[new]
    pub fn new(msg: String) -> (Self, PyException) {
        (AncorError {}, PyException::new_err(msg).into())
    }
}
