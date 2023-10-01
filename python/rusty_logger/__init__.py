# pylint: disable=no-name-in-module


from ._rusty_logger import RustyLogger, LogConfig, JsonConfig, LogLevel, LogFileConfig
from .version import __version__


__all__ = ["logger", "LogConfig", "LogMetadata", "JsonConfig", "LogLevel", "LogFileConfig", "__version__"]

logger = RustyLogger(
    config=LogConfig(
        stdout=True,
        level=LogLevel.INFO,
    )
)
