from setuptools import setup
from setuptools_rust import RustExtension, Strip


setup(
    rust_extensions=[RustExtension('pyiced.pyiced', 'Cargo.toml', debug=False, strip=Strip.Debug)],
)
