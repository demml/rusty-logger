use std::io;

pub struct JsonLogger {
    pub output: String,
    pub level: String,
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
            tracer.with_writer(io::stdout).init();
        } else {
            tracer.with_writer(io::stderr).init();
        }

        Self {
            output: output,
            level: level,
        }
    }

    pub fn info(&self, message: &str) -> () {
        tracing::info!("{}", message);
    }

    pub fn debug(&self, message: &str) -> () {
        tracing::debug!("{}", message);
    }

    pub fn warn(&self, message: &str) -> () {
        tracing::warn!("{}", message);
    }

    pub fn error(&self, message: &str) -> () {
        tracing::error!("{}", message);
    }
}

#[cfg(test)]
mod tests {

    #[test]
    fn test_stdout_logger() {
        use super::JsonLogger;
        let logger = JsonLogger::new("stdout".to_string(), "info".to_string());
        logger.info("test");
        logger.debug("test");
        logger.warn("test");
        logger.error("test");
    }

    #[test]
    fn test_stderr_logger() {
        use super::JsonLogger;
        let logger = JsonLogger::new("stderr".to_string(), "info".to_string());
        logger.info("test");
        logger.debug("test");
        logger.warn("test");
        logger.error("test");
    }
}
