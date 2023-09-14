use pyo3::prelude::*;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::env;
use std::io;
use std::path::Path;
use time::format_description::FormatItem;
use tracing_core::dispatcher::DefaultGuard;
use tracing_subscriber::filter::EnvFilter;
use tracing_subscriber::filter::LevelFilter;

use tracing_subscriber::fmt::time::UtcTime;
use tracing_subscriber::fmt::MakeWriter;
use tracing_subscriber::prelude::__tracing_subscriber_Layer;
use tracing_subscriber::prelude::__tracing_subscriber_SubscriberExt;
use tracing_subscriber::registry::LookupSpan;
use tracing_subscriber::util::SubscriberInitExt;
use tracing_subscriber::Layer;

#[pyclass]
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct JsonConfig {
    #[pyo3(get, set)]
    pub span: bool,

    #[pyo3(get, set)]
    pub flatten: bool,
}

#[pymethods]
#[allow(clippy::too_many_arguments)]
impl JsonConfig {
    #[new]
    pub fn new(span: Option<bool>, flatten: Option<bool>) -> JsonConfig {
        JsonConfig {
            span: span.unwrap_or(false),
            flatten: flatten.unwrap_or(true),
        }
    }
}

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
    pub line_number: bool,

    #[pyo3(get, set)]
    pub time_format: Option<String>,

    #[pyo3(get, set)]
    pub json_config: Option<JsonConfig>,
}

