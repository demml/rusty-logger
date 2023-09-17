from importlib.metadata import version, PackageNotFoundError


try:
    __version__ = version("rusty_logger")
except PackageNotFoundError:
    __version__ = "unknown"
