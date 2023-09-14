from typing import Optional, Dict

class JsonConfig:
    def __init__(self, span: bool, flatten: bool):
        """Creates logger json configuration

        Args:
            span:
                Whether to log span information.
            flatten:
                Whether to flatten the any fields that are passed.
        """
        ...

class LogConfig:
    def __init__(
        self,
        stdout: bool = True,
        stderr: bool = False,
        filename: Optional[str] = None,
        level: str = "INFO",
        env: Optional[str] = None,
        target: bool = False,
        line_number: bool = False,
        time_format: Optional[str] = None,
        json_config: Optional[JsonConfig] = None,
    ):
        """Creates logger configuration

        Args:
            stdout:
                Whether to log to stdout.
            stderr:
                Whether to log to stderr.
            filename:
                Optional name of log file to write to. Can be a path (logs/test.log)
                or just a name (test.log).
            level:
                The level to log at.
            env:
                The environment name to associate with logs. Defaults to "development"
            target:
                Whether to log target information.
            line_number:
                Whether to log line number information.
            time_format:
                The time format to use for logs. Defaults to "[year]-[month]-[day]T[hour]:[minute]:[second]".
                For more information on time formats, see https://time-rs.github.io/book/api/well-known-format-descriptions.html
            json_config:
                Optional json logger configuration.
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

class Logger:
    @classmethod
    def get_logger(
        cls,
        name: str,
        config: Optional[LogConfig] = None,
    ) -> "Logger":
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
