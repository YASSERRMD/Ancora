use pyo3::create_exception;
use pyo3::exceptions::PyException;

create_exception!(
    _ancora,
    AncorError,
    PyException,
    "Raised by the Ancora Python SDK."
);
