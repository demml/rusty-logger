from rusty_logger import Logger, LogLevel, LogConfig

# Create a logger with default configuration
# Pushes logs to stdout at INFO level
logger = Logger.get_logger(name=__file__, config=LogConfig(level=LogLevel.INFO))
logger.info("test info")

# Set to ERROR level
logger.set_level(level=LogLevel.ERROR)

# This should log
logger.error("test error")

# This should not log
logger.info("test info")
