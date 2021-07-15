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
//! Converts the qoqo_calculator CalculatorComplex struct and methods for parsing and evaluating
//! mathematical expressions in string form to complex into a Python class.

use crate::{convert_into_calculator_float, CalculatorFloatWrapper};
use num_complex::Complex;
use pyo3::class::basic::CompareOp;
use pyo3::exceptions::{PyNotImplementedError, PyTypeError, PyValueError, PyZeroDivisionError};
use pyo3::prelude::*;
use pyo3::ToPyObject;
use pyo3::{PyNumberProtocol, PyObjectProtocol};
use qoqo_calculator::{CalculatorComplex, CalculatorError, CalculatorFloat};
use std::collections::HashMap;
use std::convert::TryInto;
use std::panic::catch_unwind;

/// Convert an f64 float (or any input that can be cast to float) or a string to CalculatorComplex.
///
/// # Arguments
///
/// * `input` - the input to be converted to CalculatorComplex
///
/// # Returns
///
/// `CalculatorFloat` - the input converted to CalculatorComplex
/// `CalculatorError` - error in the conversion process
///
pub fn convert_into_calculator_complex(
    input: &PyAny,
) -> Result<CalculatorComplex, CalculatorError> {
    let try_real_part = input.getattr("real");
    match try_real_part {
        Ok(x) => {
            let real_part_converted = convert_into_calculator_float(x)?;
            let try_imag_part = input.getattr("imag");
            match try_imag_part {
                Ok(y) => {
                    let imag_part_converted = convert_into_calculator_float(y)?;
                    Ok(CalculatorComplex::new(
                        real_part_converted,
                        imag_part_converted,
                    ))
                }
                _ => Err(CalculatorError::NotConvertable),
            }
        }
        _ => {
            let str_converted = convert_into_calculator_float(input)?;
            Ok(CalculatorComplex::new(str_converted, 0.0))
        }
    }
}

#[pyclass(name = "CalculatorComplex", module = "qoqo_calculator_pyo3")]
#[derive(Clone, Debug)]
pub struct CalculatorComplexWrapper {
    pub cc_internal: CalculatorComplex,
}

/// Python wrapper for rust CalculatorComplex from qoqo_calculator.
#[pymethods]
impl CalculatorComplexWrapper {
    /// Create new Python instance of CalculatorComplexWrapper.
    ///
    /// # Arguments
    ///
    /// * `input` - input to instantiate the CalculatorComplex with
    ///
    /// # Returns
    ///
    /// `PyResult<Self>` - CalculatorComplexWrapper of converted input or corresponding Python error
    ///
    #[new]
    fn new(input: &PyAny) -> PyResult<Self> {
        let converted = convert_into_calculator_complex(input).map_err(|_| {
            PyTypeError::new_err("Input can not be converted to Calculator Complex")
        })?;
        Ok(CalculatorComplexWrapper {
            cc_internal: converted,
        })
    }

    /// Return the __repr__ magic method to represent objects in Python of CalculatorComplex.
    fn __repr__(&self) -> PyResult<String> {
        Ok(format!("{}", self.cc_internal))
    }

    /// Return the __format__ magic method to represent objects in Python of CalculatorComplex.
    fn __format__(&self, _format_spec: &str) -> PyResult<String> {
        Ok(format!("{}", self.cc_internal))
    }

    /// Create Python copy of CalculatorComplexWrapper.
    ///
    /// # Returns
    ///
    /// `CalculatorComplexWrapper` - clone of CalculatorFloat in a CalculatorComplexWrapper
    ///
    fn __copy__(&self) -> CalculatorComplexWrapper {
        self.clone()
    }

    /// Create Python deep copy of CalculatorComplexWrapper.
    ///
    /// # Returns
    ///
    /// `CalculatorComplexWrapper` - clone of CalculatorFloat in a CalculatorComplexWrapper
    ///
    fn __deepcopy__(&self, _memodict: Py<PyAny>) -> CalculatorComplexWrapper {
        self.clone()
    }

