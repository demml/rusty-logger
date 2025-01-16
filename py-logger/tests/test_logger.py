from rusty_logger import RustyLogger, LoggingConfig  # type: ignore


def test_config():
    LoggingConfig()


def test_logger():
    logger = RustyLogger.get_logger()
    logger.debug("Debug message")
    logger.info("Info message")
    logger.warn("Warning message")
    logger.error("Error message")
    logger.trace("Trace message")


def test_json_logger():
    config = LoggingConfig(use_json=True)
    logger = RustyLogger.get_logger(config)
    logger.debug("Debug message")
    logger.info("Info message")
    logger.warn("Warning message")
    logger.error("Error message")
    logger.trace("Trace message")
