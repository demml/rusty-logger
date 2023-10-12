import glob
from rusty_logger import LogConfig, JsonConfig, LogLevel, LogFileConfig, __version__, logger
import shutil


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
