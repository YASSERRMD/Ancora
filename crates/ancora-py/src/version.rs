use pyo3::prelude::*;

/// Return the Ancora Python SDK version string.
#[pyfunction(name = "version")]
pub fn py_version() -> &'static str {
    env!("CARGO_PKG_VERSION")
}