#[pymethods]
#[allow(clippy::too_many_arguments)]
impl LogConfig {
    // py init
    #[new]
    pub fn new(
        stdout: Option<bool>,
        stderr: Option<bool>,
        filename: Option<String>,
        level: Option<String>,
        env: Option<String>,
        target: Option<bool>,
        line_number: Option<bool>,
        time_format: Option<String>,
        json_config: Option<JsonConfig>,
    ) -> LogConfig {
        let log_env = env.unwrap_or_else(|| match env::var("APP_ENV") {
            Ok(val) => val,
            Err(_e) => "development".to_string(),
        });
        let time_format = time_format
            .unwrap_or_else(|| "[year]-[month]-[day]T[hour]:[minute]:[second]".to_string());

        let json_log_config = match json_config {
            Some(val) => Some(val),
            None => Some(JsonConfig::new(None, None)),
        };

        LogConfig {
            stdout: stdout.unwrap_or(true),
            stderr: stderr.unwrap_or(false),
            filename,
            level: level.unwrap_or_else(|| "INFO".to_string()),
            env: Some(log_env),
            target: target.unwrap_or(false),
            line_number: line_number.unwrap_or(false),
            time_format: Some(time_format),
            json_config: json_log_config,
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

#[derive(Debug, Clone)]
pub struct LoggTimer<'a> {
    pub timer: UtcTime<Vec<FormatItem<'a>>>,
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

fn get_file_params(log_config: &LogConfig) -> (String, String) {
    let file = log_config.filename.as_ref().unwrap().to_string();
    let directory = get_log_directory(&file);
    let file_name_prefix = get_file_name(Some(&file));

    (directory, file_name_prefix)
}

/// Rust logging class
///
/// # Arguments
///
/// * `output` - The output of the logger. Either "stdout" or "stderr"
/// * `level` - The level of the logger. Either "info", "debug", "warn", or "error"
/// * `name` - The name of the file
///
///
#[allow(dead_code)]
pub struct RustLogger {
    pub env: String,
    pub name: String,
    guard: DefaultGuard,
}

impl RustLogger {
    /// Create a new logger
    ///
    /// # Arguments
    ///
    /// * `output` - The output of the logger. Either "stdout" or "stderr"
    /// * `level` - The level of the logger. Either "info", "debug", "warn", or "error"
    /// * `name` - The name of the file
    ///
    pub fn new(log_config: &LogConfig, name: Option<String>) -> RustLogger {
        let layers = RustLogger::build_layers(log_config);
        let global_filter =
            EnvFilter::from_default_env().add_directive(match log_config.level.as_str() {
                "DEBUG" => LevelFilter::DEBUG.into(),
                "INFO" => LevelFilter::INFO.into(),
                "WARN" => LevelFilter::WARN.into(),
                "ERROR" => LevelFilter::ERROR.into(),
                "TRACE" => LevelFilter::TRACE.into(),
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

    /// Build the json layer for the logger
    ///
    /// # Arguments
    ///
    /// * `log_config` - The configuration for the logger
    /// * `writer` - The writer for the logger
    /// * `timer` - The timer for the logger
    ///
    /// # Returns
    ///
    /// * `Box<dyn Layer<S> + Send + Sync>` - The layer for the logger
    fn construct_json_layer<W2, S>(
        log_config: &LogConfig,
        writer: W2,
        timer: UtcTime<Vec<FormatItem<'static>>>,
    ) -> Box<dyn __tracing_subscriber_Layer<S> + Send + Sync>
    where
        S: tracing_core::Subscriber,
        W2: for<'writer> MakeWriter<'writer> + 'static + Send + Sync,
        for<'a> S: LookupSpan<'a>,
    {
        let flatten = log_config.json_config.as_ref().unwrap().flatten;
        let span = log_config.json_config.as_ref().unwrap().span;

        let layer = tracing_subscriber::fmt::layer()
            .with_target(log_config.target)
            .json()
            .flatten_event(flatten)
            .with_current_span(span)
            .with_line_number(log_config.line_number.to_owned())
            .with_timer(timer)
            .with_writer(writer)
            .boxed();

        layer
    }

    /// Build the json layers for the logger
    ///
    /// # Arguments
    ///
    /// * `log_config` - The configuration for the logger
    ///
    /// # Returns
    ///
    /// * `Vec<Box<dyn Layer<S> + Send + Sync>>` - The layers for the logger
    fn construct_json_layers<S>(log_config: &LogConfig) -> Vec<Box<dyn Layer<S> + Send + Sync>>
    where
        S: tracing_core::Subscriber,
        for<'a> S: LookupSpan<'a>,
    {
        let mut layers = Vec::new();

        // set time format (applies to all layers)
        let time_format =
            time::format_description::parse("hello").expect("Failed to parse time format");
        let log_timer = LoggTimer {
            timer: UtcTime::new(time_format).to_owned(),
        };

        if log_config.stdout {
            layers.push(RustLogger::construct_json_layer(
                log_config,
                io::stdout,
                log_timer.timer.clone(),
            ));
        }

        if log_config.stderr {
            layers.push(RustLogger::construct_json_layer(
                log_config,
                io::stderr,
                log_timer.timer.clone(),
            ));
        }

        if log_config.filename.is_some() {
            let (directory, file_name_prefix) = get_file_params(log_config);
            let file_appender = tracing_appender::rolling::hourly(directory, file_name_prefix);
            layers.push(RustLogger::construct_json_layer(
                log_config,
                file_appender,
                log_timer.timer.clone(),
            ));
        }

        layers
    }

    fn construct_cmd_layer<W2, S>(
        log_config: &LogConfig,
        writer: W2,
        timer: UtcTime<Vec<FormatItem<'static>>>,
    ) -> Box<dyn __tracing_subscriber_Layer<S> + Send + Sync>
    where
        S: tracing_core::Subscriber,
        W2: for<'writer> MakeWriter<'writer> + 'static + Send + Sync,
        for<'a> S: LookupSpan<'a>,
    {
        let layer = tracing_subscriber::fmt::layer()
            .with_target(log_config.target)
            .with_line_number(log_config.line_number)
            .with_timer(timer)
            .with_writer(writer)
            .boxed();

        layer
    }

    /// Build the layers for the logger
    ///
    /// # Arguments
    ///
    /// * `log_config` - The configuration for the logger
    ///
    /// # Returns
    ///
    /// * `Vec<Box<dyn Layer<S> + Send + Sync>>` - The layers for the logger
    fn construct_cmd_layers<S>(log_config: &LogConfig) -> Vec<Box<dyn Layer<S> + Send + Sync>>
    where
        S: tracing_core::Subscriber,
        for<'a> S: LookupSpan<'a>,
    {
        let mut layers = Vec::new();
        // set time format (applies to all layers)
        let time_format =
            time::format_description::parse("hello").expect("Failed to parse time format");
        let log_timer = LoggTimer {
            timer: UtcTime::new(time_format).to_owned(),
        };

        if log_config.stdout {
            let writer = io::stdout;
            layers.push(RustLogger::construct_cmd_layer(
                log_config,
                writer,
                log_timer.timer.clone(),
            ));
        }

        if log_config.stderr {
            let writer = io::stderr;
            layers.push(RustLogger::construct_cmd_layer(
                log_config,
                writer,
                log_timer.timer.clone(),
            ));
        }

        if log_config.filename.is_some() {
            let (directory, file_name_prefix) = get_file_params(log_config);
            let writer = tracing_appender::rolling::hourly(directory, file_name_prefix);
            layers.push(RustLogger::construct_cmd_layer(
                log_config,
                writer,
                log_timer.timer.clone(),
            ));
        }

        layers
    }

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
        if log_config.json_config.is_some() {
            RustLogger::construct_json_layers(log_config)
        } else {
            RustLogger::construct_cmd_layers(log_config)
        }
    }

    /// Log an info message
    ///
    /// # Arguments
    ///
    /// * `message` - The message to log
    ///
    pub fn info(&self, message: &str, metadata: Option<&LogMetadata>) {
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
    pub fn debug(&self, message: &str, metadata: Option<&LogMetadata>) {
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
    pub fn warning(&self, message: &str, metadata: Option<&LogMetadata>) {
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
    pub fn error(&self, message: &str, metadata: Option<&LogMetadata>) {
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

    /// Log an trace message
    ///
    /// # Arguments
    ///
    /// * `message` - The message to log
    ///
    pub fn trace(&self, message: &str, metadata: Option<&LogMetadata>) {
        match metadata {
            Some(val) => tracing::error!(
                message = message,
                app_env = self.env,
                name = self.name,
                info = ?val.info
            ),
            None => tracing::trace!(message = message, app_env = self.env, name = self.name),
        };
    }
}

#[cfg(test)]
mod tests {
    use super::{JsonConfig, LogConfig, LogMetadata, RustLogger};

    fn generate_test_json_config(level: String, stdout: bool, stderr: bool) -> LogConfig {
        LogConfig {
            stdout,
            stderr,
            filename: None,
            level,
            env: None,
            target: false,
            json_config: Some(JsonConfig::new(None, None)),
            line_number: false,
            time_format: None,
        }
    }

    #[test]
    fn test_stdout_logger() {
        let levels = vec!["INFO", "DEBUG", "WARN", "ERROR", "TRACE"];

        levels.iter().for_each(|level| {
            let config = generate_test_json_config(level.to_string(), true, false);
            let logger = RustLogger::new(&config, None);
            logger.info("test", None);
            logger.debug("test", None);
            logger.warning("test", None);
            logger.error("test", None);
            logger.trace("test", None);
        });
    }

    #[test]
    fn test_stderr_logger() {
        let levels = vec!["INFO", "DEBUG", "WARN", "ERROR", "TRACE"];
        let metadata = LogMetadata {
            info: std::collections::HashMap::from([("Mercury".to_string(), "Mercury".to_string())]),
        };

        levels.iter().for_each(|level| {
            let config = generate_test_json_config(level.to_string(), false, true);
            let logger = RustLogger::new(&config, None);
            logger.info("test", Some(&metadata));
            logger.debug("test", Some(&metadata));
            logger.warning("test", Some(&metadata));
            logger.error("test", Some(&metadata));
            logger.trace("test", Some(&metadata));
        });
    }
}
