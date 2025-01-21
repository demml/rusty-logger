PROJECT=rusty-logger
PYTHON_VERSION=3.12.8
SOURCE_OBJECTS=python/rusty_logger

.PHONY: format
format:
	cargo fmt

.PHONY: lints
lints:
	cargo clippy --workspace --all-targets -- -D warnings
