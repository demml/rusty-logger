from typing import Optional, Dict

class LogConfig:
    def __init__(
        self,
        stdout: bool = True,
        stderr: bool = False,
        level: str = "INFO",
        filename: Optional[str] = None,
        env: Optional[str] = None,
    ):
        """Creates logger configuration

        Args:
            stdout:
                Whether to log to stdout.
            stderr:
                Whether to log to stderr.
            level:
                The level to log at.
            filename:
                Optional name of log file to write to. Can be a path (logs/test.log)
                or just a name (test.log).
            env:
                The environment name to associate with logs. Defaults to "development"

        """
        ...

class LogMetadata:
    def __init__(self, info: Dict[str, str]):
        """Creates logger metadata

        Args:
            init:
                The metadata to associate with logs.
        """
        ...

class JsonLogger:
    @classmethod
    def get_logger(
        cls,
        name: str,
        config: Optional[LogConfig] = None,
    ) -> "JsonLogger":
        """Gets a logger with the given name. If output is None, the logger will log to stdout.

        Args:
            name:
                The name of the logger. Usually this is the name of the module that is using the logger.
            config:
                The configuration for the logger.

        Returns:
            A `JsonLogger` instance.
        """
        ...
    def info(self, message: str, metadata: Optional[LogMetadata] = None) -> None:
        """Logs a message at the INFO level.

        Args:
            message:
                The message to log.
        """
        ...
    def debug(self, message: str, metadata: Optional[LogMetadata] = None) -> None:
        """Logs a message at the DEBUG level.

        Args:
            message:
                The message to log.
        """
        ...
    def warning(self, message: str, metadata: Optional[LogMetadata] = None) -> None:
        """Logs a message at the WARNING level.

        Args:
            message:
                The message to log.
        """
        ...
    def error(self, message: str, metadata: Optional[LogMetadata] = None) -> None:
        """Logs a message at the ERROR level.

        Args:
            message:
                The message to log.
        """
        ...
