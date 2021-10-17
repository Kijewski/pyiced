all:

.DELETE_ON_ERROR:
.ONESHELL:

.PHONY: all clean docs install wheel

ifeq ($(strip ${PYTHON}),)
  env/: PYTHON:=$(shell readlink -e "$(shell "$(shell which where which | head -n1)" python3.10 python3 | head -n1)")
endif


all:
	@echo "There is no default or 'all' target!"
	@echo " -- make docs: build HTML documentation"
	@echo " -- make install: install into local environment"
	@echo " -- make build: build source code and binary distribution"
	@false


clean:
	-rm -r -- "./build/"
	-rm -r -- "./dist/"
	-rm -r -- "./pyiced.egg-info/"
	-rm -r -- "./src/pyiced.egg-info/"
	-rm -r -- "./target/"


env/: requirements-dev.txt
	-rm -r -- "./env/"

	"${PYTHON}" -m venv -- "./env/" || exit 1

	if [ -e ./env/bin/activate ]
	then
		. ./env/bin/activate || exit 1
	else
		set -o igncr || true
		. env/Scripts/activate || exit 1
	fi

	python -m pip install -U pip || exit 1
	python -m pip install -U wheel setuptools || exit 1
	python -m pip install -Ur requirements-dev.txt || exit 1


install: | env/
	if [ -e ./env/bin/activate ]
	then
		. ./env/bin/activate || exit 1
	else
		set -o igncr || true
		. env/Scripts/activate || exit 1
	fi
	python -m pip install . || exit 1


build: | env/
	if [ -e ./env/bin/activate ]
	then
		. ./env/bin/activate || exit 1
	else
		set -o igncr || true
		. env/Scripts/activate || exit 1
	fi
	python -m build || exit 1


docs: install | env/
	-rm -r -- "./dist/doctrees/"
	-rm -r -- "./dist/html/"

	if [ -e ./env/bin/activate ]
	then
		. ./env/bin/activate || exit 1
	else
		set -o igncr || true
		. env/Scripts/activate || exit 1
	fi
	python -m sphinx -M html ./docs/ ./dist/
