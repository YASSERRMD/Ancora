use std::collections::HashMap;
use std::sync::Mutex;

use pyo3::prelude::*;

use crate::error::AncorError;

pub(crate) struct InnerRuntime {
    pub _runs: Mutex<HashMap<String, ()>>,
    pub _store: ancora_core::journal::MemoryStore,
}

impl InnerRuntime {
    fn new() -> Self {
        Self {
            _runs: Mutex::new(HashMap::new()),
            _store: ancora_core::journal::MemoryStore::new(),
        }
    }
}

/// Handle to the Ancora agent runtime.
///
/// The runtime manages the in-process execution state for agent runs.
/// Call :meth:`free` when done, or use the runtime as a context manager.
#[pyclass(name = "Runtime")]
pub struct PyRuntime {
    inner: Option<Box<InnerRuntime>>,
}

#[pymethods]
impl PyRuntime {
    /// Create a new Runtime instance.
    #[new]
    pub fn new() -> PyResult<Self> {
        Ok(Self {
            inner: Some(Box::new(InnerRuntime::new())),
        })
    }

    /// Release the runtime resources. Subsequent calls are no-ops.
    pub fn free(&mut self) {
        self.inner = None;
    }

    /// Return True if the runtime has been freed.
    #[getter]
    pub fn is_freed(&self) -> bool {
        self.inner.is_none()
    }

    pub fn __repr__(&self) -> String {
        if self.inner.is_some() {
            "Runtime(active)".to_string()
        } else {
            "Runtime(freed)".to_string()
        }
    }

    fn __enter__(slf: PyRef<'_, Self>) -> PyRef<'_, Self> {
        slf
    }

    fn __exit__(
        &mut self,
        _exc_type: Option<&Bound<'_, PyAny>>,
        _exc_val: Option<&Bound<'_, PyAny>>,
        _exc_tb: Option<&Bound<'_, PyAny>>,
    ) -> PyResult<bool> {
        self.free();
        Ok(false)
    }
}

impl Drop for PyRuntime {
    fn drop(&mut self) {
        self.inner = None;
    }
}