    /// Get new arguments for Python of CalculatorComplexWrapper.
    ///
    /// # Returns
    ///
    /// `((PyObject,), HashMap<String, String>)` - arguments of CalculatorComplex
    ///
    fn __getnewargs_ex__(&self) -> ((PyObject,), HashMap<String, String>) {
        let gil = pyo3::Python::acquire_gil();
        let py = gil.python();
        let x = 0.0;
        let object = x.to_object(py);
        ((object,), HashMap::new())
    }

    /// Get real and imaginary parts of CalculatorComplexWrapper for Python.
    ///
    /// # Returns
    ///
    /// `(PyObject, PyObject)` - real and imaginary parts of CalculatorComplex
    ///
    fn __getstate__(&self) -> (PyObject, PyObject) {
        let gil = pyo3::Python::acquire_gil();
        let py = gil.python();
        let object_real = match self.cc_internal.re {
            CalculatorFloat::Float(ref x) => x.to_object(py),
            CalculatorFloat::Str(ref x) => x.to_object(py),
        };
        let object_imag = match self.cc_internal.im {
            CalculatorFloat::Float(ref x) => x.to_object(py),
            CalculatorFloat::Str(ref x) => x.to_object(py),
        };
        (object_real, object_imag)
    }

    /// Set real and imaginary parts of CalculatorComplexWrapper for Python.
    fn __setstate__(&mut self, state: (Py<PyAny>, Py<PyAny>)) -> PyResult<()> {
        *self = CalculatorComplexWrapper::from_pair(state.0, state.1)?;
        Ok(())
    }

    /// Convert contents of CalculatorComplex to a Python dictionary.
    fn to_dict(&self) -> HashMap<String, PyObject> {
        let mut dict = HashMap::new();
        let gil = pyo3::Python::acquire_gil();
        let py = gil.python();
        dict.insert("is_calculator_complex".to_string(), true.to_object(py));
        match &self.cc_internal.re {
            CalculatorFloat::Float(x) => {
                dict.insert("real".to_string(), x.to_object(py));
            }
            CalculatorFloat::Str(x) => {
                dict.insert("real".to_string(), x.to_object(py));
            }
        }
        match &self.cc_internal.im {
            CalculatorFloat::Float(x) => {
                dict.insert("imag".to_string(), x.to_object(py));
            }
            CalculatorFloat::Str(x) => {
                dict.insert("imag".to_string(), x.to_object(py));
            }
        }
        dict
    }

    /// Get real part of CalculatorComplex.
    #[getter]
    fn real(&self) -> CalculatorFloatWrapper {
        CalculatorFloatWrapper {
            cf_internal: self.cc_internal.re.clone(),
        }
    }

    /// Get imaginary part of CalculatorComplex.
    #[getter]
    fn imag(&self) -> CalculatorFloatWrapper {
        CalculatorFloatWrapper {
            cf_internal: self.cc_internal.im.clone(),
        }
    }

    /// Create a new instance of CalculatorComplex from a pair of values.
    #[staticmethod]
    fn from_pair(re: Py<PyAny>, im: Py<PyAny>) -> PyResult<CalculatorComplexWrapper> {
        let gil = pyo3::Python::acquire_gil();
        let py = gil.python();
        let re_ref = re.as_ref(py);
        let imag_ref = im.as_ref(py);
        let re_cf = convert_into_calculator_float(re_ref).map_err(|_| {
            PyTypeError::new_err("Real input can not be converted to Calculator Complex")
        })?;
        let im_cf = convert_into_calculator_float(imag_ref).map_err(|_| {
            PyTypeError::new_err("Imag input can not be converted to Calculator Complex")
        })?;
        Ok(CalculatorComplexWrapper {
            cc_internal: CalculatorComplex::new(re_cf, im_cf),
        })
    }

    /// Return complex conjugate of x: x*=x.re-i*x.im.
    fn conj(&self) -> CalculatorComplexWrapper {
        Self {
            cc_internal: self.cc_internal.conj(),
        }
    }

    /// Return phase of complex number x: arg(x).
    fn arg(&self) -> CalculatorFloatWrapper {
        CalculatorFloatWrapper {
            cf_internal: self.cc_internal.arg(),
        }
    }

