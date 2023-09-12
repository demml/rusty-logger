use pyo3::prelude::*;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::env;
use std::io;
use std::path::Path;
use tracing_core::dispatcher::DefaultGuard;
use tracing_subscriber::filter::EnvFilter;
use tracing_subscriber::filter::LevelFilter;
use tracing_subscriber::{prelude::*, registry::LookupSpan, Layer};

#[pyclass]
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct LogConfig {
    #[pyo3(get, set)]
    pub stdout: bool,

    #[pyo3(get, set)]
    pub stderr: bool,

    #[pyo3(get, set)]
    pub filename: Option<String>,

    #[pyo3(get, set)]
    pub level: String,

    #[pyo3(get, set)]
    pub env: Option<String>,

    #[pyo3(get, set)]
    pub target: bool,

    #[pyo3(get, set)]
    pub span: bool,
}

#[pymethods]
impl LogConfig {
    #[new]
    pub fn new(
        stdout: Option<bool>,
        stderr: Option<bool>,
        filename: Option<String>,
        level: Option<String>,
        env: Option<String>,
        target: Option<bool>,
        span: Option<bool>,
    ) -> LogConfig {
        let log_level = level.unwrap_or_else(|| "INFO".to_string());
        let log_env = env.unwrap_or_else(|| match env::var("APP_ENV") {
            Ok(val) => val,
            Err(_e) => "development".to_string(),
        });

        let log_target = target.unwrap_or(false);
        let log_span = span.unwrap_or(false);

        LogConfig {
            stdout: stdout.unwrap_or(true),
            stderr: stderr.unwrap_or(false),
            filename,
            level: log_level,
            env: Some(log_env),
            target: log_target,
            span: log_span,
        }
    }
}

#[pyclass]
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct LogMetadata {
    #[pyo3(get, set)]
    pub info: HashMap<String, String>,
}

#[pymethods]
impl LogMetadata {
    #[new]
    pub fn new(info: HashMap<String, String>) -> LogMetadata {
        LogMetadata { info }
    }
}

/// Get the name of the file
///
/// # Arguments
///
/// * `name` - The name of the file
///
fn get_file_name<T: Into<String>>(name: Option<T>) -> String {
    let file_name = match name {
        Some(val) => val.into(),
        None => file!().to_string(),
    };

    Path::new(&file_name)
        .file_name()
        .and_then(|s| s.to_str())
        .unwrap()
        .to_string()
}

#[allow(clippy::single_char_pattern)]
fn get_log_directory(output: &str) -> String {
    let file_splits = output.split("/").to_owned().collect::<Vec<&str>>().clone();
    let directory = file_splits[..file_splits.len() - 1].join("/");

    if directory.is_empty() {
        "./logs".to_string()
    } else {
        directory
    }
}

/// A logger that outputs JSON
///
/// # Arguments
///
/// * `output` - The output of the logger. Either "stdout" or "stderr"
/// * `level` - The level of the logger. Either "info", "debug", "warn", or "error"
/// * `name` - The name of the file
///
///
#[allow(dead_code)]
pub struct JsonLogger {
    pub env: String,
    pub name: String,
    guard: DefaultGuard,
}

impl JsonLogger {
    /// Build the layers for the logger
    ///
    /// # Arguments
    ///
    /// * `log_config` - The configuration for the logger
    ///
    /// # Returns
    ///
    /// * `Vec<Box<dyn Layer<S> + Send + Sync>>` - The layers for the logger
    fn build_layers<S>(log_config: &LogConfig) -> Vec<Box<dyn Layer<S> + Send + Sync>>
    where
        S: tracing_core::Subscriber,
        for<'a> S: LookupSpan<'a>,
    {
        let mut layers = Vec::new();

        if log_config.stdout {
            let layer: Box<dyn Layer<S> + Send + Sync> = tracing_subscriber::fmt::layer()
                .with_target(log_config.target)
                .json()
                .with_current_span(log_config.span)
                .with_writer(io::stdout)
                .boxed();

            layers.push(layer);
        }

        if log_config.stderr {
            let layer = tracing_subscriber::fmt::layer()
                .with_target(log_config.target)
                .json()
                .with_current_span(log_config.span)
                .with_writer(io::stderr)
                .boxed();
            layers.push(layer);
        }

        if log_config.filename.is_some() {
            let file = log_config.filename.as_ref().unwrap().to_string();
            let directory = get_log_directory(&file);
            let file_name_prefix = get_file_name(Some(&file));
            let file_appender = tracing_appender::rolling::hourly(directory, file_name_prefix);
            let layer = tracing_subscriber::fmt::layer()
                .json()
                .with_target(log_config.target)
                .with_current_span(log_config.span)
                .with_writer(file_appender)
                .boxed();
            layers.push(layer);
        }

        layers
    }

