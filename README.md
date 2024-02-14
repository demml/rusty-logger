
<p align="center">
<img src="https://github.com/thorrester/rusty-logger/blob/main/img/rusty-logger-logo.png?raw=true"  width="767" height="159" alt="rusty logger logo"/></a>
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
| `thread_id`  | Whether to display the thread id  | `False` |
| `time_format` | Custom time format for logger | `[year]-[month]-[day]T[hour repr:24]:[minute]:[second]::[subsecond digits:4]` |
| `json_config`  | `JsonConig`  | `None` |
| `json_config.flatten`  | Whether to flatten any passed fields  | `True` |
| `file_config`  | `LogFileConfig`  | `None` |
| `file_config.filename`  | Filename for log  | `log/logs.log` |
| `file_config.rotate`  | File rotation specification. `daily`, `hourly`, `minutely` or `never`  | `never` |

## Constraints

Time is currently limited to UTC; however, you can customize time format to your liking using the `time_format` arg. Please refer to (time docs)[https://time-rs.github.io/book/api/format-description.html] for formatting guidelines. In addition, because `Rusty-Logger` calls `Rust` directly, it's not currently possible to pull the specific line number where logging takes place unless python is directly used (if you're even interested in this feature :smile:). If you'd like to see this feature implemented, and you want to contribute, please refer to the [contributing](https://github.com/thorrester/rusty-logger/blob/main/CONTRIBUTING.md) guide.

In addition, `Rusty-Logger` is a *mostly* drop-in replacement, meaning that you may need to make some minor changes to your existing code. For example, `Rusty-Logger` does not support current python lazy formatting (e.g. `logger.info("Number: %s", 10)`). Instead, `Rusty-Logger` uses Rust's default bracket ({}) formatting.

```python
# This is not supported
logger.info("Number: %s", 10)

# This is supported
logger.info("Number: {}", 10)
```

## Show Me The Code!

### Basic Usage

```python
from rusty_logger import Logger

logger = Logger.get_logger(__file__)
logger.info("Loggy McLogface")
```

output
```shell
2023-10-18T00:11:43::3194  INFO Loggy McLogface app_env="development" name="your_file.py"
``` 

### JSON

```python
from rusty_logger import Logger, LogConfig, JsonConfig

logger = Logger.get_logger(__file__, LogConfig(json_config=JsonConfig()))
logger.info("Loggy McLogface logs")
```

output
```shell
{"timestamp":"2023-10-18T00:10:59::9732","level":"INFO","message":"Loggy McLogface logs","app_env":"development","name":"your_file.py"}
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
logger.warning("Loggy McLogface logs logs")
```

output from `log/test.log`
```shell
{"timestamp":"2023-10-18T00:10:10::9364","level":"WARN","message":"Loggy McLogface logs logs","app_env":"development","name":"your_file.py"}

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
logger.error("Loggy McLogface logs logs that are logs")
```

output
```shell
{"timestamp":"2023-10-18T00:09:32::4053","level":"ERROR","message":"Loggy McLogface logs logs that are logs","app_env":"development","name":"your_file.py"}
```
## Additional examples

For additional examples, please see the [examples](https://github.com/thorrester/rusty-logger/tree/main/examples) directory which contains timed example of vanilla logger vs `Rusty-Logger`, `python-json-logger` vs `Rusty-Logger` as well as a multi-worker API example.

## Performance
Why would we do this when python logging is fine? Because we wanted something faster :smile:. From our own benchmarks, `Rusty-Logger` tends to be ~`1x` faster than vanilla python logging and ~2.5x faster than vanilla `JSON` logging. And while speed may not be mission critical for a few thousands logs, it can be for millions, which many companies deal with on a daily basis. Time is money and compute, and we want to save you both :moneybag: :computer:.

## Contributing
While `Rusty-Logger` is production ready out of the box, it is still in it's infancy and is ripe for additional contributions. If you'd like to contribute, please see the [contributing](https://github.com/thorrester/rusty-logger/blob/main/CONTRIBUTING.md) guide.


Thank You!!! :heart: :heart: :heart: