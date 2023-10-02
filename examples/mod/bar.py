from custom_logger import logger


logger.name = __file__


def bar():
    logger.info("test info")
