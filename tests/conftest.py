import pytest
from rusty_logger import Logger, LogConfig, LogFileConfig
from importlib import reload
import sys


@pytest.fixture(scope="function")
def file_logger():
    file_config = LogFileConfig(filename="log/test.log")
    logger = Logger.get_logger(
        name=__file__,
        config=LogConfig(level="DEBUG", file_config=file_config),
    )

    yield logger
