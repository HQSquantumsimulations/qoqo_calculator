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

//! calculator_float module
//!
//! Converts the qoqo_calculator CalculatorFloat enum and methods for parsing and evaluating
//! mathematical expressions in string form to float into a Python class.

use num_complex::Complex;
use pyo3::class::basic::CompareOp;
use pyo3::exceptions::{PyNotImplementedError, PyTypeError, PyValueError, PyZeroDivisionError};
use pyo3::prelude::*;
use qoqo_calculator::{CalculatorError, CalculatorFloat};
use std::collections::HashMap;
use std::convert::From;
use std::panic::catch_unwind;

/// Convert an f64 float (or any input that can be cast to float) or a string to CalculatorFloat.
///
/// # Arguments
///
/// * `input` - the input to be converted to CalculatorFloat
///
/// # Returns
///
/// `CalculatorFloat` - the input converted to CalculatorFloat
/// `CalculatorError` - error in the conversion process
///
pub fn convert_into_calculator_float(
    input: &Bound<PyAny>,
) -> Result<CalculatorFloat, CalculatorError> {
    let try_f64_conversion = input.call_method0("__float__");
    match try_f64_conversion {
        Ok(x) => Ok(CalculatorFloat::from(
            f64::extract_bound(&x).map_err(|_| CalculatorError::NotConvertable)?,
        )),
        _ => {
            let try_str_conversion = input
                .get_type()
                .name()
                .map_err(|_| CalculatorError::NotConvertable)?;
            match try_str_conversion.to_str() {
                Ok("str") => Ok(CalculatorFloat::from(
                    String::extract_bound(input).map_err(|_| CalculatorError::NotConvertable)?,
                )),
                Ok("CalculatorFloat") => {
                    let try_cf_conversion = input
                        .call_method0("__str__")
                        .map_err(|_| CalculatorError::NotConvertable)?;
                    Ok(CalculatorFloat::from(
                        String::extract_bound(&try_cf_conversion)
                            .map_err(|_| CalculatorError::NotConvertable)?,
                    ))
                }
                _ => Err(CalculatorError::NotConvertable),
            }
        }
    }
}

#[pyclass(name = "CalculatorFloat", module = "qoqo_calculator_pyo3")]
#[derive(Clone, Debug)]
pub struct CalculatorFloatWrapper {
    pub internal: CalculatorFloat,
}
/// Python wrapper for rust CalculatorFloat from qoqo_calculator.
#[pymethods]
impl CalculatorFloatWrapper {
    /// Create new Python instance of CalculatorFloatWrapper.
    ///
    /// # Arguments
    ///
    /// * `input` - input to instantiate the CalculatorFloat with
    ///
    /// # Returns
    ///
    /// `PyResult<Self>` - CalculatorFloatWrapper of converted input or corresponding Python error
    ///
    #[new]
    fn new(input: &Bound<PyAny>) -> PyResult<Self> {
        let converted = convert_into_calculator_float(input)
            .map_err(|_| PyTypeError::new_err("Input can not be converted to Calculator Float"))?;
        Ok(CalculatorFloatWrapper {
            internal: converted,
        })
    }

    /// Return the __format__ magic method to represent objects in Python of CalculatorFloat.
    fn __format__(&self, _format_spec: &str) -> PyResult<String> {
        Ok(format!("{}", self.internal))
    }

    /// Create Python copy of CalculatorFloatWrapper.
    ///
    /// # Returns
    ///
    /// `CalculatorFloatWrapper` - clone of CalculatorFloat in a CalculatorFloatWrapper
    ///
    fn __copy__(&self) -> CalculatorFloatWrapper {
        self.clone()
    }

    /// Create Python deep copy of CalculatorFloatWrapper.
    ///
    /// # Returns
    ///
    /// `CalculatorFloatWrapper` - clone of CalculatorFloat in a CalculatorFloatWrapper
    ///
    fn __deepcopy__(&self, _memodict: Py<PyAny>) -> CalculatorFloatWrapper {
        self.clone()
    }

    /// Get new arguments for Python of CalculatorFloatWrapper.
    ///
    /// # Returns
    ///
    /// `((PyObject,), HashMap<String, String>)` - arguments of CalculatorFloat
    ///
    fn __getnewargs_ex__(&self) -> ((PyObject,), HashMap<String, String>) {
        Python::with_gil(|py| {
            let object = match self.internal {
                CalculatorFloat::Float(ref x) => x.to_object(py),
                CalculatorFloat::Str(ref x) => x.to_object(py),
            };
            ((object,), HashMap::new())
        })
    }

