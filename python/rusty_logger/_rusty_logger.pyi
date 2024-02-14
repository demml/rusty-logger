from typing import Optional

class LogLevel:
    """Enum for log levels"""

    @property
    def INFO(self) -> str:
        """The INFO log level"""
        ...
    @property
    def DEBUG(self) -> str:
        """The DEBUG log level"""
        ...
    @property
    def WARN(self) -> str:
        """The WARNING log level"""
        ...
    @property
    def ERROR(self) -> str:
        """The ERROR log level"""
        ...
    @property
    def TRACE(self) -> str:
        """The TRACE log level"""
        ...

class JsonConfig:
    def __init__(
        self,
        flatten: bool = True,
    ):
        """Creates logger json configuration

        Args:
            flatten:
                Whether to flatten the any fields that are passed
        """
        ...
    @property
    def flatten(self) -> bool:
        """Whether to flatten any fields that are passed"""
        ...

class LogFileConfig:
    def __init__(self, filename: str = "log/logs.log", rotate: str = "never"):
        """Creates logger file configuration for recording logs

        Args:
            filename:
                The name of the file to write logs to
            rotate:
                The rotation policy for the log file. Can be "never", "daily", "hourly", or "minutely"
        """
        ...

class LogConfig:
    def __init__(
        self,
        stdout: bool = True,
        stderr: bool = False,
        level: str = "INFO",
        name: Optional[str] = None,
        app_env: str = "development",
        time_format: str = "[year]-[month]-[day]T[hour repr:24]:[minute]:[second]::[subsecond digits:4]",
        lock_guard: bool = False,
        thread_id: bool = False,
        color: bool = False,
        json_config: Optional[JsonConfig] = None,
        file_config: Optional[LogFileConfig] = None,
    ):
        """Creates logger configuration

        Args:
            stdout:
                Whether to log to stdout
            stderr:
                Whether to log to stderr
            filename:
                Optional name of log file to write to. Can be a path (logs/test.log)
                or just a name (test.log)
            level:
                The level to log at
            app_env:
                The environment name to associate with logs. Defaults to "development"
            name:
                Name to record when logging events. Usually this is the name of the module that is using the logger
            time_format:
                The time format to use for logs
            lock_guard:
                Boolean indicating whether to lock this logger to current context. Usually this will be false
            thread_id:
                Whether to record the thread id in logs
            color:
                Whether to colorize logs
            json_config:
                Optional json logger configuration
            file_config:
                Optional file logger configuration
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
        """The level to log at"""
        ...
    @property
    def app_env(self) -> Optional[str]:
        """The environment name to associate with logs. Defaults to "development"""
        ...
    @property
    def name(self) -> bool:
        """Name to record when logging events. Usually this is the name of the module that is using the logger"""
        ...
    @property
    def time_format(self) -> Optional[str]:
        """The time format to use for logs"""
        ...
    @property
    def json_config(self) -> Optional[JsonConfig]:
        """Optional json logger configuration"""
        ...
    @property
    def lock_guard(self) -> bool:
        """Boolean indicating whether to lock this logger to current context"""
        ...
    @property
    def thread_id(self) -> bool:
        """Whether to record the thread id in logs"""
        ...
    @property
    def color(self) -> bool:
        """Whether to colorize logs"""
        ...

class Logger:
    @classmethod
    def get_logger(self, name: Optional[str] = None, config: Optional[LogConfig] = None) -> Logger:
        """Gets a new logger.

        Args:
            name:
                Name to record with logger. Usually this is the name of the module that is using the logger
            config:
                The configuration for the logger

        Returns:
            `RustyLogger`
        """
        ...
    def set_level(self, level: str) -> None:
        """Sets the log level of the logger.

        Args:
            level:
                The level to log at.
        """
        ...
    @property
    def config(self) -> LogConfig:
        """The configuration for the logger."""
        ...
    def info(self, message: str, *args, **kwargs) -> None:
        """Logs a message at the INFO level.

        Args:
            message:
                The message to log
            args:
                Args to format the message with
            kwargs:
                Kwargs to format the message with
        """
        ...
    def debug(self, message: str, *args) -> None:
        """Logs a message at the DEBUG level.

        Args:
            message:
                The message to log
            args:
                Args to format the message with
        """
        ...
    def warning(self, message: str, *args) -> None:
        """Logs a message at the WARNING level.

        Args:
            message:
                The message to log
            args:
                Args to format the message with
        """
        ...
    def error(self, message: str, *args) -> None:
        """Logs a message at the ERROR level.

        Args:
            message:
                The message to log
            args:
                Args to format the message with
        """
        ...
    def trace(self, message: str, *args) -> None:
        """Logs a message at the TRACE level.

        Args:
            message:
                The message to log
            args:
                Args to format the message with
        """
        ...
