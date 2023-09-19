use pyo3::prelude::*;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::env;
use std::io;
use std::path::Path;
use tracing_core::dispatcher::DefaultGuard;
use tracing_subscriber::filter::LevelFilter;
use tracing_subscriber::layer::Layered;

use owo_colors::OwoColorize;
use time::format_description::FormatItem;
use tracing_subscriber::fmt::time::UtcTime;
use tracing_subscriber::fmt::MakeWriter;
use tracing_subscriber::prelude::__tracing_subscriber_Layer;
use tracing_subscriber::prelude::__tracing_subscriber_SubscriberExt;
use tracing_subscriber::registry::LookupSpan;
use tracing_subscriber::reload;
use tracing_subscriber::util::SubscriberInitExt;
use tracing_subscriber::Layer;
use tracing_subscriber::Registry;

static DEFAULT_TIME_PATTERN: &str =
    "[year]-[month]-[day]T[hour repr:24]:[minute]:[second]::[subsecond digits:4]";

type ReloadHandle =
    reload::Handle<LevelFilter, Layered<Vec<Box<dyn Layer<Registry> + Send + Sync>>, Registry>>;

#[pyclass(dict)]
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct JsonConfig {
    #[pyo3(get, set)]
    pub flatten: bool,
}

#[pymethods]
impl JsonConfig {
    #[new]
    pub fn new(flatten: Option<bool>) -> Self {
        JsonConfig {
            flatten: flatten.unwrap_or(true),
        }
    }

    pub fn __str__(&self) -> PyResult<String> {
        Ok(serde_json::to_string_pretty(&self).unwrap())
    }
}

#[pyclass(dict)]
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct LogFileConfig {
    #[pyo3(get, set)]
    pub filename: String,

    #[pyo3(get, set)]
    pub rotate: String,
}

#[pymethods]
impl LogFileConfig {
    #[new]
    pub fn new(filename: Option<String>, rotate: Option<String>) -> Self {
        LogFileConfig {
            filename: filename.unwrap_or("log/logs.log".to_string()),
            rotate: rotate.unwrap_or("never".to_string()),
        }
    }

    pub fn __str__(&self) -> PyResult<String> {
        Ok(serde_json::to_string_pretty(&self).unwrap())
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
    pub level: String,

    #[pyo3(get, set)]
    pub app_env: String,

    #[pyo3(get, set)]
    pub target: bool,

    #[pyo3(get, set)]
    pub time_format: String,

    #[pyo3(get, set)]
    pub json_config: Option<JsonConfig>,

    #[pyo3(get, set)]
    pub file_config: Option<LogFileConfig>,
}

#[pymethods]
#[allow(clippy::too_many_arguments)]
impl LogConfig {
    // py init
    #[new]
    pub fn new(
        stdout: Option<bool>,
        stderr: Option<bool>,
        level: Option<String>,
        app_env: Option<String>,
        target: Option<bool>,
        time_format: Option<String>,
        json_config: Option<JsonConfig>,
        file_config: Option<LogFileConfig>,
    ) -> Self {
        let log_env = match app_env {
            Some(val) => val,
            None => match env::var("APP_ENV") {
                Ok(val) => val,
                Err(_e) => "development".to_string(),
            },
        };

        let stdout = stdout.unwrap_or(false);
        let stderr = stderr.unwrap_or(false);

        let stdout = if !stdout && !stderr && file_config.is_none() {
            let msg = format!(
                "{}: {}. {}",
                "Invalid LogConfig".bold().red(),
                "No output specified",
                "Defaulting to stdout".green(),
            );
            println!("{}", msg);

            true
        } else {
            stdout
        };

        LogConfig {
            stdout,
            stderr,
            level: level.unwrap_or_else(|| "INFO".to_string()),
            app_env: log_env,
            target: target.unwrap_or(false),
            time_format: time_format.unwrap_or_else(|| DEFAULT_TIME_PATTERN.to_string()),
            json_config,
            file_config,
        }
    }

    pub fn log_level(&mut self, level: String) {
        self.level = level;
    }

    pub fn __str__(&self) -> PyResult<String> {
        Ok(serde_json::to_string_pretty(&self).unwrap())
    }
}

#[pyclass]
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct LogMetadata {
    #[pyo3(get, set)]
    pub data: HashMap<String, String>,
}

#[pymethods]
impl LogMetadata {
    #[new]
    pub fn new(data: HashMap<String, String>) -> LogMetadata {
        LogMetadata { data }
    }