    /// Python getter function which returns True when
    /// CalculatorFloat does not contain symbolic expression.
    #[getter]
    fn is_float(&self) -> bool {
        self.internal.is_float()
    }

    /// Python getter function which returns True when
    /// CalculatorFloat does not contain symbolic expression.
    fn float(&self) -> PyResult<f64> {
        Ok(*self
            .internal
            .float()
            .map_err(|_| PyTypeError::new_err("Symbolic value cannot be cast to float"))?)
    }

    /// Returns square root of CalculatorFloat.
    fn sqrt(&self) -> CalculatorFloatWrapper {
        CalculatorFloatWrapper {
            internal: self.internal.sqrt(),
        }
    }

    /// Return atan2 for CalculatorFloat and generic type `Py<PyAny>`.
    ///
    /// # Arguments
    ///
    /// * `other` - Any Python object that can be converted to CalculatorFloat
    ///
    fn atan2(&self, other: &Bound<PyAny>) -> PyResult<CalculatorFloatWrapper> {
        let other_cf = convert_into_calculator_float(other).map_err(|_| {
            PyTypeError::new_err("Right hand side can not be converted to Calculator Float")
        })?;
        Ok(CalculatorFloatWrapper {
            internal: self.internal.atan2(other_cf),
        })
    }

    /// Return True if self value is close to other value.
    fn isclose(&self, other: &Bound<PyAny>) -> PyResult<bool> {
        let other_cf = convert_into_calculator_float(other).map_err(|_| {
            PyTypeError::new_err("Right hand side can not be converted to Calculator Float")
        })?;
        Ok(self.internal.isclose(other_cf))
    }

    /// Return exponential function exp(x) for CalculatorFloat.
    fn exp(&self) -> CalculatorFloatWrapper {
        CalculatorFloatWrapper {
            internal: self.internal.exp(),
        }
    }

    /// Return sine function sin(x) for CalculatorFloat.
    fn sin(&self) -> CalculatorFloatWrapper {
        CalculatorFloatWrapper {
            internal: self.internal.sin(),
        }
    }

    /// Return cosine function cos(x) for CalculatorFloat.
    fn cos(&self) -> CalculatorFloatWrapper {
        CalculatorFloatWrapper {
            internal: self.internal.cos(),
        }
    }

    /// Return arccosine function acos(x) for CalculatorFloat.
    fn acos(&self) -> CalculatorFloatWrapper {
        CalculatorFloatWrapper {
            internal: self.internal.acos(),
        }
    }

    /// Return absolute value abs(x) for CalculatorFloat.
    fn abs(&self) -> CalculatorFloatWrapper {
        CalculatorFloatWrapper {
            internal: self.internal.abs(),
        }
    }

    /// Return signum value sign(x) for CalculatorFloat.
    fn signum(&self) -> CalculatorFloatWrapper {
        CalculatorFloatWrapper {
            internal: self.internal.signum(),
        }
    }

    /// Returns signum value sign(x) for CalculatorFloat.
    fn sign(&self) -> CalculatorFloatWrapper {
        CalculatorFloatWrapper {
            internal: self.internal.signum(),
        }
    }

    /// Python getter function which returns the value stored in CalculatorFloat.
    #[getter]
    fn value(&self) -> PyObject {
        Python::with_gil(|py| match self.internal {
            CalculatorFloat::Float(ref x) => x.to_object(py),
            CalculatorFloat::Str(ref x) => x.to_object(py),
        })
    }

    /// Implement the x.__complex__() (complex(x)) Python magic method to convert a
    /// CalculatorFloat into a complex.
    ///
    /// # Returns
    ///
    /// * `PyResult<Complex<f64>>`
    ///
    /// Converts the Rust Panic when CalculatorFloat contains symbolic string value
    /// into a Python error
    ///
    fn __complex__(&self) -> PyResult<Complex<f64>> {
        match self.internal {
            CalculatorFloat::Float(x) => Ok(Complex::new(x, 0.0)),
            CalculatorFloat::Str(_) => Err(PyValueError::new_err(
                "Symbolic Value can not be cast to complex.",
            )),
        }
    }

