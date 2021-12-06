all: build docs


.DELETE_ON_ERROR:
.ONESHELL:
.PHONY: all build clean dists docs install sdist


ENV_DIR := env

ifeq ($(strip ${PYTHON}),)
  PREFERRED_PYTHON_VERSION ?= python3.10
  ${ENV_DIR}/: PYTHON:=$(shell readlink -e "$(shell "$(shell which where which | head -n1)" ${PREFERRED_PYTHON_VERSION} pythonw3 python3 | head -n1)")
endif


define VENV
  if [ -e "./${ENV_DIR}/bin/activate" ]; then
    . "./${ENV_DIR}/bin/activate" || exit 1;
  else
    set -o igncr || true;
    . "./${ENV_DIR}/Scripts/activate" || exit 1;
  fi
endef


clean:
	rm -r -- "./build/" || true
	rm -r -- "./dist/" || true
	rm -r -- "./pyiced.egg-info/" || true
	rm -r -- "./src/pyiced.egg-info/" || true
	rm -r -- "./target/" || true


${ENV_DIR}/: requirements-dev.txt
	rm -r -- "./${ENV_DIR}/" || true

	"${PYTHON}" -m venv -- "./${ENV_DIR}/" || exit 1

	${VENV}
	python -m pip install -U pip || exit 1
	python -m pip install -U wheel setuptools || exit 1
	python -m pip install -Ur requirements-dev.txt || exit 1


install: | ${ENV_DIR}/
	${VENV}
	python -m pip install . || exit 1


build: | ${ENV_DIR}/
	${VENV}
	python -m build || exit 1


sdist: | ${ENV_DIR}/
	${VENV}
	python -m build --sdist || exit 1


docs: install | ${ENV_DIR}/
	rm -r -- "./dist/doctrees/" || true
	rm -r -- "./dist/html/" || true

	${VENV}
	python -m sphinx -M html ./docs/ ./dist/


dists:
	${MAKE} PREFERRED_PYTHON_VERSION=python3.10 ENV_DIR=env3.10 build || true
	${MAKE} PREFERRED_PYTHON_VERSION=python3.9 ENV_DIR=env3.9 build || true
	${MAKE} PREFERRED_PYTHON_VERSION=python3.8 ENV_DIR=env3.8 build || true
	${MAKE} PREFERRED_PYTHON_VERSION=python3.7 ENV_DIR=env3.7 build || true
