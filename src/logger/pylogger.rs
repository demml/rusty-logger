use std::env::Args;
use std::hash::Hash;

use crate::logger::rust_logger::{LogConfig, LogMetadata, RustLogger};
use pyo3::prelude::*;
use pyo3::types::PyString;
use pyo3::types::{PyDict, PyList, PyTuple, PyType};
use serde_json::{json, to_string_pretty};
use std::collections::HashMap;
use std::fmt;
use tracing_subscriber::fmt::format;

#[derive(FromPyObject, Debug)]
enum PyTypes<'a> {
    #[pyo3(transparent, annotation = "str")]
    String(String),
    #[pyo3(transparent, annotation = "int")]
    Int(i64),
    #[pyo3(transparent, annotation = "float")]
    Float(f64),
    #[pyo3(transparent, annotation = "dict")]
    PyDict(HashMap<String, &'a PyAny>),
    #[pyo3(transparent, annotation = "list")]
    PyList(Vec<&'a PyAny>),
    #[pyo3(transparent)]
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

#[pyclass(name = "Logger", subclass)]
pub struct PyJsonLogger {
    logger: RustLogger,

    #[pyo3(get, set)]
    pub config: LogConfig,
}

pub fn parse_args(args: &PyTuple) -> Vec<String> {
    let args = args
        .iter()
        .map(|x| match x.extract::<PyTypes>() {
            Ok(PyTypes::String(s)) => s,
            Ok(PyTypes::Int(i)) => i.to_string(),
            Ok(PyTypes::Float(f)) => f.to_string(),
            Ok(PyTypes::PyDict(d)) => {
                let mut dict = HashMap::new();
                for (k, v) in d {
                    dict.insert(k.clone(), v.to_string());
                }
                to_string_pretty(&dict).unwrap()
            }
            Ok(PyTypes::PyList(l)) => {
                let mut list = Vec::new();
                for v in l {
                    list.push(v.to_string());
                }
                to_string_pretty(&list).unwrap()
            }
            Ok(PyTypes::CatchAll(c)) => c.to_string(),
            Err(e) => {
                println!("Error: {}", e);
                "".to_string()
            }
        })
        .collect::<Vec<String>>();

    args
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
            LogConfig::new(None, None, None, None, None, None, None, None, None, None)
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

    #[pyo3(signature = (message, *args, metadata=None))]
    pub fn info(&self, message: &str, args: &PyTuple, metadata: Option<LogMetadata>) {
        let args = if args.is_empty() {
            None
        } else if args.is_none() {
            None
        } else {
            Some(parse_args(args))
        };

        self.logger.info(message, args, metadata.as_ref());
    }

    #[pyo3(signature = (message, *args, metadata=None))]
    pub fn debug(&self, message: &str, args: &PyTuple, metadata: Option<LogMetadata>) {
        let args = if args.is_empty() {
            None
        } else if args.is_none() {
            None
        } else {
            Some(parse_args(args))
        };
        self.logger.debug(message, args, metadata.as_ref());
    }

    #[pyo3(signature = (message, *args, metadata=None))]
    pub fn warning(&self, message: &str, args: &PyTuple, metadata: Option<LogMetadata>) {
        let args = if args.is_empty() {
            None
        } else if args.is_none() {
            None
        } else {
            Some(parse_args(args))
        };
        self.logger.warning(message, args, metadata.as_ref());
    }

    #[pyo3(signature = (message, *args, metadata=None))]
    pub fn error(&self, message: &str, args: &PyTuple, metadata: Option<LogMetadata>) {
        let args = if args.is_empty() {
            None
        } else if args.is_none() {
            None
        } else {
            Some(parse_args(args))
        };
        self.logger.error(message, args, metadata.as_ref());
    }

    #[pyo3(signature = (message, *args, metadata=None))]
    pub fn trace(&self, message: &str, args: &PyTuple, metadata: Option<LogMetadata>) {
        let args = if args.is_empty() {
            None
        } else if args.is_none() {
            None
        } else {
            Some(parse_args(args))
        };
        self.logger.trace(message, args, metadata.as_ref());
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
