from typing import Optional
from rusty_logger import Logger, LogConfig, JsonConfig


# Create custom json logger
class JsonLogger(Logger):
    @classmethod
    def get_logger(cls, name: str, config: Optional[LogConfig] = None) -> Logger:
        return super().get_logger(
            name=name,
            config=LogConfig(json_config=JsonConfig()),
        )
