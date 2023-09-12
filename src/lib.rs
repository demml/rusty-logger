mod logger;

use crate::logger::pylogger::PyJsonLogger;
use crate::logger::rust_logger::{LogConfig, LogMetadata};
use pyo3::prelude::*;

/// Python implementation for the Rusty Logger
#[pymodule]
fn rusty_logger(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_class::<PyJsonLogger>()?;
    m.add_class::<LogConfig>()?;
    m.add_class::<LogMetadata>()?;
    Ok(())
}
