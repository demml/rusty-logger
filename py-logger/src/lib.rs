use ::rusty_logging::logger::{LogLevel, LoggingConfig, RustyLogger, WriteLevel};
use pyo3::prelude::*;
/// Python implementation for the Rusty Logger
///
#[pymodule]
fn rusty_logger(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_class::<RustyLogger>()?;
    m.add_class::<LoggingConfig>()?;
    m.add_class::<LogLevel>()?;
    m.add_class::<WriteLevel>()?;
    Ok(())
}
