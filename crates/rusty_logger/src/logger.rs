use crate::error::LoggingError;
use dynfmt::{Format, SimpleCurlyFormat};
use pyo3::prelude::*;
use pyo3::types::PyTuple;
use pyo3::types::PyTupleMethods;
use serde::{Deserialize, Serialize};
use std::io;
use std::str::FromStr;
use tracing_subscriber;
use tracing_subscriber::fmt::time::UtcTime;

#[pyclass(eq)]
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize, Default)]
pub enum LogLevel {
    Debug,
    #[default]
    Info,
    Warn,
    Error,
    Trace,
}

impl FromStr for LogLevel {
    type Err = LoggingError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "debug" => Ok(LogLevel::Debug),
            "info" => Ok(LogLevel::Info),
            "warn" => Ok(LogLevel::Warn),
            "error" => Ok(LogLevel::Error),
            "trace" => Ok(LogLevel::Trace),
            _ => Ok(LogLevel::Info),
        }
    }
}

#[pyclass(eq)]
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize, Default)]
pub enum WriteLevel {
    #[default]
    Stdout,
    Stderror,
}

#[allow(clippy::len_zero)] // len tends to be faster than is_empty in tests
fn format_string(message: &str, args: &Vec<String>) -> String {
    if args.len() > 0 {
        SimpleCurlyFormat
            .format(message, args)
            .unwrap_or_else(|_| message.into())
            .to_string()
    } else {
        message.to_string()
    }
}

pub fn parse_args(args: &Bound<'_, PyTuple>) -> Option<Vec<String>> {
    if args.is_empty() {
        None
    } else {
        Some(args.iter().map(|x| x.to_string()).collect())
    }
}

const DEFAULT_TIME_PATTERN: &str =
    "[year]-[month]-[day]T[hour repr:24]:[minute]:[second]::[subsecond digits:4]";

fn build_json_subscriber(
    log_level: tracing::Level,
    config: &LoggingConfig,
) -> Result<(), LoggingError> {
    let sub = tracing_subscriber::fmt()
        .with_max_level(log_level)
        .json()
        .with_target(false)
        .flatten_event(true)
        .with_thread_ids(config.show_threads.clone())
        .with_timer(config.time_format()?);

    if config.write_level == WriteLevel::Stderror {
        sub.with_writer(io::stderr).try_init().map_err(|e| {
            LoggingError::Error(format!("Failed to setup logging with error: {}", e))
        })?;
    } else {
        sub.with_writer(io::stdout).try_init().map_err(|e| {
            LoggingError::Error(format!("Failed to setup logging with error: {}", e))
        })?;
    }
    Ok(())
}

fn build_subscriber(log_level: tracing::Level, config: &LoggingConfig) -> Result<(), LoggingError> {
    let sub = tracing_subscriber::fmt()
        .with_max_level(log_level)
        .with_target(false)
        .with_thread_ids(config.show_threads.clone())
        .with_timer(config.time_format()?);

    if config.write_level == WriteLevel::Stderror {
        sub.with_writer(io::stderr).try_init().map_err(|e| {
            LoggingError::Error(format!("Failed to setup logging with error: {}", e))
        })?;
    } else {
        sub.with_writer(io::stdout).try_init().map_err(|e| {
            LoggingError::Error(format!("Failed to setup logging with error: {}", e))
        })?;
    }
    Ok(())
}

pub fn setup_logging(config: &LoggingConfig) -> Result<(), LoggingError> {
    let display_level = match &config.log_level {
        LogLevel::Debug => tracing::Level::DEBUG,
        LogLevel::Info => tracing::Level::INFO,
        LogLevel::Warn => tracing::Level::WARN,
        LogLevel::Error => tracing::Level::ERROR,
        LogLevel::Trace => tracing::Level::TRACE,
    };

    if config.use_json {
        return build_json_subscriber(display_level, config);
    } else {
        return build_subscriber(display_level, config);
    }
}

