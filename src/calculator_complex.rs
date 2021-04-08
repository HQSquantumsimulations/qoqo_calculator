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

//! calculator_complex module
//!
//! Provides CalculatorComplex struct and methods for parsing and evaluating
//! mathematical expressions in string form to complex.

use crate::CalculatorError;
use crate::CalculatorFloat;
use num_complex::Complex;
use serde::{Deserialize, Serialize};
use std::convert::TryFrom;
use std::fmt;
use std::ops;

/// Struct CalculatorComplex.
///
/// # Fields
///
/// * `re` - CalculatorFloat value of real part of CalculatorComplex
/// * `im` - CalculatorFloat value of imaginary part of CalculatorComplex
///
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CalculatorComplex {
    pub re: CalculatorFloat,
    pub im: CalculatorFloat,
}

/// Implement Default value 0 for CalculatorComplex.
impl Default for CalculatorComplex {
    fn default() -> Self {
        CalculatorComplex {
            re: CalculatorFloat::from(0),
            im: CalculatorFloat::from(0),
        }
    }
}

/// Initialize CalculatorComplex from CalculatorComplex reference &CalculatorComplex.
///
/// # Returns
///
/// * `CalculatorFloat`
///
impl<'a> From<&'a CalculatorComplex> for CalculatorComplex {
    fn from(item: &'a CalculatorComplex) -> Self {
        (*item).clone()
    }
}

/// Initialize CalculatorComplex from type that can be cast to CalculatorFloat.
///
/// # Returns
///
/// * `CalculatorComplex`
///
impl<T> From<T> for CalculatorComplex
where
    CalculatorFloat: From<T>,
{
    fn from(item: T) -> Self {
        Self {
            re: CalculatorFloat::from(item),
            im: CalculatorFloat::Float(0.0),
        }
    }
}

/// Initialize CalculatorComplex from Complex.
///
/// # Returns
///
/// * `CalculatorComplex`
///
impl From<Complex<f64>> for CalculatorComplex {
    fn from(item: Complex<f64>) -> Self {
        Self {
            re: CalculatorFloat::from(item.re),
            im: CalculatorFloat::from(item.im),
        }
    }
}

/// Try turning CalculatorComplex into f64 float.
///
/// # Returns
///
/// * `f64`
///
/// # Panics
///
/// Panics when CalculatorFloat contains symbolic string value
///
impl TryFrom<CalculatorComplex> for f64 {
    type Error = CalculatorError;

    fn try_from(value: CalculatorComplex) -> Result<Self, Self::Error> {
        match value.im {
            CalculatorFloat::Float(x) => {
                if x != 0.0 {
                    return Err(CalculatorError::ComplexCanNotBeConvertedToFloat { val: value });
                }
            }
            _ => return Err(CalculatorError::ComplexSymbolicNotConvertable { val: value }),
        }
        match value.re {
            CalculatorFloat::Float(x) => Ok(x),
            CalculatorFloat::Str(_) => {
                Err(CalculatorError::ComplexSymbolicNotConvertable { val: value })
            }
        }
    }
}

/// Try turning CalculatorComplex into Complex<f64> float.
///
/// # Returns
///
/// * `f64`
///
/// # Panics
///
/// Panics when CalculatorFloat contains symbolic string value
///
impl TryFrom<CalculatorComplex> for Complex<f64> {
    type Error = CalculatorError;

    fn try_from(value: CalculatorComplex) -> Result<Self, CalculatorError> {
        let im = match value.im {
            CalculatorFloat::Float(x) => x,
            _ => return Err(CalculatorError::ComplexSymbolicNotConvertable { val: value }),
        };
        let re = match value.re {
            CalculatorFloat::Float(x) => x,
            CalculatorFloat::Str(_) => {
                return Err(CalculatorError::ComplexSymbolicNotConvertable { val: value })
            }
        };
        Ok(Complex::new(re, im))
    }
}

/// Implement Display trait for CalculatorComplex.
///
/// Allows use of simple text formating
///
impl fmt::Display for CalculatorComplex {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({} + i * {})", self.re, self.im)
    }
}

