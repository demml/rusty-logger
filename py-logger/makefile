PROJECT=rusty-logger
PYTHON_VERSION=3.12.8
SOURCE_OBJECTS=python/rusty_logger

format.isort:
	uv run isort ${SOURCE_OBJECTS}
format.black:
	uv run black ${SOURCE_OBJECTS}
format.ruff:
	uv run ruff check --silent --exit-zero ${SOURCE_OBJECTS}
format: format.isort format.ruff format.black

lints.format_check:
	uv run black --check ${SOURCE_OBJECTS}
lints.ruff:
	uv run ruff check ${SOURCE_OBJECTS}
lints.mypy:
	uv run mypy ${SOURCE_OBJECTS}
lints.pylint:
	uv run pylint ${SOURCE_OBJECTS}
lints: lints.ruff lints.pylint lints.mypy
lints.ci: lints.format_check lints.ruff lints.pylint lints.mypy

setup.project:
	uv sync --all-extras --group dev
	uv run maturin develop --uv

test.unit:
	uv run pytest \
		--cov \
		--cov-fail-under=0 \
		--cov-report xml:./coverage.xml \
		--cov-report term 