use pyo3::prelude::*;

mod error;
mod runtime;
mod version;

pub use error::AncorError;
pub use runtime::PyRuntime;
pub use version::py_version;

/// Ancora Python SDK: PyO3 bindings for the Ancora agent runtime.
#[pymodule]
fn _ancora(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_class::<PyRuntime>()?;
    m.add_function(wrap_pyfunction!(py_version, m)?)?;
    let err_type = m.py().get_type_bound::<AncorError>();
    err_type.setattr("ErrOk", 0)?;
    err_type.setattr("ErrInternal", 1)?;
    err_type.setattr("ErrNotFound", 2)?;
    err_type.setattr("ErrInvalidArg", 3)?;
    m.add("AncorError", err_type)?;
    Ok(())
}