    pub fn __str__(&self) -> PyResult<String> {
        Ok(serde_json::to_string_pretty(&self).unwrap())
    }
}

/// Get the name of the file
///
/// # Arguments
///
/// * `name` - The name of the file
///
fn get_file_name(filename: &str) -> String {
    Path::new(&filename)
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

fn get_file_params(file_config: &LogFileConfig) -> (String, String) {
    let directory = get_log_directory(&file_config.filename);
    let file_name_prefix = get_file_name(&file_config.filename);

    (directory, file_name_prefix)
}

/// Get the file appender
///
/// # Arguments
///
/// * `rotate` - The rotation type
/// * `directory` - The directory to write the file to
/// * `file_name_prefix` - The prefix of the file name
///
/// # Returns
///
/// * `RollingFileAppender` - The file appender
fn get_file_appender(
    rotate: &str,
    directory: &str,
    file_name_prefix: &str,
) -> tracing_appender::rolling::RollingFileAppender {
    if rotate == "hourly" {
        tracing_appender::rolling::hourly(directory, file_name_prefix)
    } else if rotate == "daily" {
        tracing_appender::rolling::daily(directory, file_name_prefix)
    } else if rotate == "minutely" {
        tracing_appender::rolling::minutely(directory, file_name_prefix)
    } else {
        tracing_appender::rolling::never(directory, file_name_prefix)
    }
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
    pub config: LogConfig,
    guard: DefaultGuard,
    reload_handle: ReloadHandle,
}

impl RustLogger {
    pub fn get_timer(time_format: String) -> UtcTime<Vec<FormatItem<'static>>> {
        let time = Box::new(time_format);
        let time_format_result = time::format_description::parse(Box::leak(time).as_str());

        // handle invalid user time format
        // Very rare that this would fail but let's handle it anyway
        let time_format = time_format_result.unwrap_or_else(|error| {
            println!("{}: {}", "Invalid time format:".bold().red(), error);
            println!("Defaulting to pattern: {}", DEFAULT_TIME_PATTERN.green());

            time::format_description::parse(DEFAULT_TIME_PATTERN).unwrap()
        });

        UtcTime::new(time_format)
    }

    pub fn get_level_filter(level: &str) -> LevelFilter {
        match level {
            "DEBUG" => LevelFilter::DEBUG,
            "INFO" => LevelFilter::INFO,
            "WARN" => LevelFilter::WARN,
            "ERROR" => LevelFilter::ERROR,
            "TRACE" => LevelFilter::TRACE,
            _ => LevelFilter::INFO,
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
        let filter = RustLogger::get_level_filter(&log_config.level);
        let (filter, reload_handle) = reload::Layer::new(filter);

        let guard = tracing_subscriber::registry()
            .with(layers)
            .with(filter)
            .set_default();

        let logger_filename = get_file_name(&name.unwrap_or("default".to_string()));

        Self {
            env: match env::var("APP_ENV") {
                Ok(val) => val,
                Err(_e) => "development".to_string(),
            },
            name: logger_filename,
            guard,
            config: log_config.clone(),
            reload_handle,
        }
    }

    pub fn reload_level(&mut self, level: &str) -> Result<(), reload::Error> {
        let filter = RustLogger::get_level_filter(level);
        self.reload_handle.reload(filter)
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
    ) -> Box<dyn __tracing_subscriber_Layer<S> + Send + Sync>
    where
        S: tracing_core::Subscriber,
        W2: for<'writer> MakeWriter<'writer> + 'static + Send + Sync,
        for<'a> S: LookupSpan<'a>,
    {
        let flatten = log_config.json_config.as_ref().unwrap().flatten;
        let timer = RustLogger::get_timer(log_config.time_format.clone());

        let layer = tracing_subscriber::fmt::layer()
            .with_target(log_config.target)
            .json()
            .flatten_event(flatten)
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

        if log_config.stdout {
            layers.push(RustLogger::construct_json_layer(log_config, io::stdout));
        }

        if log_config.stderr {
            layers.push(RustLogger::construct_json_layer(log_config, io::stderr));
        }

        if log_config.file_config.is_some() {
            let (directory, file_name_prefix) =
                get_file_params(log_config.file_config.as_ref().unwrap());
            let file_appender = get_file_appender(
                &log_config.file_config.as_ref().unwrap().rotate,
                &directory,
                &file_name_prefix,
            );
            layers.push(RustLogger::construct_json_layer(log_config, file_appender));
        }

        layers
    }

    fn construct_cmd_layer<W2, S>(
        log_config: &LogConfig,
        writer: W2,
    ) -> Box<dyn __tracing_subscriber_Layer<S> + Send + Sync>
    where
        S: tracing_core::Subscriber,
        W2: for<'writer> MakeWriter<'writer> + 'static + Send + Sync,
        for<'a> S: LookupSpan<'a>,
    {
        let timer = RustLogger::get_timer(log_config.time_format.clone());
        let layer = tracing_subscriber::fmt::layer()
            .with_target(log_config.target)
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

        if log_config.stdout {
            let writer = io::stdout;
            layers.push(RustLogger::construct_cmd_layer(log_config, writer));
        }

        if log_config.stderr {
            let writer = io::stderr;
            layers.push(RustLogger::construct_cmd_layer(log_config, writer));
        }

        if log_config.file_config.is_some() {
            let (directory, file_name_prefix) =
                get_file_params(log_config.file_config.as_ref().unwrap());
            let writer = get_file_appender(
                &log_config.file_config.as_ref().unwrap().rotate,
                &directory,
                &file_name_prefix,
            );

            layers.push(RustLogger::construct_cmd_layer(log_config, writer));
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
                metadata = ?val.data
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
                metadata = ?val.data
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
                metadata = ?val.data
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
                metadata = ?val.data
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
                metadata = ?val.data
            ),
            None => tracing::trace!(message = message, app_env = self.env, name = self.name),
        };
    }
}

#[cfg(test)]
mod tests {
    use super::{JsonConfig, LogConfig, LogFileConfig, LogMetadata, RustLogger};

