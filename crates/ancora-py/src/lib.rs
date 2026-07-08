// pyo3 0.22's macro expansions (create_exception!, #[pymethods]) reference
// cfgs and PyErr conversions that this crate never declares; both are
// harmless artifacts of the macro internals, not real issues in this crate.
#![allow(unexpected_cfgs)]
#![allow(clippy::useless_conversion)]

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
