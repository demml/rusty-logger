use crate::logger::rust_logger::{JsonConfig, LogConfig, RustLogger};
use pyo3::prelude::*;
use pyo3::types::PyTuple;
use serde_json::{json, to_string_pretty};

#[derive(FromPyObject, Debug)]
enum PyTypes<'a> {
    CatchAll(&'a PyAny),
}

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

pub fn parse_args(args: &PyTuple) -> Vec<String> {
    let args = args
        .iter()
        .map(|x| match x.extract::<PyTypes>() {
            Ok(PyTypes::CatchAll(c)) => c.to_string(),
            Err(e) => {
                println!("Error: {}", e);
                "".to_string()
            }
        })
        .collect::<Vec<String>>();

    args
}

#[pyclass(name = "RustyLogger", subclass)]
pub struct PyLogger {
    logger: RustLogger,

    pub config: LogConfig,
    pub name: Option<String>,
}

#[pymethods]
#[allow(unused_variables)]
impl PyLogger {
    /// Create a new logger
    ///
    /// # Arguments
    /// * `config` - The log config to use
    ///
    /// # Returns
    /// A new logger
    #[new]
    pub fn new(config: Option<LogConfig>) -> PyResult<Self> {
        let log_config = config.unwrap_or_else(|| {
            // get default
            LogConfig::new(None, None, None, None, None, None, None, None, None, None)
        });

        let logger = RustLogger::new(&log_config);
        Ok(PyLogger {
            logger,
            config: log_config,
            name: None,
        })
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

    /// Update the name used for the logger
    fn update_name(&mut self, name: String) {
        let mut config = self.config.clone();
        config.update_name(name);
        self.config = config;
    }

    /// Drop the guard for the logger
    fn drop_guard(&mut self) {
        if self.logger.guard.is_some() {
            self.logger.guard.take();
        }
    }

    #[getter]
    pub fn get_name(&self) -> Option<&String> {
        self.config.name.as_ref()
    }

    #[setter]
    pub fn set_name(&mut self, name: String) {
        let mut config = self.config.clone();
        config.update_name(name);
        self.config = config;
    }

    #[getter]
    pub fn get_config(&self) -> LogConfig {
        self.config.clone()
    }

    #[setter]
    pub fn set_config(&mut self, config: LogConfig) {
        self.drop_guard();
        self.logger = RustLogger::new(&config);
    }

    pub fn json(&mut self) {
        let mut config = self.config.clone();
        config.json_config = Some(JsonConfig::new(None));
        self.config = config;

        self.drop_guard();
        self.logger = RustLogger::new(&self.config);
    }

    /// Log at INFO level
    ///
    /// # Arguments
    /// * `message` - The message to log
    /// * `args` - The arguments to log
    ///
    #[pyo3(signature = (message, *args))]
    pub fn info(&self, message: &str, args: &PyTuple) {
        let args = if args.is_empty() {
            None
        } else {
            Some(parse_args(args))
        };

        self.logger.info(message, args, &self.config);
    }

    /// Log at DEBUG level
    ///
    /// # Arguments
    /// * `message` - The message to log
    /// * `args` - The arguments to log
    ///
    #[pyo3(signature = (message, *args))]
    pub fn debug(&self, message: &str, args: &PyTuple) {
        let args = if args.is_empty() {
            None
        } else {
            Some(parse_args(args))
        };
        self.logger.debug(message, args, &self.config);
    }

    /// Log at WARN level
    ///
    /// # Arguments
    /// * `message` - The message to log
    /// * `args` - The arguments to log
    ///
    #[pyo3(signature = (message, *args))]
    pub fn warning(&self, message: &str, args: &PyTuple) {
        let args = if args.is_empty() {
            None
        } else {
            Some(parse_args(args))
        };
        self.logger.warning(message, args, &self.config);
    }

    /// Log at ERROR level
    ///
    /// # Arguments
    /// * `message` - The message to log
    /// * `args` - The arguments to log
    /// * `metadata` - The metadata to log
    ///
    #[pyo3(signature = (message, *args))]
    pub fn error(&self, message: &str, args: &PyTuple) {
        let args = if args.is_empty() {
            None
        } else {
            Some(parse_args(args))
        };
        self.logger.error(message, args, &self.config);
    }

    /// Log at TRACE level
    ///
    /// # Arguments
    /// * `message` - The message to log
    /// * `args` - The arguments to log
    /// * `metadata` - The metadata to log
    ///
    #[pyo3(signature = (message, *args))]
    pub fn trace(&self, message: &str, args: &PyTuple) {
        let args = if args.is_empty() {
            None
        } else {
            Some(parse_args(args))
        };
        self.logger.trace(message, args, &self.config);
    }

    /// String magic method for PyLogger class
    pub fn __str__(&self) -> PyResult<String> {
        let json = json!({
            "type": "Logger",
            "name": self.config.name,
            "level": self.config.app_env,
            "config": self.config,
        });

        Ok(to_string_pretty(&json).unwrap())
    }
}

#[cfg(test)]
mod tests {
    use super::{JsonConfig, LogConfig, PyLogger};
    use crate::logger::rust_logger::LogFileConfig;
    use pyo3::prelude::*;
    use pyo3::types::PyTuple;
    use std::fs::File;
    use std::io::{self, BufRead};
    use std::path::Path;

    fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
    where
        P: AsRef<Path>,
    {
        let file = File::open(filename)?;
        Ok(io::BufReader::new(file).lines())
    }

    #[test]
    fn test_pylogger_json() {
        pyo3::prepare_freethreaded_python();
        Python::with_gil(|py| {
            let config = LogConfig::new(
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                Some(JsonConfig::new(None)),
                Some(LogFileConfig::new(Some("log/test.log".to_string()), None)),
                Some(true),
            );
            let elements: Vec<i32> = vec![0, 1];
            let tuple: &PyTuple = PyTuple::new(py, elements);
            let mut logger = PyLogger::new(Some(config)).unwrap();
            logger.json();
            logger.info("Hello World {} {}", tuple);

            let lines = read_lines("log/test.log").unwrap();
            let mut count = 0;
            for line in lines {
                if let Ok(line) = line {
                    if line.contains("Hello World 0 1") {
                        count += 1;

                        // try loading into json
                        serde_json::from_str::<serde_json::Value>(&line).unwrap();
                    }
                }
            }
            assert_eq!(count, 1);
            std::fs::remove_dir_all("log").unwrap();
        });
    }

    #[test]
    fn test_pylogger() {
        pyo3::prepare_freethreaded_python();
        Python::with_gil(|py| {
            let elements: Vec<i32> = vec![0, 1];
            let tuple: &PyTuple = PyTuple::new(py, elements);
            let mut logger = PyLogger::new(None).unwrap();
            logger.info("Hello World {} {}", tuple);
            logger.json();
            logger.info("Hello World {} {}", tuple);
        });
    }
}
