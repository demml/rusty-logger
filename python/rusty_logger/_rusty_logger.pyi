from typing import Optional, Dict

class LogLevel:
    """Enum for log levels."""

    @property
    def INFO(self) -> str:
        """The INFO log level."""
        ...
    @property
    def DEBUG(self) -> str:
        """The DEBUG log level."""
        ...
    @property
    def WARN(self) -> str:
        """The WARNING log level."""
        ...
    @property
    def ERROR(self) -> str:
        """The ERROR log level."""
        ...
    @property
    def TRACE(self) -> str:
        """The TRACE log level."""
        ...

class JsonConfig:
    def __init__(
        self,
        flatten: bool = True,
    ):
        """Creates logger json configuration

        Args:
            flatten:
                Whether to flatten the any fields that are passed.
        """
        ...
    @property
    def flatten(self) -> bool:
        """Whether to flatten any fields that are passed."""
        ...

class LogFileConfig:
    def __init__(self, filename: str = "log/logs.log", rotate: str = "never"):
        """Creates logger file configuration for recording logs

        Args:
            filename:
                The name of the file to write logs to.
            rotate:
                The rotation policy for the log file. Can be "never", "daily", "hourly", or "minutely".
        """
        ...

class LogConfig:
    def __init__(
        self,
        stdout: bool = True,
        stderr: bool = False,
        level: str = "INFO",
        app_env: Optional[str] = "development",
        time_format: Optional[str] = "[year]-[month]-[day]T[hour repr:24]:[minute]:[second]::[subsecond digits:4]",
        json_config: Optional[JsonConfig] = None,
        file_config: Optional[LogFileConfig] = None,
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
            app_env:
                The environment name to associate with logs. Defaults to "development"
            time_format:
                The time format to use for logs.
            json_config:
                Optional json logger configuration.
            file_config:
                Optional file logger configuration.
        """
        ...
    @property
    def stdout(self) -> bool:
        """Whether to log to stdout"""
        ...
    @property
    def stderr(self) -> bool:
        """Whether to log to stderr"""
        ...
    @property
    def filename(self) -> Optional[str]:
        """Optional name of log file to write to. Can be a path (logs/test.log)
        or just a name (test.log)"""
        ...
    @property
    def level(self) -> str:
        """The level to log at."""
        ...
    @property
    def app_env(self) -> Optional[str]:
        """The environment name to associate with logs. Defaults to "development"""
        ...
    @property
    def time_format(self) -> Optional[str]:
        """The time format to use for logs"""
        ...
    @property
    def json_config(self) -> Optional[JsonConfig]:
        """Optional json logger configuration"""
        ...

class LogMetadata:
    def __init__(self, data: Dict[str, str]):
        """Creates logger metadata

        Args:
            data:
                The metadata to associate with logs.
        """
        ...
    @property
    def data(self) -> Dict[str, str]:
        """The metadata to associate with logs."""
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
    @property
    def config(self) -> LogConfig:
        """The configuration for the logger."""
        ...
    def set_level(self, level: str) -> None:
        """Sets the log level of the logger.

        Args:
            level:
                The level to log at.
        """
        ...
    def info(self, message: str, metadata: Optional[LogMetadata] = None) -> None:
        """Logs a message at the INFO level.

        Args:
            message:
                The message to log.
            metadata:
                Optional metadata to associate with the log.
        """
        ...
    def debug(self, message: str, metadata: Optional[LogMetadata] = None) -> None:
        """Logs a message at the DEBUG level.

        Args:
            message:
                The message to log.
            metadata:
                Optional metadata to associate with the log.
        """
        ...
    def warning(self, message: str, metadata: Optional[LogMetadata] = None) -> None:
        """Logs a message at the WARNING level.

        Args:
            message:
                The message to log.
            metadata:
                Optional metadata to associate with the log.
        """
        ...
    def error(self, message: str, metadata: Optional[LogMetadata] = None) -> None:
        """Logs a message at the ERROR level.

        Args:
            message:
                The message to log.
            metadata:
                Optional metadata to associate with the log.
        """
        ...
    def trace(self, message: str, metadata: Optional[LogMetadata] = None) -> None:
        """Logs a message at the TRACE level.

        Args:
            message:
                The message to log.
            metadata:
                Optional metadata to associate with the log.
        """
        ...
