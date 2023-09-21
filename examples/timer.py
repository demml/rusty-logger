from rusty_logger import Logger, LogLevel, LogConfig
import logging
import timeit
import shutil
import pathlib
import sys

pathlib.Path.mkdir(pathlib.Path("logs"), exist_ok=True)

# setup py logger
logging.basicConfig(
    # filename="logs/py_log.log",
    level=logging.INFO,
    format="%(asctime)s %(levelname)s %(message)s",
    datefmt="%Y-%m-%d %H:%M:%S",
    handlers=[logging.StreamHandler(sys.stdout)],
)

logger = Logger.get_logger(
    name=__file__,
    config=LogConfig(
        stdout=True,
        # level=LogLevel.DEBUG,
        # filename="logs/rust_log.log",
    ),
)


rust_result = timeit.timeit(stmt='logger.info("test info")', globals=globals(), number=100)
py_result = timeit.timeit(stmt='logging.info("test info")', globals=globals(), number=100)

print(f"Rust: {rust_result}")
print(f"Python: {py_result}")
print(f"Rust logger is {py_result / rust_result} times faster than Python default logger when logging to file")

shutil.rmtree("logs", ignore_errors=True)
