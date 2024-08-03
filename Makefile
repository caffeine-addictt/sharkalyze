PYTHON:=python

ifeq ($(OS),Windows_NT)
RM_CMD:=rd /s /q
NULL:=/dev/nul
else
RM_CMD:=rm -rf
NULL:=/dev/null
endif


# =================================== DEFAULT =================================== #

default: all

## default: Runs build and test
.PHONY: default
all: lint test

# =================================== HELPERS =================================== #

## help: print this help message
.PHONY: help
help:
	@echo 'Usage: make [target]'
	@echo ''
	@echo 'Commands:'
	@sed -n 's/^## //p' ${MAKEFILE_LIST} | column -t -s ':' |  sed -e 's/^/ /'

## install: Install dependencies
.PHONY: install
install:
	${PYTHON} -m pip install poetry
	${PYTHON} -m poetry install

## test: Runs tests
.PHONY: test
test:
	${PYTHON} -m poetry run pytest -vv

## lint: Lint code
.PHONY: lint
lint:
	${PYTHON} -m poetry run ruff format --check

## format: Format code
.PHONY: format
format:
	${PYTHON} -m poetry run ruff format

## tidy: Clean up code artifacts
.PHONY: tidy
tidy:
	${RM_CMD} .pytest_cache
	${RM_CMD} .ruff_cache
	${PYTHON} -Bc "for p in __import__('pathlib').Path('.').rglob('*.py[co]'): p.unlink()"
	${PYTHON} -Bc "for p in __import__('pathlib').Path('.').rglob('__pycache__'): p.rmdir()"
