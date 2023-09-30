import glob
from rusty_logger import LogConfig, LogMetadata, JsonConfig, LogLevel, LogFileConfig, __version__, logger
import shutil
import json

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
    assert config.target is False
    assert config.json_config is None


def test_log_level():
    assert LogLevel.DEBUG == "DEBUG"
    assert LogLevel.INFO == "INFO"
    assert LogLevel.WARN == "WARN"
    assert LogLevel.ERROR == "ERROR"
    assert LogLevel.TRACE == "TRACE"


def test_info_logger_stdout():
    logger.config = LogConfig(lock_guard=True)
    logger.name = __file__
    logger.info("test info")
    logger.debug("test debug")
    logger.warning("test warning")
    logger.error("test error")
    logger.trace("test trace")


def test_debug_logger_file():
    file_config = LogFileConfig(filename="log/test.log")
    logger.config = LogConfig(
        level="DEBUG",
        file_config=file_config,
        lock_guard=True,
    )
    logger.name = __file__

    logger.info("test info")
    logger.debug("test debug")
    logger.warning("test warning")
    logger.error("test error")

    assert glob.glob(f"log/test.log*")

    for name in glob.glob(f"log/test.log*"):
        with open(name, "r") as fp:
            for count, line in enumerate(fp):
                pass
            count = count + 1
    assert count == 4
    shutil.rmtree("log", ignore_errors=False)


def test_warn_logger_file():
    file_config = LogFileConfig(filename="log/test.log")
    logger.config = LogConfig(
        level="TRACE",
        json_config=JsonConfig(flatten=True),
        file_config=file_config,
        lock_guard=True,
    )
    logger.name = __name__
    logger.info("test info")
    logger.debug("test debug")
    logger.warning("test warning")
    logger.error("test error")
    logger.trace("test error")

    assert glob.glob(f"log/test.log*")

    for name in glob.glob(f"log/test.log*"):
        with open(name, "r") as fp:
            for count, line in enumerate(fp):
                pass
            count = count + 1
    assert count == 5
    shutil.rmtree("log", ignore_errors=False)


def test_error_logger_file():
    file_config = LogFileConfig(filename="log/test.log")
    logger.config = LogConfig(
        level="ERROR",
        file_config=file_config,
        lock_guard=True,
    )
    logger.name = __name__
    logger.info("test info")
    logger.debug("test debug")
    logger.warning("test warning")
    logger.error("test error")

    assert glob.glob(f"log/test.log*")

    for name in glob.glob(f"log/test.log*"):
        with open(name, "r") as fp:
            for count, line in enumerate(fp):
                count
                pass
            count = count + 1
    assert count == 1
    shutil.rmtree("log", ignore_errors=False)


def test_modules():
    from tests.mod_one import TestOne
    from tests.mod_two import TestTwo

    TestOne.test_logger()
    TestTwo.test_logger()

    for name in glob.glob(f"log/test.log*"):
        with open(name, "r") as fp:
            for count, line in enumerate(fp):
                pass
            count = count + 1
    assert count == 2
    shutil.rmtree("log", ignore_errors=False)


def test_metadata():
    file_config = LogFileConfig(filename="log/test.log")
    logger.config = LogConfig(
        level="INFO",
        json_config=JsonConfig(),
        file_config=file_config,
        lock_guard=True,
    )

    logger.info("test info", metadata=LogMetadata(data={"test": "info"}))

    for name in glob.glob(f"log/test.log*"):
        with open(name, "r") as fp:
            json_list = list(fp)

        for json_str in json_list:
            result = json.loads(json_str)
            result = json.loads(result["metadata"])
            assert result.get("name") is None

        assert "test" in result
        shutil.rmtree("log", ignore_errors=False)


def test_invalid_config_format():
    logger.config = LogConfig(
        stderr=False,
        stdout=False,
        time_format="[hour]:[minute]",
        lock_guard=True,
    )
    logger.name = __file__

    # Logger will default to stdout true if not set
    assert logger.config.stdout == True
    logger.info("blah")


def test_info_logger_stdout_args():
    # turn of guard locking for last test
    logger.config = LogConfig(lock_guard=True)
    logger.info("test info {} {} {} ", "test", 10.43, {"test": "test"})
