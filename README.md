
<p align="center">
  <a href="https://fastapi.tiangolo.com"><img src="https://github.com/thorrester/rusty-logger/blob/main/img/rusty-logger-logo.png?raw=true"  width="767" height="159" alt="rusty logger logo"/></a>
</p>


[![Lints-Tests](https://github.com/thorrester/rusty-logger/actions/workflows/lint-testing.yml/badge.svg?branch=main)](https://github.com/thorrester/rusty-logger/actions/workflows/lint-testing.yml)
[![codecov](https://codecov.io/gh/thorrester/rusty-logger/graph/badge.svg?token=RVDMQRUEHT)](https://codecov.io/gh/thorrester/rusty-logger)

## Rusty Logger

Simple, opinionated and blazingly fast python logging. `Rusty-Logger` is a thin python wrapper for `Rust's` `tracing` library that provides a *mostly* drop-in replacement for `pythons` default logging.


## Table of contents

- [Supported configuration](#supported-configuration)
- [Constraints](#constraints)
- [Additional metadata](#additional-metadata)
- [Code examples](#show-me-the-code)
- [Additional examples](#additional-examples)
- [Performance](#performance)
- [Contributing](#contributing)

## Supported Configuration

| Arg  | Description | Default |
| ------------- | :-------------:| :-------------: |
| `stdout`  | Log to stdout  | `True` |
| `stderr`  | Log to stderr  | `False` |
| `level`  | Level to log  | `INFO` |
| `app_env`  | Application environment (APP_ENV env var)  | `development` |
| `lock_guard`  | Whether to lock logger to current context  | `False` |
| `time_format` | Custom time format for logger | `[year]-[month]-[day]T[hour repr:24]:[minute]:[second]::[subsecond digits:4]` |
| `json_config`  | `JsonConig`  | `None` |
| `json_config.flatten`  | Whether to flatten any passed fields  | `True` |
| `file_config`  | `LogFileConfig`  | `None` |
| `file_config.filename`  | Filename for log  | `log/logs.log` |
| `file_config.rotate`  | File rotation specification. `daily`, `hourly`, `minutely` or `never`  | `never` |

## Constraints

Time is currently limited to UTC; however, you can customize time format to your liking using the `time_format` arg. Please refer to (time docs)[https://time-rs.github.io/book/api/format-description.html] for formatting guidelines. In addition, because `Rusty-Logger` calls `Rust` directly, it's not currently possible to pull the specific line number where logging takes place unless python is directly used (if you're even interested in this feature :smile:). If you'd like to see this feature implemented, and you want to contribute, please refer to the [contributing](https://github.com/thorrester/rusty-logger/blob/main/CONTRIBUTING.md) guide.

In addition, `Rusty-Logger` is a *mostly* drop-in replacement, meaning that in many workflows it'll work out of the box with no code change needed. However, in cases of `lazy` logging through additional `args``, `args` are expected to already be formatted as a `str` and will not be formatted by `Rusty-Logger` as the `rust` logic expects a `Vec<&str>` for args. For example, the following will not work:

```python
# This will not work
logger.info("Number: %s", 10)

# This will work
logger.info("Number: %s", str(10))
```

## Additional Metadata

You may also pass additional metadata along with any logging messages via the `LogMetadata` class, which takes a `Dict[str, str]` as an argument. 

## Show Me The Code!

### Basic Usage

```python
from rusty_logger import Logger

logger = Logger.get_logger(__file__)
logger.info("his palms are sweaty")
```

output
```shell
2023-09-15T20:16:31.985449Z  INFO his palms are sweaty app_env="development" name="your_file.py"
``` 

### JSON

```python
from rusty_logger import Logger, LogConfig, JsonConfig

logger = Logger.get_logger(__file__, LogConfig(json_config=JsonConfig()))
logger.info("knees weak")
```

output
```shell
{"timestamp":"2023-09-15T20:19:52.182299Z","level":"INFO","message":"knees weak","app_env":"development","name":"your_file.py"}
```

### Log to file

```python
from rusty_logger import Logger, LogConfig, JsonConfig, LogLevel, LogFileConfig

logger = Logger.get_logger(
    name=__file__,
    config=LogConfig(
        stdout=False,
        level=LogLevel.WARN,
        json_config=JsonConfig(),
        file_config=LogFileConfig(filename="logs/test.log"),
    ),
)
logger.warning("arms are heavy")
```

output from `log/test.log`
```shell
{"timestamp":"2023-09-15T20:23:37.461645Z","level":"WARN","message":"arms are heavy","app_env":"development","name":"your_file.py"}
```

### Adding some metadata

```python
from rusty_logger import Logger, LogConfig, JsonConfig, LogMetadata

logger = Logger.get_logger(__file__, LogConfig(json_config=JsonConfig()))
metadata = LogMetadata(data={"there's": "vomit"})
logger.info("on his sweater already", metadata=metadata)
```

output
```shell
{"timestamp":"2023-09-15T20:27:29.013887Z","level":"INFO","message":"on his sweater already","app_env":"development","name":"your_file.py","info":"{\"there's\": \"vomit\"}"}
```

### Record multiple places at once

```python
from rusty_logger import Logger, LogConfig, JsonConfig, LogMetadata, LogLevel, LogFileConfig

logger = Logger.get_logger(
    __file__,
    LogConfig(
        stdout=True,
        level=LogLevel.ERROR,
        json_config=JsonConfig(),
        file_config=LogFileConfig(filename="logs/test.log")
    ),
)
logger.error("MOM'S SPAGHETTI!")
```

output
```shell
{"timestamp":"2023-09-15T20:32:23.417027Z","level":"ERROR","message":"MOM'S SPAGHETTI!","app_env":"development","name":"your_file.py"}
```
## Additional examples

For additional examples, please see the [examples](https://github.com/thorrester/rusty-logger/tree/main/examples) directory which contains timed example of vanilla logger vs `Rusty-Logger`, `python-json-logger` vs `Rusty-Logger` as well as a multi-worker API example.

## Performance
Why would we do this when python logging is fine? Because we wanted something faster :smile:. From our own benchmarks, `Rusty-Logger` tends to be ~`4x` faster than vanilla python logging. And while speed may not be mission critical for a few thousands logs, it can be for millions, which many companies deal with on a daily basis. Time is money and compute, and we want to save you both :moneybag: :computer:.

## Contributing
While `Rusty-Logger` is production ready out of the box, it is still in it's infancy and is ripe for additional contributions. If you'd like to contribute, please see the [contributing](https://github.com/thorrester/rusty-logger/blob/main/CONTRIBUTING.md) guide.


Thank You!!! :heart: :heart: :heart: