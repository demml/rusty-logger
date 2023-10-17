from rusty_logger import Logger, JsonConfig, LogConfig


class ApiLogger(Logger):
    @classmethod
    def get_logger(cls, name: str) -> Logger:
        return super().get_logger(
            name=name,
            config=LogConfig(
                stdout=True,
                time_format="[year]-[month]-[day]T[hour repr:24]:[minute]:[second]",
                json_config=JsonConfig(),
                thread_id=True,
            ),
        )
