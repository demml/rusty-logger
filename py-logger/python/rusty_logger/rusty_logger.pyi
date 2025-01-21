from typing import Optional

class LogLevel:
    Debug: "LogLevel"
    Info: "LogLevel"
    Warn: "LogLevel"
    Error: "LogLevel"
    Trace: "LogLevel"

class WriteLevel:
    Stdout: "WriteLevel"
    Stderr: "WriteLevel"

class LoggingConfig:
    show_threads: bool
    log_level: LogLevel
    write_level: WriteLevel
    use_json: bool

    def __init__(
        self,
        show_threads: Optional[bool] = False,
        log_level: Optional[LogLevel] = LogLevel.Info,
        write_level: Optional[WriteLevel] = WriteLevel.Stdout,
        use_json: Optional[bool] = False,
    ) -> None:
        """Initializes the LoggingConfig.

        Args:
            show_threads (Optional[bool]):
                Whether to show thread information in the logs. Defaults to False.
            log_level (Optional[LogLevel]):
                The log level to use. Defaults to LogLevel.Info.
            write_level (Optional[WriteLevel]):
                The write level to use. Defaults to WriteLevel.Stdout.
            use_json (Optional[bool]):
                Whether to use JSON format for logs. Defaults to False.
        """

    @staticmethod
    def json_default() -> "LoggingConfig":
        """Gets a default JSON configuration.

        show_threads: True
        log_level: Env or LogLevel.Info
        write_level: WriteLevel.Stdout
        use_json: True

        Returns:
            LoggingConfig:
                The default JSON configuration.
        """

    @staticmethod
    def default() -> "LoggingConfig":
        """Gets a default configuration.

        show_threads: True
        log_level: Env or LogLevel.Info
        write_level: WriteLevel.Stdout
        use_json: False

        Returns:
            LoggingConfig:
                The default JSON configuration.
        """

class RustyLogger:
    """The Rusty Logger class to use with your python and rust-backed projects."""

    @staticmethod
    def setup_logging(config: Optional[LoggingConfig] = None) -> None:
        """Sets up the logger with the given configuration.

        Args:
            config (LoggingConfig):
                The configuration to use for the logger.
        """

    @staticmethod
    def get_logger(config: Optional[LoggingConfig] = None) -> "RustyLogger":
        """Gets the logger instance.

        Args:
            config (LoggingConfig):
                The configuration to use for the logger.

        Returns:
            RustyLogger:
                The logger instance.
        """

    def debug(self, message: str, *args) -> None:
        """Logs a debug message.

        Args:
            message (str):
                The message to log.

            *args:
                Additional arguments to log.
        """

    def info(self, message: str, *args) -> None:
        """Logs an info message.

        Args:
            message (str):
                The message to log.

            *args:
                Additional arguments to log.
        """

    def warn(self, message: str, *args) -> None:
        """Logs a warning message.

        Args:
            message (str):
                The message to log.

            *args:
                Additional arguments to log.
        """

    def error(self, message: str, *args) -> None:
        """Logs an error message.

        Args:
            message (str):
                The message to log.

            *args:
                Additional arguments to log.
        """

    def trace(self, message: str, *args) -> None:
        """Logs a trace message.

        Args:
            message (str):
                The message to log.

            *args:
                Additional arguments to log.
        """
