use crate::logger::rust_logger::{LogConfig, LogMetadata, RustLogger};
use pyo3::prelude::*;
use pyo3::types::PyType;
use serde_json::{json, to_string_pretty};

#[pyclass]
pub struct LogLevel {}

#[pymethods]
#[allow(non_snake_case)]
impl LogLevel {
    #[classattr]
    fn DEBUG() -> String {
        "DEBUG".to_string()
    }

    #[classattr]
    fn WARN() -> String {
        "WARN".to_string()
    }

    #[classattr]
    fn INFO() -> String {
        "INFO".to_string()
    }

    #[classattr]
    fn ERROR() -> String {
        "ERROR".to_string()
    }

    #[classattr]
    fn TRACE() -> String {
        "TRACE".to_string()
    }
}

#[pyclass(name = "Logger", subclass)]
pub struct PyJsonLogger {
    logger: RustLogger,
    pub config: LogConfig,
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

        PyJsonLogger {
            logger,
            config: log_config,
        }
    }

    /// Set the log level for the logger
    ///
    /// # Arguments
    /// * `level` - The log level to set
    ///
    pub fn set_level(&mut self, level: String) {
        let mut config = self.config.clone();
        config.log_level(level);
        self.logger.reload_level(&config.level).unwrap()
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
