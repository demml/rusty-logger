use pyo3::prelude::*;
mod logger;
use logger::logger::JsonLogger;

/// A Python module implemented in Rust.
#[pymodule]
fn rusty_logger(_py: Python, m: &PyModule) -> PyResult<()> {
    Ok(())
}
