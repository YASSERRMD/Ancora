use pyo3::exceptions::PyException;
use pyo3::create_exception;

create_exception!(_ancora, AncorError, PyException, "Raised by the Ancora Python SDK.");
