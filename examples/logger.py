from rusty_logger import Logger, LogConfig

# Create a logger with default configuration
# Pushes logs to stdout at INFO level
logger = Logger.get_logger(name=__file__)
logger.info("test info")
