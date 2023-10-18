mod logger;

use crate::logger::pylogger::{LogLevel, PyLogger};
use crate::logger::rust_logger::{JsonConfig, LogConfig, LogFileConfig};
use pyo3::prelude::*;

/// Python implementation for the Rusty Logger
#[pymodule]
fn _rusty_logger(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_class::<PyLogger>()?;
    m.add_class::<LogConfig>()?;
    m.add_class::<JsonConfig>()?;
    m.add_class::<LogLevel>()?;
    m.add_class::<LogFileConfig>()?;
    Ok(())
}