    /// Return true when x is close to y.
    fn isclose(&self, other: Py<PyAny>) -> PyResult<bool> {
        let gil = pyo3::Python::acquire_gil();
        let py = gil.python();
        let other_ref = other.as_ref(py);
        let other_cc = convert_into_calculator_complex(other_ref).map_err(|_| {
            PyTypeError::new_err("Right hand side can not be converted to Calculator Complex")
        })?;
        Ok(self.cc_internal.isclose(other_cc))
    }

    /// Return absolute value of complex number x: |x|=(x.re^2+x.im^2)^1/2.
    fn abs(&self) -> CalculatorFloatWrapper {
        CalculatorFloatWrapper {
            cf_internal: self.cc_internal.norm(),
        }
    }

    /// Implement the x.__float__() (float(x)) Python magic method to convert a CalculatorComplex
    /// into a float.
    ///
    /// # Returns
    ///
    /// * `PyResult<f64>`
    ///
    /// Converts the Rust Panic when CalculatorComplex contains symbolic string value
    /// into a Python error
    ///
    fn __float__(&self) -> PyResult<f64> {
        let fl: Result<f64, CalculatorError> =
            CalculatorComplex::try_into(self.cc_internal.clone());
        match fl {
            Ok(x) => Ok(x),
            Err(x) => Err(PyValueError::new_err(format!("{:?}", x))),
        }
    }

    /// Implement the x.__complex__() (complex(x)) Python magic method to convert a
    /// CalculatorComplex into a complex.
    ///
    /// # Returns
    ///
    /// * `PyResult<Complex<f64>>`
    ///
    /// Converts the Rust Panic when CalculatorComplex contains symbolic string value
    /// into a Python error
    ///
    fn __complex__(&self) -> PyResult<Complex<f64>> {
        let com: Result<Complex<f64>, CalculatorError> =
            CalculatorComplex::try_into(self.cc_internal.clone());
        match com {
            Ok(x) => Ok(x),
            Err(x) => Err(PyValueError::new_err(format!("{:?}", x))),
        }
    }
}

#[pyproto]
impl PyObjectProtocol for CalculatorComplexWrapper {
    /// Return the __richcmp__ magic method to perform rich comparison.
    /// operations on CalculatorComplex.
    ///
    /// # Arguments
    ///
    /// * `&self` - the CalculatorComplexWrapper object
    /// * `other` - the object to compare self to
    /// * `op` - equal or not equal
    ///
    /// # Returns
    ///
    /// `PyResult<bool>` - whether the two operations compared evaluated to True or False
    ///
    fn __richcmp__(&self, other: Py<PyAny>, op: CompareOp) -> PyResult<bool> {
        let gil = pyo3::Python::acquire_gil();
        let py = gil.python();
        let other_ref = other.as_ref(py);
        let other_cc = convert_into_calculator_complex(other_ref).map_err(|_| {
            PyTypeError::new_err("Right hand side can not be converted to Calculator Complex")
        })?;
        match op {
            CompareOp::Eq => Ok(self.cc_internal == other_cc),
            CompareOp::Ne => Ok(self.cc_internal != other_cc),
            _ => Err(PyNotImplementedError::new_err(
                "Other comparison not implemented.",
            )),
        }
    }
}

#[pyproto]
impl PyNumberProtocol for CalculatorComplexWrapper {
    /// Implement the `+` (__add__) magic method to add two CalculatorComplexes.
    ///
    /// # Arguments
    ///
    /// * `lhs` - the first CalculatorComplexWrapper object in the operation
    /// * `rhs` - the second CalculatorComplexWrapper object in the operation
    ///
    /// # Returns
    ///
    /// `PyResult<CalculatorComplexWrapper>` - lhs + rhs
    ///
    fn __add__(lhs: Py<PyAny>, rhs: Py<PyAny>) -> PyResult<CalculatorComplexWrapper> {
        let gil = pyo3::Python::acquire_gil();
        let py = gil.python();
        let lhs_ref = lhs.as_ref(py);
        let rhs_ref = rhs.as_ref(py);
        let self_cc = convert_into_calculator_complex(lhs_ref).map_err(|_| {
            PyTypeError::new_err("Left hand side can not be converted to Calculator Complex")
        })?;
        let other_cc = convert_into_calculator_complex(rhs_ref).map_err(|_| {
            PyTypeError::new_err("Right hand side can not be converted to Calculator Complex")
        })?;
        Ok(CalculatorComplexWrapper {
            cc_internal: (self_cc + other_cc),
        })
    }

