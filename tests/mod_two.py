from typing import Optional
from rusty_logger import Logger, LogConfig, JsonConfig


class JsonLogger(Logger):
    @classmethod
    def get_logger(cls, name: str, config: Optional[LogConfig] = None) -> Logger:
        config = LogConfig(
            filename=f"log/test.log",
            json_config=JsonConfig(),
        )
        return super().get_logger(name, config)


class TestTwo:
    @staticmethod
    def test_logger():
        logger = JsonLogger.get_logger(__name__)
        logger.info("This is a test log")