#[pyclass]
#[derive(Clone, Default)]
pub struct LoggingConfig {
    #[pyo3(get, set)]
    show_threads: bool,

    #[pyo3(get, set)]
    log_level: LogLevel,

    #[pyo3(get, set)]
    write_level: WriteLevel,

    #[pyo3(get, set)]
    use_json: bool,
}

#[pymethods]
impl LoggingConfig {
    #[new]
    #[pyo3(signature = (show_threads=None, log_level=None, write_level=WriteLevel::Stdout, use_json=false))]
    pub fn new(
        show_threads: Option<bool>,
        log_level: Option<LogLevel>,
        write_level: Option<WriteLevel>,
        use_json: Option<bool>,
    ) -> Self {
        let show_threads = show_threads.unwrap_or(true);
        let log_level = log_level.unwrap_or(LogLevel::Info);
        let write_level = write_level.unwrap_or(WriteLevel::Stdout);
        let use_json = use_json.unwrap_or(false);
        LoggingConfig {
            show_threads,
            log_level,
            write_level,
            use_json,
        }
    }
}

impl LoggingConfig {
    fn time_format(
        &self,
    ) -> Result<UtcTime<Vec<time::format_description::FormatItem<'static>>>, LoggingError> {
        let formatter = UtcTime::new(
            time::format_description::parse(DEFAULT_TIME_PATTERN).map_err(|e| {
                LoggingError::Error(format!(
                    "Failed to parse time format: {} with error: {}",
                    DEFAULT_TIME_PATTERN, e
                ))
            })?,
        );

        Ok(formatter)
    }
}

#[pyclass]
pub struct RustyLogger {}

#[pymethods]
impl RustyLogger {
    #[staticmethod]
    #[pyo3(signature = (config=None))]
    pub fn setup_logging(config: Option<LoggingConfig>) -> Result<(), LoggingError> {
        let config = config.unwrap_or(LoggingConfig::default());
        let _ = setup_logging(&config).is_ok();

        Ok(())
    }

    #[staticmethod]
    #[pyo3(signature = (config=None))]
    pub fn get_logger(config: Option<LoggingConfig>) -> Result<Self, LoggingError> {
        let config = config.unwrap_or(LoggingConfig::default());
        let _ = setup_logging(&config).is_ok();

        Ok(RustyLogger {})
    }

    #[pyo3(signature = (message, *args))]
    pub fn info(&self, message: &str, args: &Bound<'_, PyTuple>) {
        let args = parse_args(args);
        let msg = match args {
            Some(val) => format_string(message, &val),
            None => message.to_string(),
        };
        tracing::info!(msg);
    }

    #[pyo3(signature = (message, *args))]
    pub fn debug(&self, message: &str, args: &Bound<'_, PyTuple>) {
        let args = parse_args(args);
        let msg = match args {
            Some(val) => format_string(message, &val),
            None => message.to_string(),
        };
        tracing::debug!(msg);
    }

    #[pyo3(signature = (message, *args))]
    pub fn warn(&self, message: &str, args: &Bound<'_, PyTuple>) {
        let args = parse_args(args);
        let msg = match args {
            Some(val) => format_string(message, &val),
            None => message.to_string(),
        };
        tracing::warn!(msg);
    }

    #[pyo3(signature = (message, *args))]
    pub fn error(&self, message: &str, args: &Bound<'_, PyTuple>) {
        let args = parse_args(args);
        let msg = match args {
            Some(val) => format_string(message, &val),
            None => message.to_string(),
        };
        tracing::error!(msg);
    }

    #[pyo3(signature = (message, *args))]
    pub fn trace(&self, message: &str, args: &Bound<'_, PyTuple>) {
        let args = parse_args(args);
        let msg = match args {
            Some(val) => format_string(message, &val),
            None => message.to_string(),
        };
        tracing::trace!(msg);
    }
}
