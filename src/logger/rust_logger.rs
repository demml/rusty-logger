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
#[allow(clippy::too_many_arguments)]
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
    pub app_env: String,

    #[pyo3(get, set)]
    pub target: bool,

    #[pyo3(get, set)]
    pub time_format: String,

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
        app_env: Option<String>,
        target: Option<bool>,
        time_format: Option<String>,
        json_config: Option<JsonConfig>,
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
        let filename_null = filename.is_some();

        println!("filename: {:?}", !filename_null);
        println!("stdout: {:?}", !stdout);
        println!("stderr: {:?}", !stderr);

        let stdout = if !stdout && !stderr && !filename_null {
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
            filename,
            level: level.unwrap_or_else(|| "INFO".to_string()),
            app_env: log_env,
            target: target.unwrap_or(false),
            time_format: time_format.unwrap_or_else(|| DEFAULT_TIME_PATTERN.to_string()),
            json_config,
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
    pub info: HashMap<String, String>,
}

#[pymethods]
impl LogMetadata {
    #[new]
    pub fn new(info: HashMap<String, String>) -> LogMetadata {
        LogMetadata { info }
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

        let file_name = get_file_name(name);

        Self {
            env: match env::var("APP_ENV") {
                Ok(val) => val,
                Err(_e) => "development".to_string(),
            },
            name: file_name,
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

        if log_config.filename.is_some() {
            let (directory, file_name_prefix) = get_file_params(log_config);
            let file_appender = tracing_appender::rolling::hourly(directory, file_name_prefix);
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

        if log_config.filename.is_some() {
            let (directory, file_name_prefix) = get_file_params(log_config);
            let writer = tracing_appender::rolling::hourly(directory, file_name_prefix);
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
            app_env: "development".to_string(),
            target: false,
            time_format:
                "[year]-[month]-[day] [hour repr:24]:[minute]:[second]::[subsecond digits:4]"
                    .to_string(),
            json_config: Some(JsonConfig::new(None)),
        }
    }

    fn generate_test_incorrect_config(level: String, stdout: bool, stderr: bool) -> LogConfig {
        LogConfig {
            stdout,
            stderr,
            filename: None,
            level,
            app_env: "development".to_string(),
            target: false,
            time_format: "blah-blah-blah".to_string(),
            json_config: Some(JsonConfig::new(None)),
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

    #[test]
    fn test_incorrect_date_format() {
        let metadata = LogMetadata {
            info: std::collections::HashMap::from([("Mercury".to_string(), "Mercury".to_string())]),
        };

        let config = generate_test_incorrect_config("INFO".to_string(), false, true);
        let logger = RustLogger::new(&config, None);
        logger.info("test", Some(&metadata));
    }
}
