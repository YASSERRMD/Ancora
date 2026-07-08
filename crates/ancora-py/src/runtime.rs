use std::collections::{HashMap, VecDeque};
use std::sync::Mutex;

use pyo3::exceptions::PyRuntimeError;
use pyo3::prelude::*;

pub(crate) struct InnerRun {
    pub id: String,
    pub events: VecDeque<String>,
}

impl InnerRun {
    pub fn new(id: &str, spec: &str) -> Self {
        let escaped_spec = spec.replace('\\', "\\\\").replace('"', "\\\"");
        let mut events = VecDeque::new();
        events.push_back(format!(
            r#"{{"kind":"started","run_id":"{}","spec":"{}"}}"#,
            id, escaped_spec
        ));
        for token in &["Hello", " ", "world"] {
            events.push_back(format!(
                r#"{{"kind":"token","run_id":"{}","text":"{}"}}"#,
                id, token
            ));
        }
        events.push_back(format!(r#"{{"kind":"completed","run_id":"{}"}}"#, id));
        Self {
            id: id.to_string(),
            events,
        }
    }

    pub fn poll_event(&mut self) -> Option<String> {
        self.events.pop_front()
    }

    pub fn resume(&mut self, decision: &str) {
        self.events.push_back(format!(
            r#"{{"kind":"resumed","run_id":"{}","decision":"{}"}}"#,
            self.id, decision
        ));
        self.events
            .push_back(format!(r#"{{"kind":"completed","run_id":"{}"}}"#, self.id));
    }
}

pub(crate) struct InnerRuntime {
    pub runs: Mutex<HashMap<String, InnerRun>>,
    pub _store: ancora_core::journal::MemoryStore,
}

impl InnerRuntime {
    pub fn new() -> Self {
        Self {
            runs: Mutex::new(HashMap::new()),
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

    /// Start a new agent run from JSON spec bytes. Returns the run ID string.
    pub fn start_run(&mut self, spec_bytes: &[u8]) -> PyResult<String> {
        let inner = self
            .inner
            .as_mut()
            .ok_or_else(|| PyRuntimeError::new_err("Runtime has been freed"))?;
        let spec_str = String::from_utf8_lossy(spec_bytes).into_owned();
        let run_id = uuid::Uuid::new_v4().to_string();
        let run = InnerRun::new(&run_id, &spec_str);
        inner.runs.lock().unwrap().insert(run_id.clone(), run);
        Ok(run_id)
    }

    /// Poll the next event for a run. Returns None when all events are consumed.
    pub fn poll_run(&mut self, run_id: &str) -> PyResult<Option<Vec<u8>>> {
        let inner = self
            .inner
            .as_mut()
            .ok_or_else(|| PyRuntimeError::new_err("Runtime has been freed"))?;
        let mut guard = inner.runs.lock().unwrap();
        let event = guard.get_mut(run_id).and_then(|r| r.poll_event());
        Ok(event.map(|s| s.into_bytes()))
    }

    /// Resume a suspended run with a decision payload.
    pub fn resume_run(&mut self, run_id: &str, decision: &[u8]) -> PyResult<()> {
        let inner = self
            .inner
            .as_mut()
            .ok_or_else(|| PyRuntimeError::new_err("Runtime has been freed"))?;
        let decision_str = String::from_utf8_lossy(decision).into_owned();
        let mut guard = inner.runs.lock().unwrap();
        if let Some(run) = guard.get_mut(run_id) {
            run.resume(&decision_str);
        }
        Ok(())
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

    #[pyo3(signature = (_exc_type=None, _exc_val=None, _exc_tb=None))]
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
