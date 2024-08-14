# Changelog

This changelog track changes to the qoqo_calculator project starting at version 0.6.0

## 1.2.4

* Updated maturin version from 0.14-0.15 to >=1.4

## 1.2.3

* Updated to rust 1.70
* Added the `pyo3 = "0.21"` line to the Cargo.toml in the build dependencies.

## 1.2.2

* Reverts previous fix as it is no longer needed.

## 1.2.1

* Fixes a compatibility issue for from_pyany with struqture and qoqo.

## 1.2.0

* Update to pyo3 0.21

## 1.1.5

* Update to pyo3 0.20

## 1.1.4

* Fixes issue when deserializing a float from json when float is represented by integer (e.g. `1` instead of `1.0`)

## 1.1.3

* Update to python 3.12

## 1.1.2

* Update to pyo3 0.19

## 1.1.1

* Update to pyo3 0.18

## 1.1.0

* Update dependencies, update qoqo_calculator_pyo3 to build with pyo3 0.17
* Moved metadata for python package to pyproject.toml
* Divergent version numbers in Python and Rust for qoqo_calculator_pyo3

## 1.0.0

### Added in 1.0.0

* Uses PyO3 build config

## 0.8.0

### Changed in 0.8.0

* Updating naming scheme of internal wrappers in Python interface for downstream compatability

## 0.7.0

### Changed in 0.7.0

* Default towards immutable Calculator not allowing variable assignments to avoid side effects. The `parse_str` and `parse_get` functions now take immutable Calculator references and return an error when parsing variable assignments. The old behaviour has been moved to the `parse_str_aassign` function of Calculator.
* Increased minimal Python version to 3.7
* Removed (soon to be) deprecaded Python::aquire_gil in pyo3 interface
* Updated pyo3 to 0.16

### Added in 0.7.0

* FromStr implementation for CalculatorFloat that performs a partial sanity check of expressions scanning for unrecognized string sequences and assignments.
* Default for CalculatorFloat
* abs() alias for norm() function in CalculatorComplex

## 0.6.0

### Added 0.6.0

* support for schemars jsonschema creation for CalculatorFloat and CalculatorComplex

### Changed 0.6.0

* Switch CalculatorComplex serialization to produce a tuple of CalculatorFloat serialisations in line with num_complex

* qoqo_calculator_pyo3 can now be built using a source distribution
