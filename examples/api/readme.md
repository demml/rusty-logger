# Multi-worker api

## Overview

This example demonstrates how to use a multi-worker API via `FastPi` with `rusty_logger`.
In this example a `logger.py` file is used to create a base logger that is called in other modules.
By default, the `ApiLogger` is set with `lock_guard = False` (default) in order to allow sharing of the global logger
across threads.

Instructions for running:

1. Make sure you are in the `examples/api` directory.
2. Set up your env (we use poetry)
```bash
poetry install --with dev
```
3. Run the server
```bash
bash server.sh
```

4. Test the healthcheck path and verify logs are being recorded.
```bash
curl localhost:8888/healthcheck
```
5. Re-run `bash server.sh` with `LogConfig(lock_guard=True)` and verify logs are not being recorded (logger is locked to main thread running workers).