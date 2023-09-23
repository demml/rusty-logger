import glob
from rusty_logger import Logger, LogConfig, LogMetadata, JsonConfig, LogLevel, LogFileConfig, __version__
import shutil
import subprocess


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
    completed_process = subprocess.run(["python", "tests/standard_logger.py"])
    assert completed_process.returncode == 0


def test_debug_logger_file(file_logger: Logger):
    file_logger.info("test info")
    file_logger.debug("test debug")
    file_logger.warning("test warning")
    file_logger.error("test error")

    assert glob.glob(f"log/test.log*")

    for name in glob.glob(f"log/test.log*"):
        with open(name, "r") as fp:
            for count, line in enumerate(fp):
                pass
            count = count + 1
    assert count == 4
    shutil.rmtree("log", ignore_errors=False)