    /// Implement the `+=` (__iadd__) magic method to add a CalculatorComplex
    /// to another CalculatorComplex.
    ///
    /// # Arguments
    ///
    /// * `self` - the CalculatorComplexWrapper object
    /// * `other` - the CalculatorComplexWrapper object to be added to self
    ///
    fn __iadd__(&mut self, other: Py<PyAny>) -> PyResult<()> {
        let gil = pyo3::Python::acquire_gil();
        let py = gil.python();
        let other_ref = other.as_ref(py);
        let other_cc = convert_into_calculator_complex(other_ref).map_err(|_| {
            PyTypeError::new_err("Right hand side can not be converted to Calculator Complex")
        })?;
        self.cc_internal += other_cc;
        Ok(())
    }

    /// Implement the `-` (__sub__) magic method to subtract two CalculatorComplexes.
    ///
    /// # Arguments
    ///
    /// * `lhs` - the first CalculatorComplexWrapper object in the operation
    /// * `rhs` - the second CalculatorComplexWrapper object in the operation
    ///
    /// # Returns
    ///
    /// `PyResult<CalculatorComplexWrapper>` - lhs - rhs
    ///
    fn __sub__(lhs: Py<PyAny>, rhs: Py<PyAny>) -> PyResult<CalculatorComplexWrapper> {
        let gil = pyo3::Python::acquire_gil();
        let py = gil.python();
        let lhs_ref = lhs.as_ref(py);
        let rhs_ref = rhs.as_ref(py);
        let self_cc = convert_into_calculator_complex(lhs_ref).map_err(|_| {
            PyTypeError::new_err("Left hand side can not be converted to Calculator Complex")
        })?;
        let other_cc = convert_into_calculator_complex(rhs_ref).map_err(|_| {
            PyTypeError::new_err("Right hand side can not be converted to Calculator Complex")
        })?;
        Ok(CalculatorComplexWrapper {
            cc_internal: (self_cc - other_cc),
        })
    }

    /// Implement the `-=` (__isub__) magic method to subtract a CalculatorComplex
    /// from another CalculatorComplex.
    ///
    /// # Arguments
    ///
    /// * `self` - the CalculatorComplexWrapper object
    /// * `other` - the CalculatorComplexWrapper object to be subtracted from self
    ///
    fn __isub__(&mut self, other: Py<PyAny>) -> PyResult<()> {
        let gil = pyo3::Python::acquire_gil();
        let py = gil.python();
        let other_ref = other.as_ref(py);
        let other_cc = convert_into_calculator_complex(other_ref).map_err(|_| {
            PyTypeError::new_err("Right hand side can not be converted to Calculator Complex")
        })?;
        self.cc_internal -= other_cc;
        Ok(())
    }

    /// Implement the `*` (__mul__) magic method to multiply two CalculatorComplexes.
    ///
    /// # Arguments
    ///
    /// * `lhs` - the first CalculatorComplexWrapper object in the operation
    /// * `rhs` - the second CalculatorComplexWrapper object in the operation
    ///
    /// # Returns
    ///
    /// `PyResult<CalculatorComplexWrapper>` - lhs * rhs
    ///
    fn __mul__(lhs: Py<PyAny>, rhs: Py<PyAny>) -> PyResult<CalculatorComplexWrapper> {
        let gil = pyo3::Python::acquire_gil();
        let py = gil.python();
        let lhs_ref = lhs.as_ref(py);
        let rhs_ref = rhs.as_ref(py);
        let self_cc = convert_into_calculator_complex(lhs_ref).map_err(|_| {
            PyTypeError::new_err("Left hand side can not be converted to Calculator Complex")
        })?;
        let other_cc = convert_into_calculator_complex(rhs_ref).map_err(|_| {
            PyTypeError::new_err("Right hand side can not be converted to Calculator Complex")
        })?;
        Ok(CalculatorComplexWrapper {
            cc_internal: (self_cc * other_cc),
        })
    }

