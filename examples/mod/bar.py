from examples.mod.subclass_logger import JsonLogger


logger = JsonLogger.get_logger(__file__)


def bar():
    logger.info("test info")
