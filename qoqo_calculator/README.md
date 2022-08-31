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


## Contributing

We welcome contributions to the project. If you want to contribute code, please have a look at CONTRIBUTE.md for our code contribution guidelines.
