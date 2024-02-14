from rusty_logger import Logger, LogLevel, LogConfig

# Create a logger with default configuration
# Pushes logs to stdout at INFO level
logger = Logger.get_logger(
    name=__file__,
    config=LogConfig(
        level=LogLevel.INFO,
        thread_id=True,
        color=True,
    ),
)
logger.info("test info")
