use crate::logger::logger::JsonLogger;
use pyo3::prelude::*;
use pyo3::types::PyType;

#[pyclass(name = "JsonLogger")]
pub struct PyJsonLogger {
    logger: JsonLogger,
}

#[pymethods]
impl PyJsonLogger {
    #[classmethod]
    pub fn get_logger(
        cls: &PyType,
        name: Option<String>,
        output: Option<String>,
        level: Option<String>,
    ) -> PyJsonLogger {
        let logger = JsonLogger::new(
            output.unwrap_or("stdout".to_string()),
            level.unwrap_or("info".to_string()),
            name,
        );

        PyJsonLogger { logger: logger }
    }

    pub fn info(&self, message: &str) -> () {
        self.logger.info(message);
    }

    pub fn debug(&self, message: &str) -> () {
        self.logger.debug(message);
    }

    pub fn warning(&self, message: &str) -> () {
        self.logger.warning(message);
    }

    pub fn error(&self, message: &str) -> () {
        self.logger.error(message);
    }
}
