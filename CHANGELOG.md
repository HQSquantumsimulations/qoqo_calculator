# Changelog

This changelog track changes to the qoqo_calculator project starting at version 0.6.0

## Not released

## Changed

* Default towards immutable Calculator not allowing variable assignments to avoid side effects. The `parse_str` and `parse_get` functions now take immutable Calculator references and return an error when parsing variable assignments. The old behaviour has been moved to the `parse_str_aassign` function of Calculator.
* Increased minimal Python version to 3.7

## Added

* FromStr implementation for CalculatorFloat that performs a partial sanity check of expressions scanning for unrecognized string sequences and assignments.
* Default for CalculatorFloat
* abs() alias for norm() function in CalculatorComplex

## 0.6.0

### Added 0.6.0

* support for schemars jsonschema creation for CalculatorFloat and CalculatorComplex

### Changed 0.6.0

* Switch CalculatorComplex serialization to produce a tuple of CalculatorFloat serialisations in line with num_complex

* qoqo_calculator_pyo3 can now be built using a source distribution
