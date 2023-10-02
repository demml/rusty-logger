import logging
import os
import sys
from datetime import datetime
from typing import IO
from pythonjsonlogger.jsonlogger import JsonFormatter
from rusty_logger import logger, LogLevel, LogConfig, JsonConfig


import timeit
import shutil
import pathlib
import sys

APP_ENV = os.getenv("APP_ENV", "development")
pathlib.Path.mkdir(pathlib.Path("logs"), exist_ok=True)


class LogFormatter(JsonFormatter):
    """Custom formatter"""

    def add_fields(self, log_record, record, message_dict):
        # Ensure level is first
        if log_record.get("level"):
            log_record["level"] = log_record["level"].upper()
        else:
            log_record["level"] = record.levelname
        super().add_fields(log_record, record, message_dict)
        if not log_record.get("timestamp"):
            log_record["timestamp"] = datetime.utcnow().strftime("%Y-%m-%dT%H:%M:%S")
        log_record["app_env"] = APP_ENV


class JsonLogger:
    @classmethod
    def get_handler(cls, stream: IO = sys.stdout) -> logging.StreamHandler:
        log_handler = logging.StreamHandler(stream)
        formatter = LogFormatter()
        log_handler.setFormatter(formatter)
        return log_handler

    @classmethod
    def get_logger(
        cls,
        name: str,
        stream: IO = sys.stdout,
    ):
        log = logging.getLogger(name)

        # Add a new stream handler if the log is new
        if len(log.handlers) == 0:
            log.addHandler(cls.get_handler(stream=stream))

        log_level: int = logging.getLevelName("INFO")
        log.setLevel(log_level)
        log.propagate = False

        return log


logger.config = LogConfig(
    level=LogLevel.INFO,
    stdout=True,
    json_config=JsonConfig(),
    time_format="[year]-[month]-[day]T[hour repr:24]:[minute]:[second]",
)
py_logger = JsonLogger.get_logger(name=__file__)


rust_result = timeit.timeit(stmt='logger.info("test info")', globals=globals(), number=1_000)
py_result = timeit.timeit(stmt='py_logger.info("test info")', globals=globals(), number=1_000)

print(f"Rust: {rust_result}")
print(f"Python: {py_result}")
print(f"Rust logger is {py_result / rust_result} times faster than Python default logger when logging to file")

shutil.rmtree("logs", ignore_errors=True)
