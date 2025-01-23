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
use pyo3::types::PyTuple;
use pyo3::ToPyObject;
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
    input: &Bound<PyAny>,
) -> Result<CalculatorComplex, CalculatorError> {
    let try_real_part = input.as_ref().getattr("real");
    match try_real_part {
        Ok(x) => {
            let real_part_converted = convert_into_calculator_float(&x.as_borrowed())?;
            let try_imag_part = input.getattr("imag");
            match try_imag_part {
                Ok(y) => {
                    let imag_part_converted = convert_into_calculator_float(&y.as_borrowed())?;
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
    pub internal: CalculatorComplex,
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
    fn new(input: &Bound<PyAny>) -> PyResult<Self> {
        let converted = convert_into_calculator_complex(input).map_err(|_| {
            PyTypeError::new_err("Input can not be converted to Calculator Complex")
        })?;
        Ok(CalculatorComplexWrapper {
            internal: converted,
        })
    }

    /// Return the __repr__ magic method to represent objects in Python of CalculatorComplex.
    fn __repr__(&self) -> PyResult<String> {
        Ok(format!("{}", self.internal))
    }

    /// Return the __format__ magic method to represent objects in Python of CalculatorComplex.
    fn __format__(&self, _format_spec: &str) -> PyResult<String> {
        Ok(format!("{}", self.internal))
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
        Python::with_gil(|py| {
            let x = 0.0;
            let object = x.to_object(py);
            ((object,), HashMap::new())
        })
    }

    /// Get real and imaginary parts of CalculatorComplexWrapper for Python.
    ///
    /// # Returns
    ///
    /// `(PyObject, PyObject)` - real and imaginary parts of CalculatorComplex
    ///
    fn __getstate__(&self) -> (PyObject, PyObject) {
        Python::with_gil(|py| {
            let object_real = match self.internal.re {
                CalculatorFloat::Float(ref x) => x.to_object(py),
                CalculatorFloat::Str(ref x) => x.to_object(py),
            };
            let object_imag = match self.internal.im {
                CalculatorFloat::Float(ref x) => x.to_object(py),
                CalculatorFloat::Str(ref x) => x.to_object(py),
            };
            (object_real, object_imag)
        })
    }

    /// Set real and imaginary parts of CalculatorComplexWrapper for Python.
    fn __setstate__(&mut self, state: &Bound<PyAny>) -> PyResult<()> {
        Python::with_gil(|py| {
            let tuple: Py<PyTuple> = state.into_py(py).extract(py)?;
            let bind = tuple.bind(py);
            let arg_0 = bind.get_item(0)?;
            let arg_1 = bind.get_item(1)?;
            *self = CalculatorComplexWrapper::from_pair(&arg_0, &arg_1)?;
            Ok(())
        })
    }

    /// Convert contents of CalculatorComplex to a Python dictionary.
    fn to_dict(&self) -> HashMap<String, PyObject> {
        Python::with_gil(|py| {
            let mut dict = HashMap::new();
            dict.insert("is_calculator_complex".to_string(), true.to_object(py));
            match &self.internal.re {
                CalculatorFloat::Float(x) => {
                    dict.insert("real".to_string(), x.to_object(py));
                }
                CalculatorFloat::Str(x) => {
                    dict.insert("real".to_string(), x.to_object(py));
                }
            }
            match &self.internal.im {
                CalculatorFloat::Float(x) => {
                    dict.insert("imag".to_string(), x.to_object(py));
                }
                CalculatorFloat::Str(x) => {
                    dict.insert("imag".to_string(), x.to_object(py));
                }
            }
            dict
        })
    }

    /// Get real part of CalculatorComplex.
    #[getter]
    fn real(&self) -> CalculatorFloatWrapper {
        CalculatorFloatWrapper {
            internal: self.internal.re.clone(),
        }
    }

    /// Get imaginary part of CalculatorComplex.
    #[getter]
    fn imag(&self) -> CalculatorFloatWrapper {
        CalculatorFloatWrapper {
            internal: self.internal.im.clone(),
        }
    }

    /// Create a new instance of CalculatorComplex from a pair of values.
    #[staticmethod]
    fn from_pair(re: &Bound<PyAny>, im: &Bound<PyAny>) -> PyResult<CalculatorComplexWrapper> {
        let re_cf = convert_into_calculator_float(re).map_err(|_| {
            PyTypeError::new_err("Real input can not be converted to Calculator Complex")
        })?;
        let im_cf = convert_into_calculator_float(im).map_err(|_| {
            PyTypeError::new_err("Imag input can not be converted to Calculator Complex")
        })?;
        Ok(CalculatorComplexWrapper {
            internal: CalculatorComplex::new(re_cf, im_cf),
        })
    }

    /// Return complex conjugate of x: x*=x.re-i*x.im.
    fn conj(&self) -> CalculatorComplexWrapper {
        Self {
            internal: self.internal.conj(),
        }
    }

    /// Return phase of complex number x: arg(x).
    fn arg(&self) -> CalculatorFloatWrapper {
        CalculatorFloatWrapper {
            internal: self.internal.arg(),
        }
    }

    /// Return true when x is close to y.
    fn isclose(&self, other: &Bound<PyAny>) -> PyResult<bool> {
        let other_cc = convert_into_calculator_complex(other).map_err(|_| {
            PyTypeError::new_err("Right hand side can not be converted to Calculator Complex")
        })?;
        Ok(self.internal.isclose(other_cc))
    }

    /// Return absolute value of complex number x: |x|=(x.re^2+x.im^2)^1/2.
    fn abs(&self) -> CalculatorFloatWrapper {
        CalculatorFloatWrapper {
            internal: self.internal.norm(),
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
        let fl: Result<f64, CalculatorError> = CalculatorComplex::try_into(self.internal.clone());
        match fl {
            Ok(x) => Ok(x),
            Err(x) => Err(PyValueError::new_err(format!("{x:?}"))),
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
            CalculatorComplex::try_into(self.internal.clone());
        match com {
            Ok(x) => Ok(x),
            Err(x) => Err(PyValueError::new_err(format!("{x:?}"))),
        }
    }

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
    fn __richcmp__(&self, other: &Bound<PyAny>, op: CompareOp) -> PyResult<bool> {
        let other_cc = convert_into_calculator_complex(other).map_err(|_| {
            PyTypeError::new_err("Right hand side can not be converted to Calculator Complex")
        })?;
        match op {
            CompareOp::Eq => Ok(self.internal == other_cc),
            CompareOp::Ne => Ok(self.internal != other_cc),
            _ => Err(PyNotImplementedError::new_err(
                "Other comparison not implemented.",
            )),
        }
    }

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
    fn __add__(&self, rhs: &Bound<PyAny>) -> PyResult<CalculatorComplexWrapper> {
        let self_cc = self.internal.clone();
        let other_cc = convert_into_calculator_complex(rhs).map_err(|_| {
            PyTypeError::new_err("Right hand side can not be converted to Calculator Complex")
        })?;
        Ok(CalculatorComplexWrapper {
            internal: (self_cc + other_cc),
        })
    }

    /// Implement the `+` (__radd__) magic method to add two CalculatorComplexes.
    ///
    /// # Arguments
    ///
    /// * `self` - the first CalculatorComplexWrapper object in the operation
    /// * `other` - the second CalculatorComplexWrapper object in the operation
    ///
    /// # Returns
    ///
    /// `PyResult<CalculatorComplexWrapper>` - lhs + rhs
    ///
    fn __radd__(&self, other: &Bound<PyAny>) -> PyResult<CalculatorComplexWrapper> {
        let self_cc = self.internal.clone();
        let other_cc = convert_into_calculator_complex(other).map_err(|_| {
            PyTypeError::new_err("Right hand side can not be converted to Calculator Complex")
        })?;
        Ok(CalculatorComplexWrapper {
            internal: (other_cc + self_cc),
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
    fn __iadd__(&mut self, other: &Bound<PyAny>) -> PyResult<()> {
        let other_cc = convert_into_calculator_complex(other).map_err(|_| {
            PyTypeError::new_err("Right hand side can not be converted to Calculator Complex")
        })?;
        self.internal += other_cc;
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
    fn __sub__(&self, rhs: &Bound<PyAny>) -> PyResult<CalculatorComplexWrapper> {
        let self_cc = self.internal.clone();
        let other_cc = convert_into_calculator_complex(rhs).map_err(|_| {
            PyTypeError::new_err("Right hand side can not be converted to Calculator Complex")
        })?;
        Ok(CalculatorComplexWrapper {
            internal: (self_cc - other_cc),
        })
    }

    /// Implement the `-` (__rsub__) magic method to subtract two CalculatorComplexes.
    ///
    /// # Arguments
    ///
    /// * `self` - the first CalculatorComplexWrapper object in the operation
    /// * `other` - the second CalculatorComplexWrapper object in the operation
    ///
    /// # Returns
    ///
    /// `PyResult<CalculatorComplexWrapper>` - lhs - rhs
    ///
    fn __rsub__(&self, other: &Bound<PyAny>) -> PyResult<CalculatorComplexWrapper> {
        let self_cc = self.internal.clone();
        let other_cc = convert_into_calculator_complex(other).map_err(|_| {
            PyTypeError::new_err("Right hand side can not be converted to Calculator Complex")
        })?;
        Ok(CalculatorComplexWrapper {
            internal: (other_cc - self_cc),
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
    fn __isub__(&mut self, other: &Bound<PyAny>) -> PyResult<()> {
        let other_cc = convert_into_calculator_complex(other).map_err(|_| {
            PyTypeError::new_err("Right hand side can not be converted to Calculator Complex")
        })?;
        self.internal -= other_cc;
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
    fn __mul__(&self, rhs: &Bound<PyAny>) -> PyResult<CalculatorComplexWrapper> {
        let self_cc = self.internal.clone();
        let other_cc = convert_into_calculator_complex(rhs).map_err(|_| {
            PyTypeError::new_err("Right hand side can not be converted to Calculator Complex")
        })?;
        Ok(CalculatorComplexWrapper {
            internal: (self_cc * other_cc),
        })
    }

    /// Implement the `*` (__rmul__) magic method to multiply two CalculatorComplexes.
    ///
    /// # Arguments
    ///
    /// * `self` - the first CalculatorComplexWrapper object in the operation
    /// * `other` - the second CalculatorComplexWrapper object in the operation
    ///
    /// # Returns
    ///
    /// `PyResult<CalculatorComplexWrapper>` - lhs * rhs
    ///
    fn __rmul__(&self, other: &Bound<PyAny>) -> PyResult<CalculatorComplexWrapper> {
        let self_cc = self.internal.clone();
        let other_cc = convert_into_calculator_complex(other).map_err(|_| {
            PyTypeError::new_err("Right hand side can not be converted to Calculator Complex")
        })?;
        Ok(CalculatorComplexWrapper {
            internal: (other_cc * self_cc),
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
    fn __imul__(&mut self, other: &Bound<PyAny>) -> PyResult<()> {
        let other_cc = convert_into_calculator_complex(other).map_err(|_| {
            PyTypeError::new_err("Right hand side can not be converted to Calculator Complex")
        })?;
        self.internal *= other_cc;
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
    fn __truediv__(&self, rhs: &Bound<PyAny>) -> PyResult<CalculatorComplexWrapper> {
        let self_cc = self.internal.clone();

        let other_cc = convert_into_calculator_complex(rhs).map_err(|_| {
            PyTypeError::new_err("Right hand side can not be converted to Calculator Complex")
        })?;
        let res = catch_unwind(|| self_cc / other_cc);
        match res {
            Ok(x) => Ok(CalculatorComplexWrapper { internal: x }),
            Err(_) => Err(PyZeroDivisionError::new_err("Division by zero!")),
        }
    }

    /// Implement the `/` (__rtruediv__) magic method to divide two CalculatorComplexes.
    ///
    /// # Arguments
    ///
    /// * `self` - the first CalculatorComplexWrapper object in the operation
    /// * `other` - the second CalculatorComplexWrapper object in the operation
    ///
    /// # Returns
    ///
    /// `PyResult<CalculatorComplexWrapper>` - lhs / rhs
    ///
    fn __rtruediv__(&self, other: &Bound<PyAny>) -> PyResult<CalculatorComplexWrapper> {
        let self_cc = self.internal.clone();

        let other_cc = convert_into_calculator_complex(other).map_err(|_| {
            PyTypeError::new_err("Right hand side can not be converted to Calculator Complex")
        })?;
        let res = catch_unwind(|| other_cc / self_cc);
        match res {
            Ok(x) => Ok(CalculatorComplexWrapper { internal: x }),
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
    fn __itruediv__(&mut self, other: &Bound<PyAny>) -> PyResult<()> {
        let other_cc = convert_into_calculator_complex(other).map_err(|_| {
            PyTypeError::new_err("Right hand side can not be converted to Calculator Complex")
        })?;
        if let CalculatorFloat::Float(x) = other_cc.norm() {
            if x == 0.0 {
                return Err(PyZeroDivisionError::new_err("Division by zero!"));
            }
        }
        self.internal /= other_cc;
        Ok(())
    }

    /// Implement Python minus sign for CalculatorComplex.
    fn __neg__(&self) -> PyResult<CalculatorComplexWrapper> {
        Ok(CalculatorComplexWrapper {
            internal: -self.internal.clone(),
        })
    }

    /// Return Python absolute value abs(x) for CalculatorComplex.
    fn __abs__(&self) -> PyResult<CalculatorFloatWrapper> {
        Ok(CalculatorFloatWrapper {
            internal: self.internal.norm(),
        })
    }

    /// Implement Python Inverse `1/x` for CalculatorComplex.
    fn __invert__(&self) -> PyResult<CalculatorComplexWrapper> {
        Ok(CalculatorComplexWrapper {
            internal: self.internal.recip(),
        })
    }
}

impl CalculatorComplexWrapper {
    pub fn from_pyany(input: &Bound<PyAny>) -> PyResult<CalculatorComplex> {
        convert_into_calculator_complex(input).map_err(|err| {
            PyValueError::new_err(format!("Error in convert_to_calculator_complex: {err:?}"))
        })
    }
}