    /// Return the __richcmp__ magic method to perform rich comparison
    /// operations on CalculatorFloat.
    ///
    /// # Arguments
    ///
    /// * `&self` - the CalculatorFloatWrapper object
    /// * `other` - the object to compare self to
    /// * `op` - equal or not equal
    ///
    /// # Returns
    ///
    /// `PyResult<bool>` - whether the two operations compared evaluated to True or False
    ///
    fn __richcmp__(&self, other: &Bound<PyAny>, op: CompareOp) -> PyResult<bool> {
        let other_cf = convert_into_calculator_float(other).map_err(|_| {
            PyTypeError::new_err("Right hand side can not be converted to Calculator Float")
        })?;
        match op {
            CompareOp::Eq => Ok(self.internal == other_cf),
            CompareOp::Ne => Ok(self.internal != other_cf),
            _ => Err(PyNotImplementedError::new_err(
                "Other comparison not implemented.",
            )),
        }
    }

    /// Return the __repr__ magic method to represent objects in Python of CalculatorFloat.
    fn __repr__(&self) -> PyResult<String> {
        Ok(format!("{}", self.internal))
    }

    /// Implement the `+` (__add__) magic method to add two CalculatorFloats.
    ///
    /// # Arguments
    ///
    /// * `lhs` - the first CalculatorFloatWrapper object in the operation
    /// * `rhs` - the second CalculatorFloatWrapper object in the operation
    ///
    /// # Returns
    ///
    /// `PyResult<CalculatorFloatWrapper>` - lhs + rhs
    ///
    fn __add__(&self, rhs: &Bound<PyAny>) -> PyResult<CalculatorFloatWrapper> {
        let self_cf = self.internal.clone();
        let other_cf = convert_into_calculator_float(rhs).map_err(|_| {
            PyTypeError::new_err("Right hand side can not be converted to Calculator Float")
        })?;
        Ok(CalculatorFloatWrapper {
            internal: (self_cf + other_cf),
        })
    }

    /// Implement the `+` (__add__) magic method to add two CalculatorFloats.
    ///
    /// # Arguments
    ///
    /// * `lhs` - the first CalculatorFloatWrapper object in the operation
    /// * `rhs` - the second CalculatorFloatWrapper object in the operation
    ///
    /// # Returns
    ///
    /// `PyResult<CalculatorFloatWrapper>` - lhs + rhs
    ///
    fn __radd__(&self, other: &Bound<PyAny>) -> PyResult<CalculatorFloatWrapper> {
        let self_cf = self.internal.clone();
        let other_cf = convert_into_calculator_float(other).map_err(|_| {
            PyTypeError::new_err("Right hand side can not be converted to Calculator Float")
        })?;
        Ok(CalculatorFloatWrapper {
            internal: (other_cf + self_cf),
        })
    }

    /// Implement the `+=` (__iadd__) magic method to add a CalculatorFloat
    /// to another CalculatorFloat.
    ///
    /// # Arguments
    ///
    /// * `self` - the CalculatorFloatWrapper object
    /// * `other` - the CalculatorFloatWrapper object to be added to self
    ///
    fn __iadd__(&mut self, other: &Bound<PyAny>) -> PyResult<()> {
        let other_cf = convert_into_calculator_float(other).map_err(|_| {
            PyTypeError::new_err("Right hand side can not be converted to Calculator Float")
        })?;
        self.internal += other_cf;
        Ok(())
    }

    /// Implement the `-` (__sub__) magic method to subtract two CalculatorFloats.
    ///
    /// # Arguments
    ///
    /// * `lhs` - the first CalculatorFloatWrapper object in the operation
    /// * `rhs` - the second CalculatorFloatWrapper object in the operation
    ///
    /// # Returns
    ///
    /// `PyResult<CalculatorFloatWrapper>` - lhs - rhs
    ///
    fn __sub__(&self, rhs: &Bound<PyAny>) -> PyResult<CalculatorFloatWrapper> {
        let self_cf = self.internal.clone();
        let other_cf = convert_into_calculator_float(rhs).map_err(|_| {
            PyTypeError::new_err("Right hand side can not be converted to Calculator Float")
        })?;
        Ok(CalculatorFloatWrapper {
            internal: (self_cf - other_cf),
        })
    }

