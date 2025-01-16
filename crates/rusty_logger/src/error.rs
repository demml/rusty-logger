use pyo3::PyErr;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum LoggingError {
    #[error("Logging Error: {0}")]
    Error(String),
}

impl From<LoggingError> for PyErr {
    fn from(err: LoggingError) -> PyErr {
        PyErr::new::<pyo3::exceptions::PyValueError, _>(err.to_string())
    }
}
