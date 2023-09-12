import glob
from rusty_logger import JsonLogger, LogConfig, LogMetadata
import shutil
import json


def test_info_logger_stdout():
    logger = JsonLogger.get_logger(name=__file__)
    logger.info("test info")
    logger.debug("test debug")
    logger.warning("test warning")
    logger.error("test error")


def test_debug_logger_file():
    logger = JsonLogger.get_logger(
        name=__file__,
        config=LogConfig(
            filename=f"log/test.log",
            level="DEBUG",
        ),
    )
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
    logger = JsonLogger.get_logger(
        name=__file__,
        config=LogConfig(
            filename=f"log/test.log",
            level="WARN",
        ),
    )
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
    assert count == 2
    shutil.rmtree("log", ignore_errors=False)


def test_error_logger_file():
    logger = JsonLogger.get_logger(
        name=__file__,
        config=LogConfig(
            filename=f"log/test.log",
            level="ERROR",
        ),
    )
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
    logger = JsonLogger.get_logger(
        name=__file__,
        config=LogConfig(
            filename=f"log/test.log",
            level="INFO",
        ),
    )

    logger.info("test info", metadata=LogMetadata(info={"test": "info"}))

    for name in glob.glob(f"log/test.log*"):
        with open(name, "r") as fp:
            json_list = list(fp)

        for json_str in json_list:
            result = json.loads(json_str)
            result = json.loads(result["info"])

        assert "test" in result
        shutil.rmtree("log", ignore_errors=False)