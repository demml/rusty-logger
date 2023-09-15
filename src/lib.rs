mod logger;

use crate::logger::pylogger::{LogLevel, PyJsonLogger};
use crate::logger::rust_logger::{JsonConfig, LogConfig, LogMetadata};
use pyo3::prelude::*;

/// Python implementation for the Rusty Logger
#[pymodule]
fn rusty_logger(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_class::<PyJsonLogger>()?;
    m.add_class::<LogConfig>()?;
    m.add_class::<LogMetadata>()?;
    m.add_class::<JsonConfig>()?;
    m.add_class::<LogLevel>()?;
    Ok(())
}