    fn generate_test_file_config(
        level: String,
        stdout: bool,
        stderr: bool,
        rotate: &str,
        filename: &str,
    ) -> LogConfig {
        LogConfig {
            stdout,
            stderr,
            level,
            app_env: "development".to_string(),
            target: false,
            time_format:
                "[year]-[month]-[day] [hour repr:24]:[minute]:[second]::[subsecond digits:4]"
                    .to_string(),
            json_config: Some(JsonConfig::new(None)),
            file_config: Some(LogFileConfig::new(
                Some(filename.to_string()),
                Some(rotate.to_string()),
            )),
        }
    }

    fn generate_test_json_config(level: String, stdout: bool, stderr: bool) -> LogConfig {
        LogConfig {
            stdout,
            stderr,
            level,
            app_env: "development".to_string(),
            target: false,
            time_format:
                "[year]-[month]-[day] [hour repr:24]:[minute]:[second]::[subsecond digits:4]"
                    .to_string(),
            json_config: Some(JsonConfig::new(None)),
            file_config: None,
        }
    }

    fn generate_test_incorrect_config() -> LogConfig {
        LogConfig {
            stdout: false,
            stderr: false,
            level: "INFO".to_string(),
            app_env: "development".to_string(),
            target: false,
            time_format:
                "[year]-[month]-[day] [hour repr:24]:[minute]:[second]::[subsecond digits:4]"
                    .to_string(),
            json_config: Some(JsonConfig::new(None)),
            file_config: None,
        }
    }

    #[test]
    fn test_stdout_logger() {
        let levels = ["INFO", "DEBUG", "WARN", "ERROR", "TRACE"];

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
        let levels = ["INFO", "DEBUG", "WARN", "ERROR", "TRACE"];
        let metadata = LogMetadata {
            data: std::collections::HashMap::from([("Mercury".to_string(), "Mercury".to_string())]),
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

    #[test]
    fn test_file_logger_minute() {
        let config = generate_test_file_config(
            "INFO".to_string(),
            true,
            false,
            "minutely",
            "minute/log.log",
        );
        let logger = RustLogger::new(&config, None);
        logger.info("test", None);

        std::fs::remove_dir_all("minute").unwrap();
    }

    #[test]
    fn test_file_logger_hourly() {
        let config =
            generate_test_file_config("INFO".to_string(), true, false, "hourly", "hourly/log.log");
        let logger = RustLogger::new(&config, None);
        logger.info("test", None);

        std::fs::remove_dir_all("hourly").unwrap();
    }

    #[test]
    fn test_file_logger_daily() {
        let config =
            generate_test_file_config("INFO".to_string(), true, false, "daily", "daily/log.log");
        let logger = RustLogger::new(&config, None);
        logger.info("test", None);

        std::fs::remove_dir_all("daily").unwrap();
    }

    #[test]
    fn test_file_logger_never() {
        let config =
            generate_test_file_config("INFO".to_string(), true, false, "never", "never/log.log");
        let logger = RustLogger::new(&config, None);
        logger.info("test", None);

        std::fs::remove_dir_all("never").unwrap();
    }

    #[test]
    fn test_invalid_output() {
        let metadata = LogMetadata {
            data: std::collections::HashMap::from([("Mercury".to_string(), "Mercury".to_string())]),
        };

        let config = generate_test_incorrect_config();
        let logger = RustLogger::new(&config, None);
        logger.info("test", Some(&metadata));
    }
}
