from typing import Optional
from ._rusty_logger import Logger
from ._rusty_logger import LogConfig, LogMetadata, JsonConfig, LogLevel, LogFileConfig


class RustyLogger:
    def __init__(self, config: Optional[LogConfig] = None):
        """Creates a new instance of RustyLogger.

        Args:
            config:
                The configuration for the logger.

        """
        self._logger: Logger = Logger(config=config)

    @property
    def name(self) -> str:
        """The configuration for the logger."""
        return self._logger.config.name

    @name.setter
    def name(self, name: str) -> str:
        """The configuration for the logger."""
        self._logger.update_name(name)

    @property
    def config(self) -> LogConfig:
        """The configuration for the logger."""
        return self._logger.config

    @config.setter
    def config(self, config: LogConfig) -> LogConfig:
        """The configuration for the logger."""

        # drop guard in case its set (silently fails if not set)
        self._logger.drop_guard()
        self._logger = Logger(config=config)

    def set_level(self, level: str) -> None:
        """Sets the log level of the logger.

        Args:
            level:
                The level to log at.
        """
        self._logger = self._logger.set_level(level)

    def json(self) -> LogConfig:
        """Turns on json logging"""
        self._logger.config.json_config = JsonConfig()

        # reload logger
        self._logger = Logger.get_logger(config=self._logger.config)

    def info(self, message: str, metadata: Optional[LogMetadata] = None, *args) -> None:
        """Logs a message at the INFO level.

        Args:
            message:
                The message to log.
            metadata:
                Optional metadata to associate with the log.
        """
        self._logger.info(
            message=message,
            metadata=metadata,
            *args,
        )

    def debug(self, message: str, metadata: Optional[LogMetadata] = None, *args) -> None:
        """Logs a message at the DEBUG level.

        Args:
            message:
                The message to log.
            metadata:
                Optional metadata to associate with the log.
        """
        self._logger.debug(
            message=message,
            metadata=metadata,
            *args,
        )

    def warning(self, message: str, metadata: Optional[LogMetadata] = None, *args) -> None:
        """Logs a message at the WARNING level.

        Args:
            message:
                The message to log.
            metadata:
                Optional metadata to associate with the log.
        """
        self._logger.warning(
            message=message,
            metadata=metadata,
            *args,
        )

    def error(self, message: str, metadata: Optional[LogMetadata] = None, *args) -> None:
        """Logs a message at the ERROR level.

        Args:
            message:
                The message to log.
            metadata:
                Optional metadata to associate with the log.
        """
        self._logger.error(
            message=message,
            metadata=metadata,
            *args,
        )

    def trace(self, message: str, metadata: Optional[LogMetadata] = None, *args) -> None:
        """Logs a message at the TRACE level.

        Args:
            message:
                The message to log.
            metadata:
                Optional metadata to associate with the log.
        """
        self._logger.trace(
            message=message,
            metadata=metadata,
            *args,
        )
