from typing import Optional
from rusty_logger import Logger, LogConfig, JsonConfig


class JsonLogger(Logger):
    @classmethod
    def get_logger(cls, name: str, config: LogConfig | None = None) -> Logger:
        config = LogConfig(
            filename=f"log/test.log",
            json_config=JsonConfig(),
        )
        return super().get_logger(name, config)


class TestOne:
    @staticmethod
    def test_logger():
        logger = JsonLogger.get_logger(__name__)
        pointer = logger
        assert pointer == logger
        logger.info("This is a test log")