impl CalculatorComplex {
    /// Return CalculatorComplex constructed form pair of real values.
    ///
    /// # Arguments
    ///
    /// * `re` - Real part given as type that can be converted to CalculatorFloat
    /// * `im` - Imaginary part given as type that can be converted to CalculatorFloat
    ///
    pub fn new<T1, T2>(re: T1, im: T2) -> Self
    where
        CalculatorFloat: From<T1>,
        CalculatorFloat: From<T2>,
    {
        Self {
            re: CalculatorFloat::from(re),
            im: CalculatorFloat::from(im),
        }
    }

    /// Return phase of complex number x: arg(x).
    pub fn arg(&self) -> CalculatorFloat {
        self.im.atan2(&self.re)
    }
    /// Return square norm of complex number x: |x|^2=x.re^2+x.im^2.
    pub fn norm_sqr(&self) -> CalculatorFloat {
        (self.re.clone() * &self.re) + (self.im.clone() * &self.im)
    }
    /// Return norm of complex number x: |x|=(x.re^2+x.im^2)^1/2.
    pub fn norm(&self) -> CalculatorFloat {
        ((self.re.clone() * &self.re) + (self.im.clone() * &self.im)).sqrt()
    }
    /// Return complex conjugate of x: x*=x.re-i*x.im.
    pub fn conj(&self) -> CalculatorComplex {
        Self {
            re: self.re.clone(),
            im: -self.im.clone(),
        }
    }
    /// Return true when x is close to y.
    pub fn isclose<T>(&self, other: T) -> bool
    where
        CalculatorComplex: From<T>,
    {
        let other_from = Self::from(other);
        self.re.isclose(other_from.re) && self.im.isclose(other_from.im)
    }
}

/// Implement `+` for CalculatorComplex and generic type `T`.
///
/// # Arguments
///
/// * `other` - Any type T for which CalculatorComplex::From<T> trait is implemented
///
impl<T> ops::Add<T> for CalculatorComplex
where
    CalculatorComplex: From<T>,
{
    type Output = Self;
    fn add(self, other: T) -> Self {
        let other_from = Self::from(other);
        CalculatorComplex {
            re: self.re + other_from.re,
            im: self.im + other_from.im,
        }
    }
}
/// Implement `+=` for CalculatorComplex and generic type `T`.
///
/// # Arguments
///
/// * `other` - Any type T for which CalculatorComplex::From<T> trait is implemented
///
impl<T> ops::AddAssign<T> for CalculatorComplex
where
    CalculatorComplex: From<T>,
{
    fn add_assign(&mut self, other: T) {
        let other_from = Self::from(other);
        *self = CalculatorComplex {
            re: &self.re + other_from.re,
            im: &self.im + other_from.im,
        }
    }
}

/// Implement `-` for CalculatorComplex and generic type `T`.
///
/// # Arguments
///
/// * `other` - Any type T for which CalculatorComplex::From<T> trait is implemented
///
impl<T> ops::Sub<T> for CalculatorComplex
where
    CalculatorComplex: From<T>,
{
    type Output = Self;
    fn sub(self, other: T) -> Self {
        let other_from = Self::from(other);
        CalculatorComplex {
            re: self.re - other_from.re,
            im: self.im - other_from.im,
        }
    }
}
/// Implement `-=` for CalculatorComplex and generic type `T`.
///
/// # Arguments
///
/// * `other` - Any type T for which CalculatorComplex::From<T> trait is implemented
///
impl<T> ops::SubAssign<T> for CalculatorComplex
where
    CalculatorComplex: From<T>,
{
    fn sub_assign(&mut self, other: T) {
        let other_from = Self::from(other);
        *self = CalculatorComplex {
            re: self.re.clone() - other_from.re,
            im: self.im.clone() - other_from.im,
        }
    }
}

/// Implement minus sign for CalculatorComplex.
impl ops::Neg for CalculatorComplex {
    type Output = CalculatorComplex;

    fn neg(self) -> Self {
        CalculatorComplex {
            re: -self.re,
            im: -self.im,
        }
    }
}

