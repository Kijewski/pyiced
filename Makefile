all:

.DELETE_ON_ERROR:

.PHONY: all clean docs install wheel

all:
	@echo "There is no default or 'all' target!"
	@echo " -- make docs: build HTML documentation"
	@echo " -- make install: install into local environment"
	@echo " -- make source: build source code distribution"
	@echo " -- make wheel: build binary wheel"
	@false

clean:
	-rm -r -- "./build/"
	-rm -r -- "./dist/"
	-rm -r -- "./pyiced.egg-info/"
	-rm -r -- "./src/pyiced.egg-info/"
	-rm -r -- "./target/"

env/bin/activate: requirements-dev.txt
	-rm -r -- "./env/"
	"`which python3.10 python3 | head -n1`" -m venv -- "./env/"
	. ./env/bin/activate && python3 -m pip install -U pip
	. ./env/bin/activate && python3 -m pip install -U wheel setuptools
	. ./env/bin/activate && python3 -m pip install -Ur requirements-dev.txt

install: | env/bin/activate
	. ./env/bin/activate && python3 -m pip install .

wheel: | env/bin/activate
	. ./env/bin/activate && python3 ./setup.py bdist_wheel

source: | env/bin/activate
	. ./env/bin/activate && python3 ./setup.py sdist

docs: install
	-rm -r -- "./dist/doctrees/"
	-rm -r -- "./dist/html/"
	. ./env/bin/activate && python3 -m sphinx -M html ./docs/ ./dist/
