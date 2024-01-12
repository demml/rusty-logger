import glob
from rusty_logger import LogConfig, JsonConfig, LogLevel, LogFileConfig, __version__, Logger
import shutil

"""All tests are performed with guard locking
Guard locking is a feature that allows you to lock a logger to a specific context that is dropped on end of context.
All tests are performed with loggers scoped to their function context.
If lock_guard is set to False, a default global logger is used that runs the duration of the application
and is immutable after instantiation. Thus, subsequent tests with different logger configurations will fail.
However, guard_lock = false (default) is useful for applications that do not require multiple loggers.
"""


def test_version():
    assert __version__ is not None


def test_log_config():
    config = LogConfig()
    assert config.stdout is True
    assert config.stderr is False
    assert config.level == "INFO"
    assert config.app_env == "development"
    assert config.json_config is None


def test_log_level():
    assert LogLevel.DEBUG == "DEBUG"
    assert LogLevel.INFO == "INFO"
    assert LogLevel.WARN == "WARN"
    assert LogLevel.ERROR == "ERROR"
    assert LogLevel.TRACE == "TRACE"


def test_info_logger_stdout():
    logger = Logger.get_logger(
        name=__file__,
        config=LogConfig(lock_guard=True),
    )
    logger.info("test info")
    logger.debug("test debug")
    logger.warning("test warning")
    logger.error("test error")
    logger.trace("test trace")
    logger.info("test info", color="red")  # testing color parsing


def test_debug_logger_file():
    file_config = LogFileConfig(filename="log/test.log")
    logger = Logger.get_logger(
        name=__file__,
        config=LogConfig(
            lock_guard=True,
            file_config=file_config,
            level="DEBUG",
        ),
    )

    logger.info("test info")
    logger.debug("test debug")
    logger.warning("test warning")
    logger.error("test error")

    assert glob.glob("log/test.log*")

    for name in glob.glob("log/test.log*"):
        with open(name, "r") as fp:
            for count, line in enumerate(fp):
                pass
            count = count + 1
    assert count == 4
    shutil.rmtree("log", ignore_errors=False)


def test_warn_logger_file():
    file_config = LogFileConfig(filename="log/test.log")
    logger = Logger.get_logger(
        name=__file__,
        config=LogConfig(
            level="TRACE",
            json_config=JsonConfig(flatten=True),
            file_config=file_config,
            lock_guard=True,
        ),
    )

    logger.info("test info")
    logger.debug("test debug")
    logger.warning("test warning")
    logger.error("test error")
    logger.trace("test error")
    logger.info("test info", color="red")  # this will be skipped

    assert glob.glob("log/test.log*")

    for name in glob.glob("log/test.log*"):
        with open(name, "r") as fp:
            for count, line in enumerate(fp):
                pass
            count = count + 1
    assert count == 6
    shutil.rmtree("log", ignore_errors=False)


def test_error_logger_file():
    file_config = LogFileConfig(filename="log/test.log")
    logger = Logger.get_logger(
        name=__name__,
        config=LogConfig(
            level="ERROR",
            json_config=JsonConfig(flatten=True),
            file_config=file_config,
            lock_guard=True,
            thread_id=True,
        ),
    )

    logger.info("test info")
    logger.debug("test debug")
    logger.warning("test warning")
    logger.error("test error")

    assert glob.glob("log/test.log*")

    for name in glob.glob("log/test.log*"):
        with open(name, "r") as fp:
            for count, line in enumerate(fp):
                count
                pass
            count = count + 1
    assert count == 1
    shutil.rmtree("log", ignore_errors=False)


def _test_modules():
    from tests.mod_one import TestOne
    from tests.mod_two import TestTwo

    TestOne.test_logger()
    TestTwo.test_logger()

    for name in glob.glob("log/test.log*"):
        with open(name, "r") as fp:
            for count, line in enumerate(fp):
                pass
            count = count + 1
    assert count == 2
    shutil.rmtree("log", ignore_errors=False)


def test_invalid_config_format():
    logger = Logger.get_logger(
        name=__file__,
        config=LogConfig(
            stderr=False,
            stdout=False,
            time_format="[hour]:[minute]",
            lock_guard=True,
        ),
    )

    # Logger will default to stdout true if not set
    assert logger.config.stdout
    logger.info("blah")


def test_info_logger_stdout_args():
    # turn off guard locking for last test
    logger = Logger.get_logger(
        name=__file__,
        config=LogConfig(
            lock_guard=True,
        ),
    )

    logger.info("test info {} {} {} ", "test", 10.43, {"test": "test"})
