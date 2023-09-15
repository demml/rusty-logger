from rusty_logger import Logger, LogLevel, LogConfig, JsonConfig

# This logger will log to stdout at INFO level in json format with line number
logger = Logger.get_logger(
    name=__file__,
    config=LogConfig(
        level=LogLevel.INFO,
        json_config=JsonConfig(flatten=True),
    ),
)

# log message
logger.info("test info")
