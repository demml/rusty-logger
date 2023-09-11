use std::env;
use std::io;

pub struct JsonLogger {
    pub output: String,
    pub level: String,
    pub env: String,
}

impl JsonLogger {
    pub fn new(output: String, level: String) -> JsonLogger {
        let tracer = tracing_subscriber::fmt()
            .json()
            .with_target(false)
            .with_max_level(match level.as_ref() {
                "info" => tracing::Level::INFO,
                "debug" => tracing::Level::DEBUG,
                "warn" => tracing::Level::WARN,
                "error" => tracing::Level::ERROR,
                _ => tracing::Level::INFO,
            })
            .with_current_span(false);

        if output == "stdout" {
            tracer.with_writer(io::stdout).try_init().unwrap_or(());
        } else {
            tracer.with_writer(io::stderr).try_init().unwrap_or(());
        }

        Self {
            output: output,
            level: level,
            env: match env::var("APP_ENV") {
                Ok(val) => val,
                Err(_e) => "development".to_string(),
            },
        }
    }

    pub fn info(&self, message: &str) -> () {
        tracing::info!(message = message, app_env = self.env);
    }

    pub fn debug(&self, message: &str) -> () {
        tracing::debug!(message = message, app_env = self.env);
    }

    pub fn warn(&self, message: &str) -> () {
        tracing::warn!(message = message, app_env = self.env);
    }

    pub fn error(&self, message: &str) -> () {
        tracing::error!(message = message, app_env = self.env);
    }
}

#[cfg(test)]
mod tests {
    use super::JsonLogger;

    #[test]
    fn test_stdout_logger() {
        let logger = JsonLogger::new("stdout".to_string(), "info".to_string());
        logger.info("test");
        logger.debug("test");
        logger.warn("test");
        logger.error("test");
    }

    #[test]
    fn test_stderr_logger() {
        let logger = JsonLogger::new("stderr".to_string(), "info".to_string());
        logger.info("test");
        logger.debug("test");
        logger.warn("test");
        logger.error("test");
    }
}
