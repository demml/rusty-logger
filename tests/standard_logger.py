from rusty_logger import Logger, __version__


logger = Logger.get_logger(name=__file__)
logger.info("test info")
logger.debug("test debug")
logger.warning("test warning")
logger.error("test error")
logger.trace("test trace")