    /// Implement the `-` (__rsub__) magic method to subtract two CalculatorFloats.
    ///
    /// # Arguments
    ///
    /// * `lhs` - the first CalculatorFloatWrapper object in the operation
    /// * `rhs` - the second CalculatorFloatWrapper object in the operation
    ///
    /// # Returns
    ///
    /// `PyResult<CalculatorFloatWrapper>` - lhs - rhs
    ///
    fn __rsub__(&self, other: &Bound<PyAny>) -> PyResult<CalculatorFloatWrapper> {
        let self_cf = self.internal.clone();
        let other_cf = convert_into_calculator_float(other).map_err(|_| {
            PyTypeError::new_err("Right hand side can not be converted to Calculator Float")
        })?;
        Ok(CalculatorFloatWrapper {
            internal: (other_cf - self_cf),
        })
    }

    /// Implement the `-=` (__isub__) magic method to subtract a CalculatorFloat
    /// from another CalculatorFloat.
    ///
    /// # Arguments
    ///
    /// * `self` - the CalculatorFloatWrapper object
    /// * `other` - the CalculatorFloatWrapper object to be subtracted from self
    ///
    fn __isub__(&mut self, other: &Bound<PyAny>) -> PyResult<()> {
        let other_cf = convert_into_calculator_float(other).map_err(|_| {
            PyTypeError::new_err("Right hand side can not be converted to Calculator Float")
        })?;
        self.internal -= other_cf;
        Ok(())
    }

    /// Implement the `*` (__mul__) magic method to multiply two CalculatorFloats.
    ///
    /// # Arguments
    ///
    /// * `lhs` - the first CalculatorFloatWrapper object in the operation
    /// * `rhs` - the second CalculatorFloatWrapper object in the operation
    ///
    /// # Returns
    ///
    /// `PyResult<CalculatorFloatWrapper>` - lhs * rhs
    ///
    fn __mul__(&self, rhs: &Bound<PyAny>) -> PyResult<CalculatorFloatWrapper> {
        let self_cf = self.internal.clone();
        let other_cf = convert_into_calculator_float(rhs).map_err(|_| {
            PyTypeError::new_err("Right hand side can not be converted to Calculator Float")
        })?;
        Ok(CalculatorFloatWrapper {
            internal: (self_cf * other_cf),
        })
    }

    /// Implement the `*` (__rmul__) magic method to multiply two CalculatorFloats.
    ///
    /// # Arguments
    ///
    /// * `lhs` - the first CalculatorFloatWrapper object in the operation
    /// * `rhs` - the second CalculatorFloatWrapper object in the operation
    ///
    /// # Returns
    ///
    /// `PyResult<CalculatorFloatWrapper>` - lhs * rhs
    ///
    fn __rmul__(&self, other: &Bound<PyAny>) -> PyResult<CalculatorFloatWrapper> {
        let self_cf = self.internal.clone();
        let other_cf = convert_into_calculator_float(other).map_err(|_| {
            PyTypeError::new_err("Right hand side can not be converted to Calculator Float")
        })?;
        Ok(CalculatorFloatWrapper {
            internal: (other_cf * self_cf),
        })
    }

    /// Implement the `*=` (__imul__) magic method to multiply a CalculatorFloat
    /// by another CalculatorFloat.
    ///
    /// # Arguments
    ///
    /// * `self` - the CalculatorFloatWrapper object
    /// * `other` - the CalculatorFloatWrapper object to multiply self by
    ///
    fn __imul__(&mut self, other: &Bound<PyAny>) -> PyResult<()> {
        let other_cf = convert_into_calculator_float(other).map_err(|_| {
            PyTypeError::new_err("Right hand side can not be converted to Calculator Float")
        })?;
        self.internal *= other_cf;
        Ok(())
    }

    /// Return __pow__ (power) for CalculatorFloat and generic type `Py<PyAny>`.
    ///
    /// # Arguments
    ///
    /// * `other` - Any Python object that can be converted to CalculatorFloat
    ///
    fn __pow__(
        &self,
        rhs: &Bound<PyAny>,
        modulo: Option<CalculatorFloatWrapper>,
    ) -> PyResult<CalculatorFloatWrapper> {
        if let Some(_x) = modulo {
            return Err(PyNotImplementedError::new_err("Modulo is not implemented"));
        }
        let self_cf = self.internal.clone();
        let other_cf = convert_into_calculator_float(rhs).map_err(|_| {
            PyTypeError::new_err("Right hand side can not be converted to Calculator Float")
        })?;
        Ok(CalculatorFloatWrapper {
            internal: (self_cf.powf(other_cf)),
        })
    }

