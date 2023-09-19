from typing import Optional
from rusty_logger import Logger, LogConfig, JsonConfig, LogFileConfig


class JsonLogger(Logger):
    @classmethod
    def get_logger(cls, name: str, config: Optional[LogConfig] = None) -> Logger:
        file_config = LogFileConfig(filename="log/test.log")
        config = LogConfig(
            json_config=JsonConfig(),
            file_config=file_config,
        )
        return super().get_logger(name, config)


class TestTwo:
    @staticmethod
    def test_logger():
        logger = JsonLogger.get_logger(__name__)
        logger.info("This is a test log")