/// Implement `*` for CalculatorComplex and generic type `T`.
///
/// # Arguments
///
/// * `other` - Any type T for which CalculatorComplex::From<T> trait is implemented
///
impl<T> ops::Mul<T> for CalculatorComplex
where
    CalculatorComplex: From<T>,
{
    type Output = Self;
    fn mul(self, other: T) -> Self {
        let other_from = Self::from(other);
        CalculatorComplex {
            re: self.re.clone() * &other_from.re - (self.im.clone() * &other_from.im),
            im: self.re * &other_from.im + (self.im * &other_from.re),
        }
    }
}
/// Implement `*=` for CalculatorComplex and generic type `T`.
///
/// # Arguments
///
/// * `other` - Any type T for which CalculatorComplex::From<T> trait is implemented
///
impl<T> ops::MulAssign<T> for CalculatorComplex
where
    CalculatorComplex: From<T>,
{
    fn mul_assign(&mut self, other: T) {
        let other_from = Self::from(other);
        *self = CalculatorComplex {
            re: self.re.clone() * &other_from.re - (self.im.clone() * &other_from.im),
            im: self.re.clone() * &other_from.im + (self.im.clone() * &other_from.re),
        }
    }
}

/// Implement `*` for CalculatorComplex and generic type `T`.
///
/// # Arguments
///
/// * `other` - Any type T for which CalculatorComplex::From<T> trait is implemented
///
impl<T> ops::Div<T> for CalculatorComplex
where
    CalculatorComplex: From<T>,
{
    type Output = Self;
    fn div(self, other: T) -> Self {
        let other_from = Self::from(other);
        let norm = other_from.norm_sqr();
        CalculatorComplex {
            re: (self.re.clone() * &other_from.re + (self.im.clone() * &other_from.im)) / &norm,
            im: (-self.re * &other_from.im + (self.im * &other_from.re)) / &norm,
        }
    }
}
/// Implement `*=` for CalculatorComplex and generic type `T`.
///
/// # Arguments
///
/// * `other` - Any type T for which CalculatorComplex::From<T> trait is implemented
///
impl<T> ops::DivAssign<T> for CalculatorComplex
where
    CalculatorComplex: From<T>,
{
    fn div_assign(&mut self, other: T) {
        let other_from = Self::from(other);
        let norm = other_from.norm_sqr();
        *self = CalculatorComplex {
            re: (self.re.clone() * &other_from.re + (self.im.clone() * &other_from.im)) / &norm,
            im: (-self.re.clone() * &other_from.im + (self.im.clone() * &other_from.re)) / &norm,
        }
    }
}

