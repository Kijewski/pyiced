#!/usr/bin/env python3

from setuptools import setup
from setuptools_rust import RustExtension, Strip


setup(
    rust_extensions=[RustExtension(
        'pyiced._pyiced', 'Cargo.toml',
        debug=False, strip=Strip.Debug,
    )],
)
