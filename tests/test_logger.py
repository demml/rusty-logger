from rusty_logger import JsonLogger


def test_logger():
    logger = JsonLogger.get_logger("test_logger")

    logger.info("test message")
    logger.debug("test message")
    logger.warning("test message")
    logger.error("test message")
    a
