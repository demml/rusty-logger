# pylint: disable=no-name-in-module


from ._rusty_logger import LogConfig, JsonConfig, LogLevel, LogFileConfig
from .logger import RustyLogger
from .version import __version__


logger = RustyLogger(
    config=LogConfig(
        stdout=True,
        level=LogLevel.INFO,
    )
)

__all__ = ["logger", "LogConfig", "LogMetadata", "JsonConfig", "LogLevel", "LogFileConfig", "__version__"]
