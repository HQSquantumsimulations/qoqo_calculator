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
// limitations under the License.

//! calculator module
//!
//! Converts the qoqo_calculator Calculator struct for parsing string expressions to floats
//! into a Python class.

use crate::convert_into_calculator_float;
use pyo3::exceptions::{PyTypeError, PyValueError};
use pyo3::prelude::*;
use qoqo_calculator::Calculator;

#[pyclass(name = "Calculator", module = "qoqo_calculator_pyo3")]
pub struct CalculatorWrapper {
    pub r_calculator: Calculator,
}
#[pymethods]
impl CalculatorWrapper {
    /// Create new Python instance of CalculatorWrapper.
    ///
    /// # Returns
    ///
    /// `<Self>` - CalculatorWrapper instance of Calculator
    ///
    #[new]
    fn new() -> Self {
        let r_calculator = Calculator::new();
        CalculatorWrapper { r_calculator }
    }

    /// Set variable for Calculator.
    ///
    /// # Arguments
    ///
    /// * `variable_string` - string of the variable name
    /// * `val` - Float value of the variable
    ///
    fn set(&mut self, variable_string: &str, val: f64) {
        self.r_calculator.set_variable(variable_string, val);
    }

    ///  Parse a string expression.
    ///
    /// # Arguments
    ///
    /// * `input` - Expression that is parsed
    ///
    pub fn parse_str_assign(&mut self, input: &str) -> PyResult<f64> {
        match self.r_calculator.parse_str_assign(input) {
            Ok(x) => Ok(x),
            Err(x) => Err(PyValueError::new_err(format!("{x:?}; expression: {input}"))),
        }
    }

    ///  Parse a string expression.
    ///
    /// # Arguments
    ///
    /// * `input` - Expression that is parsed
    ///
    pub fn parse_str(&self, input: &str) -> PyResult<f64> {
        match self.r_calculator.parse_str(input) {
            Ok(x) => Ok(x),
            Err(x) => Err(PyValueError::new_err(format!("{x:?}; expression: {input}"))),
        }
    }

    /// Parse an input to float.
    ///
    /// # Arguments
    ///
    /// * `input` - Parsed string CalculatorFloat or returns float value
    ///
    pub fn parse_get(&self, input: &Bound<PyAny>) -> PyResult<f64> {
        let converted = convert_into_calculator_float(input)
            .map_err(|_| PyTypeError::new_err("Input can not be converted to Calculator Float"))?;
        let out = self.r_calculator.parse_get(converted);
        match out {
            Ok(x) => Ok(x),
            Err(x) => Err(PyValueError::new_err(format!("{x:?}"))),
        }
    }
}

///  Parse a string expression.
///
/// # Arguments
///
/// * `expression` - Expression that is parsed
///
pub fn parse_str_assign(expression: &str) -> PyResult<f64> {
    let mut calculator = Calculator::new();
    match calculator.parse_str_assign(expression) {
        Ok(x) => Ok(x),
        Err(x) => Err(PyValueError::new_err(format!(
            "{x:?}; expression {expression}"
        ))),
    }
}
