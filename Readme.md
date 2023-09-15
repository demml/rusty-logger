
<br>
<img src="img/rusty-logger-logo.png"  width="767" height="159" alt="rusty logger logo"/>
<br>


## Rusty Logger

Simple, opinionated and blazingly fast python logging. `Rusty-Logger` is a thin python wrapper for `Rusts` tracing library that provides minimal features for those that just want to log without crazy configurations.

## Supported Configuration

| Arg  | Description | Default |
| ------------- | :-------------:| :-------------: |
| `stdout`  | Log to stdout  | `True` |
| `stderr`  | Log to stderr  | `False` |
| `filename`  | Log to file  | `None` |
| `level`  | Level to log  | `INFO` |
| `app_env`  | Application environment (APP_ENV env var)  | `development` |
| `json_config`  | `JsonCofig`  | `None` |
| `json_config.flatten`  | Whether to flatten any passed fields  | `True` |


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