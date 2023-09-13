use crate::logger::rust_logger::{LogConfig, LogMetadata, RustLogger};
use pyo3::prelude::*;
use pyo3::types::PyType;

#[pyclass(name = "Logger")]
pub struct PyJsonLogger {
    logger: RustLogger,
}

#[pymethods]
#[allow(unused_variables)]
impl PyJsonLogger {
    #[classmethod]
    pub fn get_logger(
        cls: &PyType,
        name: Option<String>,
        config: Option<LogConfig>,
    ) -> PyJsonLogger {
        let log_config = config.unwrap_or_else(|| {
            LogConfig::new(
                None, None, None, None, None, None, None, None, None, None, None,
            )
        });

        let logger = RustLogger::new(&log_config, name);

        PyJsonLogger { logger }
    }

    pub fn info(&self, message: &str, metadata: Option<LogMetadata>) {
        self.logger.info(message, metadata);
    }

    pub fn debug(&self, message: &str, metadata: Option<LogMetadata>) {
        self.logger.debug(message, metadata);
    }

    pub fn warning(&self, message: &str, metadata: Option<LogMetadata>) {
        self.logger.warning(message, metadata);
    }

    pub fn error(&self, message: &str, metadata: Option<LogMetadata>) {
        self.logger.error(message, metadata);
    }
}
