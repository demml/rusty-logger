
<br>
<img src="img/rusty-logger-logo.png"  width="767" height="159" alt="rusty logger logo"/>
<br>

[![CI](https://github.com/thorrester/rusty-logger/actions/workflows/CI.yml/badge.svg)](https://github.com/thorrester/rusty-logger/actions/workflows/CI.yml)
[![codecov](https://codecov.io/gh/thorrester/rusty-logger/graph/badge.svg?token=RVDMQRUEHT)](https://codecov.io/gh/thorrester/rusty-logger)

## Rusty Logger

Simple, opinionated and blazingly fast python logging. `Rusty-Logger` is a thin python wrapper for `Rust's` `tracing` library that provides minimal features for those that just want to log without crazy configurations.


## Table of contents

- [Supported configuration](#supported-configuration)
- [Constraints](#constraints)
- [Additional metadata](#additional-metadata)
- [Code examples](#show-me-the-code)
- [Performance](#performance)
- [Contributing](#contributing)

## Supported Configuration

| Arg  | Description | Default |
| ------------- | :-------------:| :-------------: |
| `stdout`  | Log to stdout  | `True` |
| `stderr`  | Log to stderr  | `False` |
| `filename`  | Log to file  | `None` |
| `level`  | Level to log  | `INFO` |
| `app_env`  | Application environment (APP_ENV env var)  | `development` |
| `time_format` | Custom time format for logger | `[year]-[month]-[day]T[hour repr:24]:[minute]:[second]::[subsecond digits:4]` |
| `json_config`  | `JsonCofig`  | `None` |
| `json_config.flatten`  | Whether to flatten any passed fields  | `True` |

## Constraints
Time is currently limited to UTC; however, you can customize time format to your liking using the `time_format` arg. Please refer to (time docs)[https://time-rs.github.io/book/api/format-description.html] for formatting guidelines. In addition, because `Rusty-Logger` calls `Rust` directly, it's not currently possible to pull the specific line number where logging takes place unless python is directly used (if you're even interested in this feature :smile:). If you'd like to see this feature implemented, and you want to contribute, please refer to the [contributing](https://github.com/thorrester/rusty-logger/blob/main/CONTRIBUTING.md) guide.

## Additional Metadata

You may also pass additional metadata along with any logging messages via the `LogMetadata` class, which takes a `Dict[str, str]` as an argument. 


## Show Me The Code!

### Basic Usage

```python
from rusty_logger import Logger

# defaults to stdout and INFO level
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

# defaults to stdout and INFO level
logger = Logger.get_logger(__file__, LogConfig(json_config=JsonConfig()))
logger.info("knees weak")
```

output
```shell
{"timestamp":"2023-09-15T20:19:52.182299Z","level":"INFO","message":"knees weak","app_env":"development","name":"your_file.py"}
```

### Log to file

```python
from rusty_logger import Logger, LogConfig, JsonConfig, LogLevel

logger = Logger.get_logger(
    name=__file__,
    config=LogConfig(
        stdout=False,
        level=LogLevel.WARN,
        filename="log/test.log",
        json_config=JsonConfig(),
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
metadata = LogMetadata(info={"there's": "vomit"})
logger.info("on his sweater already", metadata=metadata)
```

output
```shell
{"timestamp":"2023-09-15T20:27:29.013887Z","level":"INFO","message":"on his sweater already","app_env":"development","name":"your_file.py","info":"{\"there's\": \"vomit\"}"}
```

### Record multiple places at once

```python
from rusty_logger import Logger, LogConfig, JsonConfig, LogMetadata, LogLevel

logger = Logger.get_logger(
    __file__,
    LogConfig(
        stdout=True,
        level=LogLevel.ERROR,
        filename="logs/test.log",
        json_config=JsonConfig(),
    ),
)
logger.error("MOM'S SPAGHETTI!")
```

output
```shell
{"timestamp":"2023-09-15T20:32:23.417027Z","level":"ERROR","message":"MOM'S SPAGHETTI!","app_env":"development","name":"your_file.py"}
```

## Performance
Why would we do this when python logging is fine? Because we wanted something faster :smile:. From our own benchmarks, `Rusty-Logger` tends to be `4.5x` faster than vanilla python logging. And while speed may not be mission critical for a few thousands logs, it can be for millions, which many companies deal with on a daily basis. Time is money and compute, and we want to save you both :moneybag: :computer:.

## Contributing
While `Rusty-Logger` is production ready out of the box, it is still in it's infancy and is ripe for additional contributions. If you'd like to contribute, please see the [contributing](https://github.com/thorrester/rusty-logger/blob/main/CONTRIBUTING.md) guide.


Thank You!!! :heart: :heart: :heart: