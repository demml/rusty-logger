from typing import Any, Optional, Bool

class LogConfig:
    def __init__(
        self,
        stdout: Bool = True,
        stderr: Bool = False,
        level: str = "INFO",
        filename: Optional[str] = None,
        env: Optional[str] = None,
    ): ...

    """Creates logger configuration
    
    Args:
        stdout:
            Whether to log to stdout.
        stderr:
            Whether to log to stderr.
        level:
            The level to log at.
        filename:
            The path to the file to log to.
        env:
            The environment variable to use to override the log level.
        
    """

class JsonLogger:
    @classmethod
    def get_logger(cls, name: str, config: Optional[LogConfig] = None) -> "JsonLogger":
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
    def info(self, message: str) -> None:
        """Logs a message at the INFO level.

        Args:
            message:
                The message to log.
        """
        ...
    def debug(self, message: str) -> None:
        """Logs a message at the DEBUG level.

        Args:
            message:
                The message to log.
        """
        ...
    def warning(self, message: str) -> None:
        """Logs a message at the WARNING level.

        Args:
            message:
                The message to log.
        """
        ...
    def error(self, message: str, **kwargs: Any) -> None:
        """Logs a message at the ERROR level.

        Args:
            message:
                The message to log.
        """
        ...
