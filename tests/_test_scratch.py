from rusty_logger import PyLogger, LogConfig


logger = PyLogger.get_logger(
    name=__file__,
    config=LogConfig(lock_guard=True),
)

logger.info("test info")
