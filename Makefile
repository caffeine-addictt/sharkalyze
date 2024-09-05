PYTHON:=python
NPM:=npm
CARGO:=cargo
RUSTUP:=rustup

ifeq ($(OS),Windows_NT)
RM_CMD:=rd /s /q
NULL:=/dev/nul
else
RM_CMD:=rm -rf
NULL:=/dev/null
endif


# =================================== DEFAULT =================================== #

default: all

## default: Installs dependencies and prints this help message
.PHONY: default
all: install help

# =================================== HELPERS =================================== #

## help: print this help message
.PHONY: help
help:
	@echo 'Usage: make [target]'
	@echo ''
	@echo 'Commands:'
	@sed -n 's/^## //p' ${MAKEFILE_LIST} | column -t -s ':' |  sed -e 's/^/ /'


## dev: Start client and server in development mode
.PHONY: dev
dev:
	trap 'kill 0' SIGINT; \
	${NPM} run dev & \
	${PYTHON} -m poetry run gunicorn server.src.main:app --reload --bind 0.0.0.0:3000 & \
	wait




## install: Install dependencies
.PHONY: install
install: install/python install/npm install/cargo
	@echo "👍 Installed dependencies!"

.PHONY: install/python
install/python:
	${PYTHON} -m pip install poetry
	${PYTHON} -m poetry install

.PHONY: install/npm
install/npm:
	${NPM} i

.PHONY: install/cargo
install/cargo:
	${RUSTUP} component add clippy
	${RUSTUP} component add rustfmt


## test: Runs tests
.PHONY: test
test: test/python test/npm test/cargo
	@echo "👍 Test passing!"

.PHONY: test/python
test/python:
	${PYTHON} -m poetry run pytest -vv --cov=. .

.PHONY: test/npm
test/npm:
	${NPM} run test

.PHONY: test/cargo
test/cargo:
	${CARGO} test


## lint: Lint code
.PHONY: lint
lint: lint/python lint/npm lint/cargo
	@echo "👍 Linting passing!"

.PHONY: lint/python
lint/python:
	${PYTHON} -m poetry run ruff format --check

.PHONY: lint/npm
lint/npm:
	${NPM} run lint

.PHONY: lint/cargo
lint/cargo:
	${CARGO} clippy


## format: Format code
.PHONY: format
format: format/python format/npm format/cargo
	@echo "👍 Formatted code!"

.PHONY: format/python
format/python:
	${PYTHON} -m poetry run ruff format

.PHONY: format/npm
format/npm:
	${NPM} run lint:fix

.PHONY: format/cargo
	${CARGO} fmt


## clean: Clean up build artifacts
.PHONY: clean
clean: clean/python clean/npm clean/cargo
	@echo "👍 Cleaned up build artifacts!"

.PHONY: clean/python
clean/python:
	${RM_CMD} venv

.PHONY: clean/npm
clean/npm:
	${RM_CMD} client/dist
	${RM_CMD} client/node_modules
	${RM_CMD} node_modules

.PHONY: clean/cargo
clean/cargo:
	 ${CARGO} clean


## tidy: Clean up code artifacts
.PHONY: tidy
tidy: tidy/python
	@echo "👍 Cleaned up code artifacts!"

.PHONY: tidy/python
tidy/python:
	${RM_CMD} .coverage
	${RM_CMD} .pytest_cache
	${RM_CMD} .ruff_cache
	${PYTHON} -Bc "for p in __import__('pathlib').Path('.').rglob('*.py[co]'): p.unlink()"
	${PYTHON} -Bc "for p in __import__('pathlib').Path('.').rglob('__pycache__'): p.rmdir()"

