use crate::logger::rust_logger::{LogConfig, RustLogger};
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

#[pyclass(name = "Logger", subclass)]
pub struct PyLogger {
    logger: RustLogger,

    #[pyo3(get, set)]
    pub config: LogConfig,
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
    pub fn update_name(&mut self, name: String) {
        let mut config = self.config.clone();
        config.update_name(name);
        self.config = config;
    }

    /// Drop the guard for the logger
    pub fn drop_guard(&mut self) {
        if self.logger.guard.is_some() {
            self.logger.guard.take();
        }
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
        } else if args.is_none() {
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
        } else if args.is_none() {
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
        } else if args.is_none() {
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
        } else if args.is_none() {
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
        } else if args.is_none() {
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
