# qoqo-calculator

qoqo-calculator is the calculator backend of the qoqo quantum computing toolkit by [HQS Quantum Simulations](https://quantumsimulations.de).

This repository contains two components:

* The core qoqo_calculator rust library
* The python interface qoqo_calculator_pyo3

## qoqo_calculator


[![Crates.io](https://img.shields.io/crates/v/qoqo_calculator)](https://crates.io/crates/qoqo_calculator)
[![GitHub Workflow Status](https://github.com/HQSquantumsimulations/qoqo_calculator/workflows/ci_tests/badge.svg)](https://github.com/HQSquantumsimulations/qoqo_calculator/actions)
[![docs.rs](https://img.shields.io/docsrs/qoqo_calculator)](https://docs.rs/qoqo_calculator/)
![Crates.io](https://img.shields.io/crates/l/qoqo_calculator)
[![codecov](https://codecov.io/gh/HQSquantumsimulations/qoqo_calculator/branch/main/graph/badge.svg?token=2MCD6EN4UX)](https://codecov.io/gh/HQSquantumsimulations/qoqo_calculator)

qoqo-calculator is the calculator backend of the qoqo quantum computing toolkit by [HQS Quantum Simulations](https://quantumsimulations.de).

qoqo-calculator provides

* A calculator that evaluates symbolic string expressions to float values
* CalculatorFloat: a struct that can represent a float value or a string based symbolic expression
* CalculatorComplex: a struct that represents complex numbers where real and imaginary parts can be CalculatorFloat


## qoqo_calculator_pyo3

[![Crates.io](https://img.shields.io/crates/v/qoqo_calculator_pyo3)](https://crates.io/crates/qoqo_calculator_pyo3)
[![GitHub Workflow Status](https://github.com/HQSquantumsimulations/qoqo_calculator_pyo3/workflows/ci_tests/badge.svg)](https://github.com/HQSquantumsimulations/qoqo_calculator/actions)
[![Documentation Status](https://img.shields.io/badge/docs-documentation-green)](https://hqsquantumsimulations.github.io/qoqo_calculator/)
[![docs.rs](https://img.shields.io/docsrs/qoqo_calculator_pyo3)](https://docs.rs/qoqo_calculator_pyo3/)
![Crates.io](https://img.shields.io/crates/l/qoqo_calculator_pyo3)
[![PyPI](https://img.shields.io/pypi/v/qoqo_calculator_pyo3)](https://pypi.org/project/qoqo_calculator_pyo3/)
[![PyPI - Format](https://img.shields.io/pypi/format/qoqo_calculator_pyo3)](https://pypi.org/project/qoqo_calculator_pyo3/)

Python interface to qoqo calculator, the calculator backend of the qoqo quantum computing toolkit by [HQS Quantum Simulations](https://quantumsimulations.de).

qoqo-calculator-pyo3 provides

* A calculator python class that evaluates symbolic string expressions to float values
* A CalculatorFloat python class that can represent a float value or a string based symbolic expression
* A CalculatorComplex python class that represents complex numbers where real and imaginary parts can be CalculatorFloat


### Installation

This package can be installed directly from pypi using

```shell
pip install qoqo-calculator-pyo3
```

For x86 based Linux, Windows and macOS machines pre-built binaries are available. For other platforms a working rust toolchain and [maturin](https://github.com/PyO3/maturin) are required to build the source distribution that is also available on PyPi.

## Contributing

We welcome contributions to the project. If you want to contribute code, please have a look at CONTRIBUTE.md for our code contribution guidelines.