/// Implement Inverse `1/x` for CalculatorFloat.
impl CalculatorComplex {
    pub fn recip(&self) -> CalculatorComplex {
        let norm = self.norm_sqr();
        CalculatorComplex {
            re: self.re.clone() / &norm,
            im: -self.im.clone() / &norm,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::CalculatorComplex;
    use super::CalculatorFloat;
    use num_complex::Complex;
    use std::convert::TryFrom;
    use std::ops::Neg;

    // Test the initialisation of CalculatorComplex from integer input
    #[test]
    fn from_int() {
        let x = CalculatorComplex::from(3);
        assert_eq!(x.re, CalculatorFloat::from(3));
        assert_eq!(x.im, CalculatorFloat::from(0));
        assert_eq!(
            x,
            CalculatorComplex {
                re: CalculatorFloat::from(3),
                im: CalculatorFloat::from(0)
            }
        );
        assert_eq!(f64::try_from(x).unwrap(), 3.0)
    }

    // Test the initialisation of CalculatorComplex from float input
    #[test]
    fn from_float() {
        let x = CalculatorComplex::from(3.1);
        assert_eq!(x.re, CalculatorFloat::from(3.1));
        assert_eq!(x.im, CalculatorFloat::from(0));
        assert_eq!(
            x,
            CalculatorComplex {
                re: CalculatorFloat::from(3.1),
                im: CalculatorFloat::from(0)
            }
        );
    }

    // Test the initialisation of CalculatorComplex from string input
    #[test]
    fn from_str() {
        let x = CalculatorComplex::from("3.1");
        assert_eq!(x.re, CalculatorFloat::from("3.1"));
        assert_eq!(x.im, CalculatorFloat::from(0));
        assert_eq!(
            x,
            CalculatorComplex {
                re: CalculatorFloat::from("3.1"),
                im: CalculatorFloat::from(0)
            }
        );
    }

    // Test the initialisation of CalculatorComplex from complex input
    #[test]
    fn from_complex() {
        let x = CalculatorComplex::from(Complex::new(1.0, 2.0));
        assert_eq!(x.re, CalculatorFloat::from(1.0));
        assert_eq!(x.im, CalculatorFloat::from(2.00));
        assert_eq!(
            x,
            CalculatorComplex {
                re: CalculatorFloat::from(1.0),
                im: CalculatorFloat::from(2.0)
            }
        );
    }

    // Test the initialisation of CalculatorComplex from Calculatorcomplex reference input
    #[test]
    fn from_calculator_complex() {
        let x = CalculatorComplex::new(1, 1);
        assert_eq!(CalculatorComplex::from(&x), x);
    }

    // Test the default function of CalculatorComplex
    #[test]
    fn default() {
        let x = CalculatorComplex::default();
        assert_eq!(x.re, CalculatorFloat::from(0.0));
        assert_eq!(x.im, CalculatorFloat::from(0.0));
        assert_eq!(x, CalculatorComplex::new(0, 0));
    }

    // Test the conversion of CalculatorComplex to Float
    #[test]
    fn try_from_float() {
        let x = CalculatorComplex::new(1.0, 0.0);
        assert_eq!(<f64>::try_from(x).unwrap(), 1.0);

        let x = CalculatorComplex::new(0.0, 1.0);
        assert!(f64::try_from(x).is_err());

        let x = CalculatorComplex::new("x", 0.0);
        assert!(f64::try_from(x).is_err());

        let x = CalculatorComplex::new(1.0, "x");
        assert!(f64::try_from(x).is_err());
    }

    // Test the conversion of CalculatorComplex to Complex
    #[test]
    fn try_from_complex() {
        let x = CalculatorComplex::new(1, 1);
        assert_eq!(Complex::<f64>::try_from(x).unwrap(), Complex::new(1.0, 1.0));

        let x = CalculatorComplex::new(0.0, "x");
        assert!(Complex::<f64>::try_from(x).is_err());

        let x = CalculatorComplex::new("x", 0.0);
        assert!(Complex::<f64>::try_from(x).is_err());
    }

    // Test the Display trait of CalculatorComplex
    #[test]
    fn display() {
        let x = CalculatorComplex::new(-3, 2);
        let x_formatted = format!("{}", x);
        assert_eq!(x_formatted, "(-3e0 + i * 2e0)");
    }
    
    // Test the addition functionality of CalculatorComplex
    #[test]
    fn try_add() {
        let x = CalculatorComplex::new(1, 1);
        let y = CalculatorComplex::new(2, "test");
        assert_eq!(x + y, CalculatorComplex::new(3.0, "(1e0 + test)"));
    }

    // Test the add_assign functionality of CalculatorComplex
    #[test]
    fn try_iadd() {
        let mut x = CalculatorComplex::new(1, 1);
        let y = CalculatorComplex::new(2, "test");
        x += y;
        assert_eq!(x, CalculatorComplex::new(3.0, "(1e0 + test)"));
    }

    // Test the subtract functionality of CalculatorComplex
    #[test]
    fn try_sub() {
        let x = CalculatorComplex::new(1, 1);
        let y = CalculatorComplex::new(2, "test");
        assert_eq!(x - y, CalculatorComplex::new(-1.0, "(1e0 - test)"));
    }

    // Test the sub_assign functionality of CalculatorComplex
    #[test]
    fn try_isub() {
        let mut x = CalculatorComplex::new(1, 1);
        let y = CalculatorComplex::new(2, "test");
        x -= y;
        assert_eq!(x, CalculatorComplex::new(-1.0, "(1e0 - test)"));
    }

    // Test the multiply functionality of CalculatorComplex
    #[test]
    fn try_mul() {
        let x = CalculatorComplex::new(1, 1);
        let y = CalculatorComplex::new(2, 2);
        assert_eq!(x * y, CalculatorComplex::new(0.0, 4.0));
    }

    // Test the mul_assign functionality of CalculatorComplex
    #[test]
    fn try_imul() {
        let mut x = CalculatorComplex::new(1, 1);
        let y = CalculatorComplex::new(2, 2);
        x *= y;
        assert_eq!(x, CalculatorComplex::new(0.0, 4.0));
    }

    // Test the division functionality of CalculatorComplex
    #[test]
    fn try_div() {
        let x = CalculatorComplex::new(1, 1);
        let y = CalculatorComplex::new(3, 4);
        assert_eq!(x / y, CalculatorComplex::new(7.0 / 25.0, -1.0 / 25.0));
    }

    // Test the div_assign functionality of CalculatorComplex
    #[test]
    fn try_idiv() {
        let mut x = CalculatorComplex::new(1, 1);
        let y = CalculatorComplex::new(3, 4);
        x /= y;
        assert_eq!(x, CalculatorComplex::new(7.0 / 25.0, -1.0 / 25.0));
    }

    // Test the arg(x) functionality of CalculatorComplex with all possible input types
    #[test]
    fn arg() {
        let x = CalculatorComplex::new(1, 2);
        let y = Complex::new(1.0, 2.0);
        assert_eq!(x.arg(), CalculatorFloat::from(y.arg()));

        let x = CalculatorComplex::new("x", 2);
        assert_eq!(x.arg(), CalculatorFloat::from("atan2(2e0, x)"));

        let x = CalculatorComplex::new(1, "2x");
        assert_eq!(x.arg(), CalculatorFloat::from("atan2(2x, 1e0)"));

        let x = CalculatorComplex::new("x", "2t");
        assert_eq!(x.arg(), CalculatorFloat::from("atan2(2t, x)"));
    }

    // Test the square norm functionality of CalculatorComplex
    #[test]
    fn norm_sqr() {
        let x = CalculatorComplex::new(1, 2);
        let y = Complex::new(1.0, 2.0);
        assert_eq!(x.norm_sqr(), CalculatorFloat::from(y.norm_sqr()));
    }

    // Test the norm functionality of CalculatorComplex
    #[test]
    fn norm() {
        let x = CalculatorComplex::new(1, 2);
        let y = Complex::new(1.0, 2.0);
        assert_eq!(x.norm(), CalculatorFloat::from(y.norm()));
    }

    // Test the conjugate functionality of CalculatorComplex
    #[test]
    fn conj() {
        let x = CalculatorComplex::new(1, 2);
        let y = Complex::new(1.0, 2.0);
        assert_eq!(x.conj(), CalculatorComplex::new(y.conj().re, y.conj().im));
    }

    // Test the isclose functionality of CalculatorComplex
    #[test]
    fn is_close() {
        let x = CalculatorComplex::new(1, 2);
        let y = Complex::new(1.0, 2.0);
        assert!(x.isclose(y));

        let y = 1.0;
        assert!(!x.isclose(y));
    }

    // // Test the negative sign (*-1) functionality of CalculatorComplex
    #[test]
    fn neg() {
        let x = CalculatorComplex::new(1, 2);
        assert_eq!(x.neg(), CalculatorComplex::new(-1, -2));
    }

    // Test the inverse functionality of CalculatorComplex
    #[test]
    fn inv() {
        let x = CalculatorComplex::new(3, 4);
        assert_eq!(x.recip(), CalculatorComplex::new(0.12, -0.16));
    }

    // Test the Debug trait for CalculatorComplex
    #[test]
    fn debug() {
        let x = CalculatorComplex::from(3.0);
        assert_eq!(format!("{:?}", x), "CalculatorComplex { re: Float(3.0), im: Float(0.0) }");

        let xs = CalculatorComplex::from("3x");
        assert_eq!(format!("{:?}", xs), "CalculatorComplex { re: Str(\"3x\"), im: Float(0.0) }");
    }

    // Test the Clone trait for CalculatorComplex
    #[test]
    fn clone_trait() {
        let x = CalculatorComplex::from(3.0);
        assert_eq!(x.clone(), x);

        let xs = CalculatorComplex::from("3x");
        assert_eq!(xs.clone(), xs);
    }

    // Test the PartialEq trait for CalculatorComplex
    #[test]
    fn partial_eq() {
        let x1 = CalculatorComplex::from(3.0);
        let x2 = CalculatorComplex::from(3.0);
        assert!(x1 == x2);
        assert!(x2 == x1);

        let x1s = CalculatorComplex::from("3x");
        let x2s = CalculatorComplex::from("3x");
        assert!(x1s == x2s);
        assert!(x2s == x1s);
    }
    
}
