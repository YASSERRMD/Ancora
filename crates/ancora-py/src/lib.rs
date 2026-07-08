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
    m.add("AncorError", m.py().get_type_bound::<AncorError>())?;
    Ok(())
}
