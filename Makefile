PROJECT=rusty-logger
PYTHON_VERSION=3.11.2
SOURCE_OBJECTS=python/rusty_logger


.PHONY: format.black
format.black:
	poetry run black ${SOURCE_OBJECTS}

.PHONY: format.ruff
format.ruff:
	poetry run ruff check --silent --exit-zero ${SOURCE_OBJECTS}

.PHONY: format
format: format.ruff format.black

.PHONY: lints.format_check
lints.format_check:
	poetry run black --check ${SOURCE_OBJECTS}

.PHONY: lints.ruff
lints.ruff:
	poetry run ruff check ${SOURCE_OBJECTS}

.PHONY: lints.mypy
lints.mypy:
	poetry run mypy ${SOURCE_OBJECTS}

.PHONY: lints.pylint
lints.pylint:
	poetry run pylint ${SOURCE_OBJECTS}

.PHONY: lints
lints: lints.ruff lints.pylint lints.mypy

.PHONY: lints.ci
lints.ci: lints.format_check lints.ruff lints.pylint lints.mypy

.PHONY: setup.project
setup.project:
	poetry install --all-extras --with dev
	pip install maturin
	maturin develop

.PHONY: test.unit
test.unit:
	poetry run pytest \
		--cov \
		--cov-fail-under=0 \
		--cov-report xml:./coverage.xml \
		--cov-report term 

.PHONY: poetry.pre.patch
poetry.pre.patch:
	poetry version prepatch

.PHONY: poetry.sub.pre.tag
poetry.sub.pre.tag:
	$(eval VER = $(shell grep "^version =" pyproject.toml | tr -d '"' | sed "s/^version = //"))
	$(eval TS = $(shell date +%s))
	$(eval REL_CANDIDATE = $(subst a0,rc.$(TS),$(VER)))
	@sed -i "s/$(VER)/$(REL_CANDIDATE)/" pyproject.toml

.PHONY: prep.pre.patch
prep.pre.patch: poetry.pre.patch poetry.sub.pre.tag

.PHONY: cargo.format
cargo.format:
	cargo fmt

.PHONY: cargo.lints
cargo.lints:
	cargo clippy --workspace --all-targets -- -D warnings

.PHONY: cargo.test
cargo.test:
	cargo test

.PHONY: cargo.bench
cargo.bench:
	cargo bench