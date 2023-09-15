use crate::logger::rust_logger::{LogConfig, LogMetadata, RustLogger};
use pyo3::prelude::*;
use pyo3::types::PyType;
use serde_json::{json, to_string_pretty};

#[pyclass(name = "Logger")]
#[derive(Debug)]
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
            // get default
            LogConfig::new(None, None, None, None, None, None, None, None)
        });

        let logger = RustLogger::new(&log_config, name);

        PyJsonLogger { logger }
    }

    pub fn info(&self, message: &str, metadata: Option<LogMetadata>) {
        self.logger.info(message, metadata.as_ref());
    }

    pub fn debug(&self, message: &str, metadata: Option<LogMetadata>) {
        self.logger.debug(message, metadata.as_ref());
    }

    pub fn warning(&self, message: &str, metadata: Option<LogMetadata>) {
        self.logger.warning(message, metadata.as_ref());
    }

    pub fn error(&self, message: &str, metadata: Option<LogMetadata>) {
        self.logger.error(message, metadata.as_ref());
    }

    pub fn trace(&self, message: &str, metadata: Option<LogMetadata>) {
        self.logger.trace(message, metadata.as_ref());
    }

    pub fn __str__(&self) -> PyResult<String> {
        let json = json!({
            "type": "Logger",
            "name": self.logger.name,
            "level": self.logger.env,
            "config": self.logger.config,
        });

        Ok(to_string_pretty(&json).unwrap())
    }
}