    /// Implement the `*=` (__imul__) magic method to multiply a CalculatorComplex
    /// by another CalculatorComplex.
    ///
    /// # Arguments
    ///
    /// * `self` - the CalculatorComplexWrapper object
    /// * `other` - the CalculatorComplexWrapper object to multiply self by
    ///
    fn __imul__(&mut self, other: Py<PyAny>) -> PyResult<()> {
        let gil = pyo3::Python::acquire_gil();
        let py = gil.python();
        let other_ref = other.as_ref(py);
        let other_cc = convert_into_calculator_complex(other_ref).map_err(|_| {
            PyTypeError::new_err("Right hand side can not be converted to Calculator Complex")
        })?;
        self.cc_internal *= other_cc;
        Ok(())
    }

    /// Implement the `/` (__truediv__) magic method to divide two CalculatorComplexes.
    ///
    /// # Arguments
    ///
    /// * `lhs` - the first CalculatorComplexWrapper object in the operation
    /// * `rhs` - the second CalculatorComplexWrapper object in the operation
    ///
    /// # Returns
    ///
    /// `PyResult<CalculatorComplexWrapper>` - lhs / rhs
    ///
    fn __truediv__(lhs: Py<PyAny>, rhs: Py<PyAny>) -> PyResult<CalculatorComplexWrapper> {
        let gil = pyo3::Python::acquire_gil();
        let py = gil.python();
        let lhs_ref = lhs.as_ref(py);
        let rhs_ref = rhs.as_ref(py);
        let self_cc = convert_into_calculator_complex(lhs_ref).map_err(|_| {
            PyTypeError::new_err("Left hand side can not be converted to Calculator Complex")
        })?;
        let other_cc = convert_into_calculator_complex(rhs_ref).map_err(|_| {
            PyTypeError::new_err("Right hand side can not be converted to Calculator Complex")
        })?;
        let res = catch_unwind(|| self_cc / other_cc);
        match res {
            Ok(x) => Ok(CalculatorComplexWrapper { cc_internal: x }),
            Err(_) => Err(PyZeroDivisionError::new_err("Division by zero!")),
        }
    }

    /// Implement the `/=` (__itruediv__) magic method to divide a CalculatorComplex
    /// by another CalculatorComplex.
    ///
    /// # Arguments
    ///
    /// * `self` - the CalculatorComplexWrapper object
    /// * `other` - the CalculatorComplexWrapper object to divide self by
    ///
    fn __itruediv__(&mut self, other: Py<PyAny>) -> PyResult<()> {
        let gil = pyo3::Python::acquire_gil();
        let py = gil.python();
        let other_ref = other.as_ref(py);
        let other_cc = convert_into_calculator_complex(other_ref).map_err(|_| {
            PyTypeError::new_err("Right hand side can not be converted to Calculator Complex")
        })?;
        if let CalculatorFloat::Float(x) = other_cc.norm() {
            if x == 0.0 {
                return Err(PyZeroDivisionError::new_err("Division by zero!"));
            }
        }
        self.cc_internal /= other_cc;
        Ok(())
    }

    /// Implement Python minus sign for CalculatorComplex.
    fn __neg__(&self) -> PyResult<CalculatorComplexWrapper> {
        Ok(CalculatorComplexWrapper {
            cc_internal: -self.cc_internal.clone(),
        })
    }

    /// Return Python absolute value abs(x) for CalculatorComplex.
    fn __abs__(&self) -> PyResult<CalculatorFloatWrapper> {
        Ok(CalculatorFloatWrapper {
            cf_internal: self.cc_internal.norm(),
        })
    }

    /// Implement Python Inverse `1/x` for CalculatorComplex.
    fn __invert__(&self) -> PyResult<CalculatorComplexWrapper> {
        Ok(CalculatorComplexWrapper {
            cc_internal: self.cc_internal.recip(),
        })
    }
}
