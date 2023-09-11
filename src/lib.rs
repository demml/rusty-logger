mod logger;

use crate::logger::pylogger::PyJsonLogger;
use pyo3::prelude::*;

/// A Python module implemented in Rust.
/// Name must match cargo lib name
#[pymodule]
fn rusty_logger(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_class::<PyJsonLogger>()?;

    Ok(())
}
