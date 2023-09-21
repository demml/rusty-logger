from rusty_logger import Logger, LogLevel, LogConfig, JsonConfig, LogMetadata, LogFileConfig

# This logger will log to stdout at INFO level in json format
logger = Logger.get_logger(
    name=__file__,
    config=LogConfig(
        level=LogLevel.INFO,
        json_config=JsonConfig(flatten=True),
        file_config=LogFileConfig(filename="logs/test.log"),
    ),
)

metadata = LogMetadata(data={"key": "value"})
# log message
logger.info("test info", metadata=metadata)
