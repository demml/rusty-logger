from importlib.metadata import PackageNotFoundError, version

try:
    __version__ = version("rusty_logger")
except PackageNotFoundError:
    __version__ = "unknown"
