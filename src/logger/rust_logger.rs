use pyo3::prelude::*;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::env;
use std::io;
use std::path::Path;
use tracing_core::dispatcher::DefaultGuard;
use tracing_subscriber::filter::EnvFilter;
use tracing_subscriber::filter::LevelFilter;
use tracing_subscriber::fmt::time::UtcTime;
use tracing_subscriber::fmt::MakeWriter;
use tracing_subscriber::prelude::__tracing_subscriber_SubscriberExt;
use tracing_subscriber::registry::LookupSpan;
use tracing_subscriber::util::SubscriberInitExt;
use tracing_subscriber::Layer;

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

    #[pyo3(get, set)]
    pub flatten: bool,

    #[pyo3(get, set)]
    pub json: bool,

    #[pyo3(get, set)]
    pub line_number: bool,

    #[pyo3(get, set)]
    pub time_format: Option<String>,
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
        flatten: Option<bool>,
        json: Option<bool>,
        line_number: Option<bool>,
        time_format: Option<String>,
    ) -> LogConfig {
        let log_level = level.unwrap_or_else(|| "INFO".to_string());
        let log_env = env.unwrap_or_else(|| match env::var("APP_ENV") {
            Ok(val) => val,
            Err(_e) => "development".to_string(),
        });

        let log_target = target.unwrap_or(false);
        let log_span = span.unwrap_or(false);
        let log_flatten = flatten.unwrap_or(true);
        let log_json = json.unwrap_or(true);
        let log_line = line_number.unwrap_or(false);
        let time_format = time_format
            .unwrap_or_else(|| "[year]-[month]-[day]T[hour]:[minute]:[second]".to_string());

        LogConfig {
            stdout: stdout.unwrap_or(true),
            stderr: stderr.unwrap_or(false),
            filename,
            level: log_level,
            env: Some(log_env),
            target: log_target,
            span: log_span,
            flatten: log_flatten,
            json: log_json,
            line_number: log_line,
            time_format: Some(time_format),
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

fn get_file_params(log_config: &LogConfig) -> (String, String) {
    let file = log_config.filename.as_ref().unwrap().to_string();
    let directory = get_log_directory(&file);
    let file_name_prefix = get_file_name(Some(&file));

    (directory, file_name_prefix)
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
pub struct RustLogger {
    pub env: String,
    pub name: String,
    guard: DefaultGuard,
}

impl RustLogger {
    fn construct_json_layer<W2, S>(
        log_config: &LogConfig,
        writer: W2,
        layers: &mut Vec<Box<dyn Layer<S> + Send + Sync>>,
    ) -> ()
    where
        S: tracing_core::Subscriber,
        W2: for<'writer> MakeWriter<'writer> + 'static + Send + Sync,
        for<'a> S: LookupSpan<'a>,
    {
        let time_format = time::format_description::parse(log_config.time_format.as_ref().unwrap())
            .expect("Failed to parse time format");
        let timer = UtcTime::new(time_format);

        layers.push(
            tracing_subscriber::fmt::layer()
                .with_target(log_config.target)
                .json()
                .flatten_event(log_config.flatten)
                .with_current_span(log_config.span)
                .with_line_number(log_config.line_number)
                .with_timer(timer)
                .with_writer(writer)
                .boxed(),
        );
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

        if log_config.stdout {
            RustLogger::construct_json_layer(log_config, io::stdout, &mut layers);
        }

        if log_config.stderr {
            RustLogger::construct_json_layer(log_config, io::stderr, &mut layers);
        }

        if log_config.filename.is_some() {
            let (directory, file_name_prefix) = get_file_params(log_config);
            let file_appender = tracing_appender::rolling::hourly(directory, file_name_prefix);
            RustLogger::construct_json_layer(log_config, file_appender, &mut layers);
        }

        layers
    }

    fn construct_cmd_layer<W2, S>(
        log_config: &LogConfig,
        writer: W2,
        layers: &Vec<Box<dyn Layer<S> + Send + Sync>>,
    ) -> ()
    where
        S: tracing_core::Subscriber,
        W2: for<'writer> MakeWriter<'writer> + 'static + Send + Sync,
        for<'a> S: LookupSpan<'a>,
    {
        let time_format = time::format_description::parse(log_config.time_format.as_ref().unwrap())
            .expect("Failed to parse time format");

        let timer = UtcTime::new(time_format);
        let layer = tracing_subscriber::fmt::layer()
            .with_target(log_config.target)
            .with_line_number(log_config.line_number)
            .with_timer(timer)
            .with_writer(writer)
            .boxed();

        layers.push(layer)
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
        if log_config.stdout {
            RustLogger::construct_cmd_layer(log_config, io::stdout, &layers);
        }

        if log_config.stderr {
            RustLogger::construct_cmd_layer(log_config, io::stderr, &layers);
        }

        if log_config.filename.is_some() {
            let (directory, file_name_prefix) = get_file_params(log_config);
            let file_appender = tracing_appender::rolling::hourly(directory, file_name_prefix);
            RustLogger::construct_cmd_layer(log_config, file_appender, &layers);
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
        if log_config.json {
            RustLogger::construct_json_layers(log_config)
        } else {
            RustLogger::construct_cmd_layers(log_config)
        }
    }

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
    use super::{LogConfig, RustLogger};

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
            flatten: false,
            json: true,
            line_number: false,
            time_format: None,
        };
        let logger = RustLogger::new(&config, None);
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
            flatten: false,
            json: true,
            line_number: false,
            time_format: None,
        };
        let logger = RustLogger::new(&config, None);
        logger.info("test", None);
        logger.debug("test", None);
        logger.warning("test", None);
        logger.error("test", None);
    }
}
