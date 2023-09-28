import glob
from rusty_logger import Logger, LogConfig, LogMetadata, JsonConfig, LogLevel, LogFileConfig, __version__
import shutil
import json

"""All tests are performed with guard locking
Guard locking is a feature that allows you to lock a logger to a specific context that is dropped on end of context.
All tests are performed with loggers scoped to their function context.
If lock_guard is set to False, a default global logger is used that runs the duration of the application
and is immutable after instantiation. Thus, subsequent tests with different logger configurations will fail.
However, guard_lock = false (default) is useful for applications that do not require multiple loggers.
"""


class A:
    def __str__(self):
        return "A"


def test_info_logger_stdout():
    logger = Logger.get_logger(
        name=__file__,
        config=LogConfig(lock_guard=True),
    )
    logger.info("test info")
    logger.info(
        "test info {} {} {} {} {} {} {}", 10, "hello", 10.43, {"a": 1, "b": 2}, [1, 2, 3], A(), '{"a": 1, "b": 2}'
    )

    logger.debug("test debug")
    logger.warning("test warning")
    logger.error("test error")
    logger.trace("test trace")
