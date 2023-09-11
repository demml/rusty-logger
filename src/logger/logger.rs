use std::env;
use std::io;
use std::path::Path;
use tracing_appender::non_blocking::WorkerGuard;
use tracing_subscriber::filter::LevelFilter;
use tracing_subscriber::fmt::format::JsonFields;
use tracing_subscriber::fmt::format::{Format, Json};
use tracing_subscriber::FmtSubscriber;
use tracing_subscriber::{prelude::*, registry::LookupSpan, Layer};

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

fn get_log_directory(output: &str) -> String {
    let file_splits = output.split("/").to_owned().collect::<Vec<&str>>().clone();
    let directory = file_splits[..file_splits.len() - 1].join("/");

    if directory.is_empty() {
        "./logs".to_string()
    } else {
        directory
    }
}

/// Initialize the tracing subscriber
///
/// # Arguments
///
/// * `output` - The output of the logger. Either "stdout" or "stderr"
/// * `level` - The level of the logger. Either "info", "debug", "warn", or "error"
///
fn build_layer(output: &str, level: &str) -> () {
    let layer = tracing_subscriber::fmt::layer()
        .json()
        .with_target(false)
        .with_current_span(false);

    if output == "stdout" {
        tracing_subscriber::registry()
            .with(
                layer
                    .with_writer(io::stdout)
                    .with_filter(match level.as_ref() {
                        "info" => LevelFilter::INFO,
                        "debug" => LevelFilter::DEBUG,
                        "warn" => tracing::Level::WARN,
                        "error" => tracing::Level::ERROR,
                        _ => tracing::Level::INFO,
                    })
                    .boxed(),
            )
            .try_init();
    } else if output == "stderr" {
        tracing_subscriber::registry()
            .with(layer.with_writer(io::stderr).boxed())
            .try_init();
    } else {
        let directory = get_log_directory(output);
        let file_name_prefix = get_file_name(Some(output));
        let file_appender = tracing_appender::rolling::hourly(directory, file_name_prefix);
        tracing_subscriber::registry()
            .with(layer.with_writer(file_appender).boxed())
            .try_init();
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
pub struct JsonLogger {
    pub output: String,
    pub level: String,
    pub env: String,
    pub name: String,
    subscriber: FmtSubscriber<JsonFields, Format<Json>>,
}

impl JsonLogger {
    /// Create a new logger
    ///
    /// # Arguments
    ///
    /// * `output` - The output of the logger. Either "stdout" or "stderr"
    /// * `level` - The level of the logger. Either "info", "debug", "warn", or "error"
    /// * `name` - The name of the file
    ///
    pub fn new(output: String, level: String, name: Option<String>) -> JsonLogger {
        // get name of file
        let file_name = get_file_name(name);

        // Set up tracing
        init_tracer(&output, &level);

        Self {
            output: output,
            level: level,
            env: match env::var("APP_ENV") {
                Ok(val) => val,
                Err(_e) => "development".to_string(),
            },
            name: file_name,
        }
    }

    /// Log an info message
    ///
    /// # Arguments
    ///
    /// * `message` - The message to log
    ///
    pub fn info(&self, message: &str) -> () {
        tracing::info!(message = message, app_env = self.env, name = self.name);
    }

    /// Log a debug message
    ///
    /// # Arguments
    ///
    /// * `message` - The message to log
    ///
    pub fn debug(&self, message: &str) -> () {
        tracing::debug!(message = message, app_env = self.env, name = self.name);
    }

    /// Log a warning message
    ///
    /// # Arguments
    ///
    /// * `message` - The message to log
    ///
    pub fn warning(&self, message: &str) -> () {
        tracing::warn!(message = message, app_env = self.env, name = self.name);
    }

    /// Log an error message
    ///
    /// # Arguments
    ///
    /// * `message` - The message to log
    ///
    pub fn error(&self, message: &str) -> () {
        tracing::error!(message = message, app_env = self.env, name = self.name);
    }
}

#[cfg(test)]
mod tests {
    use super::JsonLogger;

    #[test]
    fn test_stdout_logger() {
        let logger = JsonLogger::new("stdout".to_string(), "info".to_string(), None);
        logger.info("test");
        logger.debug("test");
        logger.warning("test");
        logger.error("test");
    }

    #[test]
    fn test_stderr_logger() {
        let logger = JsonLogger::new("stderr".to_string(), "info".to_string(), None);
        logger.info("test");
        logger.debug("test");
        logger.warning("test");
        logger.error("test");
    }
}
