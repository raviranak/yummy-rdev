from setuptools import find_packages, setup

NAME = "feast-yummy"
REQUIRES_PYTHON = ">=3.7.0"

INSTALL_REQUIRE = [
    "feast==0.18.0",
]

DEV_REQUIRE = [
    "flake8",
    "black==21.10b0",
    "isort>=5",
    "mypy==0.790",
    "build==0.7.0",
    "twine==3.4.2",
    "pytest>=6.0.0",
]

setup(
    name=NAME,
    version="0.0.2",
    author="Qooba",
    long_description=open("README.md").read(),
    long_description_content_type="text/markdown",
    python_requires=REQUIRES_PYTHON,
    url="https://github.com/qooba/feast-yummy",
    packages=find_packages(include=["yummy"]),
    install_requires=INSTALL_REQUIRE,
    extras_require={
        "dev": DEV_REQUIRE,
    },
    keywords=("feast featurestore polars dask ray pyspark offlinestore"),
    license='Apache License, Version 2.0',
    classifiers=[
        "License :: OSI Approved :: Apache Software License",
        "Programming Language :: Python",
        "Programming Language :: Python :: 3",
        "Programming Language :: Python :: 3.7",
        "Programming Language :: Python :: 3.8",
        "Programming Language :: Python :: 3.9",
    ],
)
