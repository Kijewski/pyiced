from setuptools import setup
from setuptools_rust import RustExtension


setup(
    name="pyiced",
    version="0.1.0",
    classifiers=[
        "License :: OSI Approved :: MIT License",
        "Development Status :: 3 - Alpha",
        "Intended Audience :: Developers",
        "Programming Language :: Python",
        "Programming Language :: Rust",
        "Operating System :: POSIX",
    ],
    packages=["pyiced"],
    rust_extensions=[RustExtension("pyiced.pyiced", "Cargo.toml", debug=False, strip=True)],
    include_package_data=True,
    zip_safe=False,
)
