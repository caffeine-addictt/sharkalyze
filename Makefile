PYTHON:=python
NPM:=npm
CARGO:=cargo
DOCKER:=docker

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


## dev: Start client and server in development mode
.PHONY: dev
dev:
	${DOCKER} compose up --watch


## prod: Run client server in production mode
.PHONY: prod
prod:
	${DOCKER} compose up --build


## down: Kill client and server
.PHONY: down
down:
	${DOCKER} compose down 2> ${NULL}


## install: Install dependencies
.PHONY: install
install: install/python install/npm

.PHONY: install/python
install/python:
	${PYTHON} -m pip install poetry
	${PYTHON} -m poetry install

.PHONY: install/npm
install/npm:
	${NPM} i


## test: Runs tests
.PHONY: test
test: test/python test/npm test/cargo

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
clean: clean/python clean/npm clean/cargo clean/docker

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

.PHONY: clean/docker
clean/docker:
	${DOCKER} system prune -f


## tidy: Clean up code artifacts
.PHONY: tidy
tidy: tidy/python tidy/docker

.PHONY: tidy/docker
tidy/docker: down
	${DOCKER} compose rm -f 2> ${NULL}
	${DOCKER} rmi sharkalyze-client 2> ${NULL}
	${DOCKER} rmi sharkalyze-server 2> ${NULL}

.PHONY: tidy/python
tidy/python:
	${RM_CMD} .coverage
	${RM_CMD} .pytest_cache
	${RM_CMD} .ruff_cache
	${PYTHON} -Bc "for p in __import__('pathlib').Path('.').rglob('*.py[co]'): p.unlink()"
	${PYTHON} -Bc "for p in __import__('pathlib').Path('.').rglob('__pycache__'): p.rmdir()"