    /// Implement the `/` (__truediv__) magic method to divide two CalculatorFloats.
    ///
    /// # Arguments
    ///
    /// * `lhs` - the first CalculatorFloatWrapper object in the operation
    /// * `rhs` - the second CalculatorFloatWrapper object in the operation
    ///
    /// # Returns
    ///
    /// `PyResult<CalculatorFloatWrapper>` - lhs / rhs
    ///
    fn __truediv__(&self, rhs: &Bound<PyAny>) -> PyResult<CalculatorFloatWrapper> {
        let self_cf = self.internal.clone();
        let other_cf = convert_into_calculator_float(rhs).map_err(|_| {
            PyTypeError::new_err("Right hand side can not be converted to Calculator Float")
        })?;
        let res = catch_unwind(|| self_cf / other_cf);
        match res {
            Ok(x) => Ok(CalculatorFloatWrapper { internal: x }),
            Err(_) => Err(PyZeroDivisionError::new_err("Division by zero!")),
        }
    }

    /// Implement the `/` (__truediv__) magic method to divide two CalculatorFloats.
    ///
    /// # Arguments
    ///
    /// * `lhs` - the first CalculatorFloatWrapper object in the operation
    /// * `rhs` - the second CalculatorFloatWrapper object in the operation
    ///
    /// # Returns
    ///
    /// `PyResult<CalculatorFloatWrapper>` - lhs / rhs
    ///
    fn __rtruediv__(&self, other: &Bound<PyAny>) -> PyResult<CalculatorFloatWrapper> {
        let self_cf = self.internal.clone();
        let other_cf = convert_into_calculator_float(other).map_err(|_| {
            PyTypeError::new_err("Right hand side can not be converted to Calculator Float")
        })?;
        let res = catch_unwind(|| other_cf / self_cf);
        match res {
            Ok(x) => Ok(CalculatorFloatWrapper { internal: x }),
            Err(_) => Err(PyZeroDivisionError::new_err("Division by zero!")),
        }
    }

    /// Implement the `/=` (__itruediv__) magic method to divide a CalculatorFloat
    /// by another CalculatorFloat.
    ///
    /// # Arguments
    ///
    /// * `self` - the CalculatorFloatWrapper object
    /// * `other` - the CalculatorFloatWrapper object to divide self by
    ///
    fn __itruediv__(&mut self, other: &Bound<PyAny>) -> PyResult<()> {
        let other_cf = convert_into_calculator_float(other).map_err(|_| {
            PyTypeError::new_err("Right hand side can not be converted to Calculator Float")
        })?;
        if let CalculatorFloat::Float(x) = other_cf {
            if x == 0.0 {
                return Err(PyZeroDivisionError::new_err("Division by zero!"));
            }
        }
        self.internal /= other_cf;
        Ok(())
    }

    /// Implement Python minus sign for CalculatorFloat.
    fn __neg__(&self) -> PyResult<CalculatorFloatWrapper> {
        Ok(CalculatorFloatWrapper {
            internal: -self.internal.clone(),
        })
    }

    /// Return Python absolute value abs(x) for CalculatorFloat.
    fn __abs__(&self) -> PyResult<CalculatorFloatWrapper> {
        Ok(CalculatorFloatWrapper {
            internal: self.internal.abs(),
        })
    }
    /// Implement Python Inverse `1/x` for CalculatorFloat.
    fn __invert__(&self) -> PyResult<CalculatorFloatWrapper> {
        Ok(CalculatorFloatWrapper {
            internal: self.internal.recip(),
        })
    }

    /// Implement the x.__float__() (float(x)) Python magic method to convert a CalculatorFloat
    /// into a float.
    ///
    /// # Returns
    ///
    /// * `PyResult<f64>`
    ///
    /// Converts the Rust Panic when CalculatorFloat contains symbolic string value
    /// into a Python error
    ///
    fn __float__(&self) -> PyResult<f64> {
        match self.internal {
            CalculatorFloat::Float(x) => Ok(x),
            CalculatorFloat::Str(_) => Err(PyValueError::new_err(
                "Symbolic Value can not be cast to float.",
            )),
        }
    }
}

impl CalculatorFloatWrapper {
    pub fn from_pyany(input: &Bound<PyAny>) -> PyResult<CalculatorFloat> {
        convert_into_calculator_float(input).map_err(|err| {
            PyValueError::new_err(format!("Error in convert_to_calculator_float: {err:?}"))
        })
    }
}
