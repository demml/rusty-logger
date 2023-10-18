from custom_logger import JsonLogger


logger = JsonLogger.get_logger(__file__)


def foo():
    logger.info("test info")
