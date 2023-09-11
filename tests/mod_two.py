from rusty_logger import JsonLogger, LogConfig

logger = JsonLogger.get_logger(__name__, config=LogConfig(filename="log/test.log"))


class TestTwo:
    @staticmethod
    def test_logger():
        logger.info("This is a test log")
