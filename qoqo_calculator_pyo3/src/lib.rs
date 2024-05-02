// Copyright © 2020-2021 HQS Quantum Simulations GmbH. All Rights Reserved.
//
// Licensed under the Apache License, Version 2.0 (the "License"); you may not use this file except
// in compliance with the License. You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software distributed under the
// License is distributed on an "AS IS" BASIS, WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either
// express or implied. See the License for the specific language governing permissions and
// limitations underthe License.

//! library module
//!
//! qoqo_calculator_pyo3 module bringing the qoqo_calculator rust library to Python.

use pyo3::prelude::*;
use pyo3::wrap_pyfunction;
mod calculator_float;
pub use calculator_float::convert_into_calculator_float;
pub use calculator_float::CalculatorFloatWrapper;
mod calculator_complex;
pub use calculator_complex::convert_into_calculator_complex;
pub use calculator_complex::CalculatorComplexWrapper;
mod calculator;
pub use calculator::parse_str_assign;
pub use calculator::CalculatorWrapper;

#[pyfunction]
fn parse_string_assign(expression: &str) -> PyResult<f64> {
    parse_str_assign(expression)
}

/// qoqo_calculator_pyo3 module bringing the qoqo_calculator rust library to Python.
///
/// qoqo_calculator is a rust library implementing:
/// * Calculator: a struct for parsing string expressions to floats
/// * CalculatorFloat: a type that contains a float or a symbolic math
///                    expression in string form.
/// * CalculatorComplex: a type that contains a CalculatorFloat as the real part
///                      and a CalculatorFloat as the imaginary part
///
/// Uses the pyo3 rust crate to create the Python bindings.
///
#[pymodule]
fn qoqo_calculator_pyo3(_py: Python, m: &Bound<PyModule>) -> PyResult<()> {
    m.add_class::<CalculatorWrapper>()?;
    m.add_class::<CalculatorFloatWrapper>()?;
    m.add_class::<CalculatorComplexWrapper>()?;
    m.add_function(wrap_pyfunction!(parse_string_assign, m)?)
        .unwrap();
    Ok(())
}