    /// Create a new logger
    ///
    /// # Arguments
    ///
    /// * `output` - The output of the logger. Either "stdout" or "stderr"
    /// * `level` - The level of the logger. Either "info", "debug", "warn", or "error"
    /// * `name` - The name of the file
    ///
    pub fn new(log_config: LogConfig, name: Option<String>) -> JsonLogger {
        let layers = JsonLogger::build_layers(&log_config);
        let global_filter =
            EnvFilter::from_default_env().add_directive(match log_config.level.as_str() {
                "DEBUG" => LevelFilter::DEBUG.into(),
                "INFO" => LevelFilter::INFO.into(),
                "WARN" => LevelFilter::WARN.into(),
                "ERROR" => LevelFilter::ERROR.into(),
                _ => LevelFilter::INFO.into(),
            });
        let guard = tracing_subscriber::registry()
            .with(layers)
            .with(global_filter)
            .set_default();

        let file_name = get_file_name(name);

        Self {
            env: match env::var("APP_ENV") {
                Ok(val) => val,
                Err(_e) => "development".to_string(),
            },
            name: file_name,
            guard,
        }
    }

    /// Log an info message
    ///
    /// # Arguments
    ///
    /// * `message` - The message to log
    ///
    pub fn info(&self, message: &str, metadata: Option<LogMetadata>) {
        match metadata {
            Some(val) => tracing::info!(
                message = message,
                app_env = self.env,
                name = self.name,
                info = ?val.info
            ),
            None => tracing::info!(message = message, app_env = self.env, name = self.name),
        };
    }

    /// Log a debug message
    ///
    /// # Arguments
    ///
    /// * `message` - The message to log
    ///
    pub fn debug(&self, message: &str, metadata: Option<LogMetadata>) {
        match metadata {
            Some(val) => tracing::debug!(
                message = message,
                app_env = self.env,
                name = self.name,
                info = ?val.info
            ),
            None => tracing::debug!(message = message, app_env = self.env, name = self.name),
        };
    }

    /// Log a warning message
    ///
    /// # Arguments
    ///
    /// * `message` - The message to log
    ///
    pub fn warning(&self, message: &str, metadata: Option<LogMetadata>) {
        match metadata {
            Some(val) => tracing::warn!(
                message = message,
                app_env = self.env,
                name = self.name,
                info = ?val.info
            ),
            None => tracing::warn!(message = message, app_env = self.env, name = self.name),
        };
    }

    /// Log an error message
    ///
    /// # Arguments
    ///
    /// * `message` - The message to log
    ///
    pub fn error(&self, message: &str, metadata: Option<LogMetadata>) {
        match metadata {
            Some(val) => tracing::error!(
                message = message,
                app_env = self.env,
                name = self.name,
                info = ?val.info
            ),
            None => tracing::error!(message = message, app_env = self.env, name = self.name),
        };
    }
}

#[cfg(test)]
mod tests {
    use super::{JsonLogger, LogConfig};

    #[test]
    fn test_stdout_logger() {
        let config = LogConfig {
            stdout: true,
            stderr: false,
            filename: None,
            level: "INFO".to_string(),
            env: None,
            target: false,
            span: false,
        };
        let logger = JsonLogger::new(config, None);
        logger.info("test", None);
        logger.debug("test", None);
        logger.warning("test", None);
        logger.error("test", None);
    }

    #[test]
    fn test_stderr_logger() {
        let config = LogConfig {
            stdout: true,
            stderr: false,
            filename: None,
            level: "INFO".to_string(),
            env: None,
            target: false,
            span: false,
        };
        let logger = JsonLogger::new(config, None);
        logger.info("test", None);
        logger.debug("test", None);
        logger.warning("test", None);
        logger.error("test", None);
    }
}
