from rusty_logger import logger, LogConfig, LogFileConfig

file_config = LogFileConfig(filename="log/test.log")
logger.config = LogConfig(
    level="DEBUG",
    file_config=file_config,
)
logger.name = __file__

logger.info("test info")
logger.debug("test debug")
logger.warning("test warning")
logger.error("test error")
