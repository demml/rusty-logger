from typing import Any, Optional

class JsonLogger:
    @classmethod
    def get_logger(cls, name: str, output: Optional[str] = None, level: Optional[str] = None) -> "JsonLogger":
        """Gets a logger with the given name. If output is None, the logger will log to stdout.

        Args:
            name:
                The name of the logger. Usually this is the name of the module that is using the logger.
            output:
                The path to the file to log to. If None, the logger will log to stdout.
            level:
                The level of the logger. If None, the logger will log at the INFO level.

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
