// Copyright © 2020-2025 HQS Quantum Simulations GmbH. All Rights Reserved.
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
//! Provides CalculatorFloat enum and methods for parsing and evaluating
//! mathematical expressions in string form to float.

use crate::calculator::{Token, TokenIterator};
use crate::CalculatorError;
#[cfg(feature = "json_schema")]
use schemars::schema::*;
use serde::de::{Deserializer, Error, Visitor};
use serde::ser::Serializer;
use serde::{Deserialize, Serialize};
use std::convert::TryFrom;
use std::fmt;
use std::ops;
use std::str::FromStr;

static ATOL: f64 = f64::EPSILON;
static RTOL: f64 = 1e-8;

/// CalculatorFloat is an enum combining Float and String.
///
/// # Variants
///
/// * `Float` - f64 value
/// * `Str` - String instance
///
#[derive(Debug, Clone, PartialEq)]
// #[cfg_attr(feature = "json_schema", derive(schemars::JsonSchema))]
pub enum CalculatorFloat {
    /// Floating point value
    Float(f64),
    /// Symbolic expression in String form
    Str(String),
}

#[cfg(feature = "json_schema")]
impl schemars::JsonSchema for CalculatorFloat {
    fn schema_name() -> String {
        "CalculatorFloat".to_string()
    }

    fn json_schema(gen: &mut schemars::gen::SchemaGenerator) -> Schema {
        let mut return_schema = SchemaObject::default();
        return_schema.subschemas().one_of =
            Some(vec![<f64>::json_schema(gen), <String>::json_schema(gen)]);
        return_schema.into()
    }
}

/// Implement Default value 0 for CalculatorFloat.
impl Default for CalculatorFloat {
    fn default() -> Self {
        CalculatorFloat::Float(0.0)
    }
}

// Implementing serde serialization
// writing directly to string or f64.
impl Serialize for CalculatorFloat {
    // Serialization function for CalculatorFloat according to float or string type.
    //
    // # Arguments
    //
    // * `self` - CalculatorFloat to be serialized
    // * `serializer` - Serializer used for serialization
    //
    // # Returns
    //
    // `S::Ok` - Serialized instance of CalculatorFloat
    // `S::Error` - Error in the serialization process
    //
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let human_readable = serializer.is_human_readable();
        if human_readable {
            match self {
                CalculatorFloat::Float(x) => serializer.serialize_f64(*x),
                CalculatorFloat::Str(x) => serializer.serialize_str(x),
            }
        } else {
            match self {
                CalculatorFloat::Float(x) => {
                    serializer.serialize_newtype_variant("CalculatorFloat", 0, "Float", x)
                }
                CalculatorFloat::Str(x) => {
                    serializer.serialize_newtype_variant("CalculatorFloat", 1, "Str", x)
                }
            }
        }
    }
}

// Deserializing directly from string or f64.
impl<'de> Deserialize<'de> for CalculatorFloat {
    // Deserialization function for CalculatorFloat.
    //
    // # Arguments
    //
    // * `self` - Serialized instance of CalculatorFloat to be deserialized
    // * `deserializer` - Deserializer used for deserialization
    //
    // # Returns
    //
    // `CalculatorFloat` - Deserialized instance of CalculatorFloat
    // `D::Error` - Error in the deserialization process
    //
    fn deserialize<D>(deserializer: D) -> Result<CalculatorFloat, D::Error>
    where
        D: Deserializer<'de>,
    {
        let human_readable = deserializer.is_human_readable();
        if human_readable {
            struct TemporaryVisitor;
            impl Visitor<'_> for TemporaryVisitor {
                type Value = CalculatorFloat;

                // Visit expectation for CalculatorFloatVisitor.
                //
                // # Arguments
                //
                // * `self` - Error
                // * `formatter` - Configuration for formatting
                //
                // # Returns
                //
                // `str` - What TemporaryVisitor should expect
                //
                fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                    formatter.write_str("float or string")
                }

                // Visit function for string value.
                //
                // # Arguments
                //
                // * `self` - Error
                // * `value` - value to be deserialized
                //
                // # Returns
                //
                // `Result<CalculatorFloat, E>` - CalculatorFloat of value or corresponding error
                //
                fn visit_str<E>(self, value: &str) -> Result<CalculatorFloat, E>
                where
                    E: Error,
                {
                    Ok(CalculatorFloat::from(value))
                }

                // Visit function for f64 value.
                //
                // # Arguments
                //
                // * `self` - Error
                // * `value` - value to be deserialized
                //
                // # Returns
                //
                // `Result<CalculatorFloat, E>` - CalculatorFloat of value or corresponding error
                //
                fn visit_f64<E>(self, value: f64) -> Result<CalculatorFloat, E>
                where
                    E: Error,
                {
                    Ok(CalculatorFloat::from(value))
                }

                // Visit function for u64 value.
                //
                // # Arguments
                //
                // * `self` - Error
                // * `value` - value to be deserialized
                //
                // # Returns
                //
                // `Result<CalculatorFloat, E>` - CalculatorFloat of value or corresponding error
                //
                fn visit_u64<E>(self, value: u64) -> Result<CalculatorFloat, E>
                where
                    E: Error,
                {
                    Ok(CalculatorFloat::from(value))
                }

                // Visit function for i32 value.
                //
                // # Arguments
                //
                // * `self` - Error
                // * `value` - value to be deserialized
                //
                // # Returns
                //
                // `Result<CalculatorFloat, E>` - CalculatorFloat of value or corresponding error
                //
                fn visit_i32<E>(self, value: i32) -> Result<CalculatorFloat, E>
                where
                    E: Error,
                {
                    Ok(CalculatorFloat::from(value))
                }

                // Visit function for i64 value.
                //
                // # Arguments
                //
                // * `self` - Error
                // * `value` - value to be deserialized
                //
                // # Returns
                //
                // `Result<CalculatorFloat, E>` - CalculatorFloat of value or corresponding error
                //
                fn visit_i64<E>(self, value: i64) -> Result<CalculatorFloat, E>
                where
                    E: Error,
                {
                    Ok(CalculatorFloat::from(value))
                }

                // Visit function for u32 value.
                //
                // # Arguments
                //
                // * `self` - Error
                // * `value` - value to be deserialized
                //
                // # Returns
                //
                // `Result<CalculatorFloat, E>` - CalculatorFloat of value or corresponding error
                //
                fn visit_u32<E>(self, value: u32) -> Result<CalculatorFloat, E>
                where
                    E: Error,
                {
                    Ok(CalculatorFloat::from(value))
                }
            }

            deserializer.deserialize_any(TemporaryVisitor)
        } else {
            // Marker struct for the Variants of CalculatorFlot
            enum Variant {
                Float,
                Str,
            }
            // Visitor extracting the Variant of the serialized CalculatorFloat enum
            struct VariantVisitor;
            impl serde::de::Visitor<'_> for VariantVisitor {
                type Value = Variant;
                fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                    fmt::Formatter::write_str(formatter, "Identifier of CalculatorFloat variant")
                }
                // when variants are marked by u64 values
                fn visit_u64<E>(self, value: u64) -> Result<Self::Value, E>
                where
                    E: Error,
                {
                    match value {
                        0u64 => Ok(Variant::Float),
                        1u64 => Ok(Variant::Str),
                        _ => Err(Error::invalid_value(
                            serde::de::Unexpected::Unsigned(value),
                            &"CalculatorFloat has two variants, expecting field identifier 0 or 1",
                        )),
                    }
                }
                // when variants are marked by String values
                fn visit_str<E>(self, value: &str) -> Result<Self::Value, E>
                where
                    E: Error,
                {
                    match value {
                        "Float" => Ok(Variant::Float),
                        "Str" => Ok(Variant::Str),
                        _ => Err(Error::unknown_variant(value, VARIANTS)),
                    }
                }
                // when variants are marked by Strings as byte fields
                fn visit_bytes<E>(self, value: &[u8]) -> Result<Self::Value, E>
                where
                    E: Error,
                {
                    match value {
                        b"Float" => Ok(Variant::Float),
                        b"Str" => Ok(Variant::Str),
                        _ => {
                            let unknown_variant_string =
                                &std::string::String::from_utf8_lossy(value);
                            Err(Error::unknown_variant(unknown_variant_string, VARIANTS))
                        }
                    }
                }
            }
            impl<'de> serde::Deserialize<'de> for Variant {
                #[inline]
                fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
                where
                    D: serde::Deserializer<'de>,
                {
                    serde::Deserializer::deserialize_identifier(deserializer, VariantVisitor)
                }
            }
            struct Visitor {}
            impl<'de> serde::de::Visitor<'de> for Visitor {
                type Value = CalculatorFloat;
                fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                    fmt::Formatter::write_str(formatter, "enum CalculatorFloat")
                }
                fn visit_enum<A>(self, data: A) -> Result<Self::Value, A::Error>
                where
                    A: serde::de::EnumAccess<'de>,
                {
                    match match serde::de::EnumAccess::variant(data) {
                        Ok(extracted_data) => extracted_data,
                        Err(error) => {
                            return Err(error);
                        }
                    } {
                        (Variant::Float, variant) => Result::map(
                            serde::de::VariantAccess::newtype_variant::<f64>(variant),
                            CalculatorFloat::Float,
                        ),
                        (Variant::Str, variant) => Result::map(
                            serde::de::VariantAccess::newtype_variant::<String>(variant),
                            CalculatorFloat::Str,
                        ),
                    }
                }
            }
            const VARIANTS: &[&str] = &["Float", "Str"];
            serde::Deserializer::deserialize_enum(
                deserializer,
                "CalculatorFloat",
                VARIANTS,
                Visitor {},
            )
        }
    }
}

/// Initialize CalculatorFloat from i32 value.
///
/// # Returns
///
/// * `CalculatorFloat::Float`
///
impl From<i32> for CalculatorFloat {
    fn from(item: i32) -> Self {
        CalculatorFloat::Float(item as f64)
    }
}

/// Initialize CalculatorFloat from i64 value.
///
/// # Returns
///
/// * `CalculatorFloat::Float`
///
impl From<i64> for CalculatorFloat {
    fn from(item: i64) -> Self {
        CalculatorFloat::Float(item as f64)
    }
}

/// Initialize CalculatorFloat from usize value.
///
/// # Returns
///
/// * `CalculatorFloat::Float`
///
impl From<u32> for CalculatorFloat {
    fn from(item: u32) -> Self {
        CalculatorFloat::Float(item as f64)
    }
}

/// Initialize CalculatorFloat from u64 value.
///
/// # Returns
///
/// * `CalculatorFloat::Float`
///
impl From<u64> for CalculatorFloat {
    fn from(item: u64) -> Self {
        CalculatorFloat::Float(item as f64)
    }
}

/// Initialize CalculatorFloat from i32 reference &.
///
/// # Returns
///
/// * `CalculatorFloat::Float`
///
impl<'a> From<&'a i32> for CalculatorFloat {
    fn from(item: &'a i32) -> Self {
        CalculatorFloat::Float(*item as f64)
    }
}

/// Initialize CalculatorFloat from i64 reference &.
///
/// # Returns
///
/// * `CalculatorFloat::Float`
///
impl<'a> From<&'a i64> for CalculatorFloat {
    fn from(item: &'a i64) -> Self {
        CalculatorFloat::Float(*item as f64)
    }
}

/// Initialize CalculatorFloat from u43 reference &.
///
/// # Returns
///
/// * `CalculatorFloat::Float`
///
impl<'a> From<&'a u32> for CalculatorFloat {
    fn from(item: &'a u32) -> Self {
        CalculatorFloat::Float(*item as f64)
    }
}

/// Initialize CalculatorFloat from u64 reference &.
///
/// # Returns
///
/// * `CalculatorFloat::Float`
///
impl<'a> From<&'a u64> for CalculatorFloat {
    fn from(item: &'a u64) -> Self {
        CalculatorFloat::Float(*item as f64)
    }
}

/// Initialize CalculatorFloat from f64 value.
///
/// # Returns
///
/// * `CalculatorFloat::Float`
///
impl From<f64> for CalculatorFloat {
    fn from(item: f64) -> Self {
        CalculatorFloat::Float(item)
    }
}

/// Initialize CalculatorFloat from f64 reference &.
///
/// # Returns
///
/// * `CalculatorFloat::Float`
///
impl<'a> From<&'a f64> for CalculatorFloat {
    fn from(item: &'a f64) -> Self {
        CalculatorFloat::Float(*item)
    }
}

/// Initialize CalculatorFloat from string value.
///
/// # Returns
///
/// * `CalculatorFloat::Str`
///
impl From<String> for CalculatorFloat {
    fn from(item: String) -> Self {
        let f = f64::from_str(item.as_str());
        match f {
            Err(_) => CalculatorFloat::Str(item),
            Ok(x) => CalculatorFloat::Float(x),
        }
    }
}

/// Initialize CalculatorFloat from string reference &String.
///
/// # Returns
///
/// * `CalculatorFloat::Float`
///
impl From<&String> for CalculatorFloat {
    fn from(item: &String) -> Self {
        let f = f64::from_str(item.as_str());
        match f {
            Err(_) => CalculatorFloat::Str(item.clone()),
            Ok(x) => CalculatorFloat::Float(x),
        }
    }
}

/// Initialize CalculatorFloat from str reference &str.
///
/// # Returns
///
/// * `CalculatorFloat::Float`
///
impl From<&str> for CalculatorFloat {
    fn from(item: &str) -> Self {
        let f = f64::from_str(item);
        match f {
            Err(_) => CalculatorFloat::Str(String::from(item)),
            Ok(x) => CalculatorFloat::Float(x),
        }
    }
}

impl FromStr for CalculatorFloat {
    type Err = CalculatorError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let f = f64::from_str(s);
        match f {
            Err(_) => {
                let mut tokeniter = TokenIterator {
                    current_expression: s,
                };
                match tokeniter.find(|t| {
                    matches!(
                        t,
                        Token::VariableAssign(_) | Token::Assign | Token::Unrecognized
                    )
                }) {
                    None => Ok(CalculatorFloat::Str(s.to_string())),
                    Some(t) => match t {
                        Token::VariableAssign(vs) => {
                            Err(CalculatorError::NotParsableAssign { variable_name: vs })
                        }
                        Token::Assign => Err(CalculatorError::NotParsableSingleAssign),
                        Token::Unrecognized => Err(CalculatorError::NotParsableUnrecognized),
                        _ => panic!(""),
                    },
                }
            }
            Ok(x) => Ok(CalculatorFloat::Float(x)),
        }
    }
}

/// Try turning CalculatorFloat into f64 float.
///
/// # Returns
///
/// * `f64`
///
/// # Panics
///
/// Panics when CalculatorFloat contains symbolic string value
///
impl TryFrom<CalculatorFloat> for f64 {
    type Error = CalculatorError;

    fn try_from(value: CalculatorFloat) -> Result<Self, Self::Error> {
        match value {
            CalculatorFloat::Float(x) => Ok(x),
            CalculatorFloat::Str(x) => Err(CalculatorError::FloatSymbolicNotConvertable { val: x }),
        }
    }
}

/// Return CalculatorFloat as String.
///
/// # Returns
///
/// * `String`
///
impl From<CalculatorFloat> for String {
    fn from(value: CalculatorFloat) -> Self {
        format!("{value}")
    }
}

/// Initialize CalculatorFloat from CalculatorFloat reference &CalculatorFloat.
///
/// # Returns
///
/// * `CalculatorFloat`
///
impl<'a> From<&'a CalculatorFloat> for CalculatorFloat {
    fn from(item: &'a CalculatorFloat) -> Self {
        (*item).clone()
    }
}

/// Implement Display trait for CalculatorFloat.
///
/// Allows use of simple text formating
///
impl fmt::Display for CalculatorFloat {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            CalculatorFloat::Float(x) => write!(f, "{x:e}"),
            CalculatorFloat::Str(y) => write!(f, "{y}"),
        }
    }
}

impl CalculatorFloat {
    /// Constant zero for CalculatorFloat
    pub const ZERO: CalculatorFloat = CalculatorFloat::Float(0.0);

    /// Constant one for CalculatorFloat
    pub const ONE: CalculatorFloat = CalculatorFloat::Float(1.0);

    /// Constant pi for CalculatorFloat
    pub const PI: CalculatorFloat = CalculatorFloat::Float(std::f64::consts::PI);

    /// Constant Euler's number e for CalculatorFloat
    pub const E: CalculatorFloat = CalculatorFloat::Float(std::f64::consts::E);

    /// Constant 1/sqrt(2) e for CalculatorFloat
    pub const FRAC_1_SQRT_2: CalculatorFloat =
        CalculatorFloat::Float(std::f64::consts::FRAC_1_SQRT_2);

    /// Constant pi / 2 e for CalculatorFloat
    pub const FRAC_PI_2: CalculatorFloat = CalculatorFloat::Float(std::f64::consts::FRAC_PI_2);

    /// Constant pi / 4 e for CalculatorFloat
    pub const FRAC_PI_4: CalculatorFloat = CalculatorFloat::Float(std::f64::consts::FRAC_PI_4);

    /// Constant sqrt(2) e for CalculatorFloat
    pub const SQRT_2: CalculatorFloat = CalculatorFloat::Float(std::f64::consts::SQRT_2);

    /// Return True when CalculatorFloat does not contain symbolic expression.
    pub fn is_float(&self) -> bool {
        match self {
            CalculatorFloat::Float(_) => true,
            CalculatorFloat::Str(_) => false,
        }
    }
    /// Return square root of CalculatorFloat.
    pub fn sqrt(&self) -> CalculatorFloat {
        match self {
            CalculatorFloat::Float(f) => CalculatorFloat::Float(f.sqrt()),
            CalculatorFloat::Str(s) => CalculatorFloat::Str(format!("sqrt({s})")),
        }
    }
    /// Return atan2 for CalculatorFloat and generic type `T`.
    ///
    /// # Arguments
    ///
    /// * `other` - Any type T for which CalculatorFloat::From<T> trait is implemented
    ///
    pub fn atan2<T>(&self, other: T) -> CalculatorFloat
    where
        T: Into<CalculatorFloat>,
    {
        let other_from: CalculatorFloat = other.into();
        match self {
            Self::Float(x) => match other_from {
                Self::Float(y) => CalculatorFloat::Float(x.atan2(y)),
                Self::Str(y) => Self::Str(format!("atan2({:e}, {})", x, &y)),
            },
            Self::Str(x) => match other_from {
                Self::Float(y) => Self::Str(format!("atan2({x}, {y:e})")),
                Self::Str(y) => Self::Str(format!("atan2({}, {})", x, &y)),
            },
        }
    }

    /// Return Power for CalculatorFloat and generic type `T`.
    ///
    /// # Arguments
    ///
    /// * `other` - Any type T for which CalculatorFloat::From<T> trait is implemented
    ///
    pub fn powf<T>(&self, other: T) -> CalculatorFloat
    where
        T: Into<CalculatorFloat>,
    {
        let other_from: CalculatorFloat = other.into();
        match self {
            Self::Float(x) => match other_from {
                Self::Float(y) => CalculatorFloat::Float(x.powf(y)),
                Self::Str(y) => Self::Str(format!("({:e} ^ {})", x, &y)),
            },
            Self::Str(x) => match other_from {
                Self::Float(y) => Self::Str(format!("({x} ^ {y:e})")),
                Self::Str(y) => Self::Str(format!("({} ^ {})", x, &y)),
            },
        }
    }

    /// Return exponential function exp(x) for CalculatorFloat.
    pub fn exp(&self) -> CalculatorFloat {
        match self {
            Self::Float(x) => CalculatorFloat::Float(x.exp()),
            Self::Str(y) => Self::Str(format!("exp({y})")),
        }
    }
    /// Return sine function sin(x) for CalculatorFloat.
    pub fn sin(&self) -> CalculatorFloat {
        match self {
            Self::Float(x) => CalculatorFloat::Float(x.sin()),
            Self::Str(y) => Self::Str(format!("sin({y})")),
        }
    }
    /// Return cosine function cos(x) for CalculatorFloat.
    pub fn cos(&self) -> CalculatorFloat {
        match self {
            Self::Float(x) => CalculatorFloat::Float(x.cos()),
            Self::Str(y) => Self::Str(format!("cos({y})")),
        }
    }
    /// Return arccosine function acos(x) for CalculatorFloat.
    pub fn acos(&self) -> CalculatorFloat {
        match self {
            Self::Float(x) => CalculatorFloat::Float(x.acos()),
            Self::Str(y) => Self::Str(format!("acos({y})")),
        }
    }
    /// Return absolute value abs(x) for CalculatorFloat.
    pub fn abs(&self) -> CalculatorFloat {
        match self {
            Self::Float(x) => CalculatorFloat::Float(x.abs()),
            Self::Str(y) => Self::Str(format!("abs({y})")),
        }
    }
    /// Return signum value sign(x) for CalculatorFloat.
    pub fn signum(&self) -> CalculatorFloat {
        match self {
            Self::Float(x) => CalculatorFloat::Float(x.signum()),
            Self::Str(y) => Self::Str(format!("sign({y})")),
        }
    }
    /// Return True if self value is close to other value.
    pub fn isclose<T>(&self, other: T) -> bool
    where
        T: Into<CalculatorFloat>,
    {
        let other_from: CalculatorFloat = other.into();
        match self {
            Self::Float(x) => match other_from {
                Self::Float(y) => (x - y).abs() <= (ATOL + RTOL * y.abs()),
                Self::Str(y) => format!("{x:e}") == y,
            },
            Self::Str(x) => match other_from {
                Self::Float(y) => x == &format!("{y:e}"),
                Self::Str(y) => x == &y,
            },
        }
    }

    /// Return Some(f64) when CalculatorFloat is a numeric value
    pub fn float(&self) -> Result<&f64, CalculatorError> {
        match self {
            Self::Float(x) => Ok(x),
            Self::Str(x) => Err(CalculatorError::FloatSymbolicNotConvertable { val: x.clone() }),
        }
    }

    /// Return inverse/reciprocal function (1/x) for CalculatorFloat.
    pub fn recip(&self) -> CalculatorFloat {
        match self {
            Self::Float(x) => Self::Float(x.recip()),
            Self::Str(y) => Self::Str(format!("(1 / {y})")),
        }
    }
}
/// Implement `+` (add) for CalculatorFloat and generic type `T`.
///
/// # Arguments
///
/// * `other` - Any type T for which CalculatorFloat::From<T> trait is implemented
///
impl<T> ops::Add<T> for CalculatorFloat
where
    T: Into<CalculatorFloat>,
{
    type Output = Self;
    fn add(self, other: T) -> Self {
        let other_from: CalculatorFloat = other.into();
        match self {
            Self::Float(x) => match other_from {
                Self::Float(y) => CalculatorFloat::Float(x + y),
                Self::Str(y) => {
                    if x != 0.0 {
                        Self::Str(format!("({:e} + {})", x, &y))
                    } else {
                        Self::Str(y)
                    }
                }
            },
            Self::Str(x) => match other_from {
                Self::Float(y) => {
                    if y != 0.0 {
                        Self::Str(format!("({} + {:e})", &x, y))
                    } else {
                        Self::Str(x)
                    }
                }
                Self::Str(y) => Self::Str(format!("({} + {})", &x, &y)),
            },
        }
    }
}

/// Implements summing over an iterator of CalculatorFloat
///
/// # Arguments
///
/// * `iter` - Any iterator over CalculatorFloat items
///
impl std::iter::Sum for CalculatorFloat {
    fn sum<I: Iterator<Item = CalculatorFloat>>(iter: I) -> Self {
        let mut sum = CalculatorFloat::from(0);
        for i in iter {
            sum += i;
        }
        sum
    }
}

/// Implement `+=` (add) for CalculatorFloat and generic type `T`.
///
/// # Arguments
///
/// * `other` - Any type T for which CalculatorFloat::From<T> trait is implemented
///
impl<T> ops::AddAssign<T> for CalculatorFloat
where
    T: Into<CalculatorFloat>,
{
    fn add_assign(&mut self, other: T) {
        let other_from: CalculatorFloat = other.into();

        match self {
            Self::Float(x) => match other_from {
                Self::Float(y) => {
                    *self = Self::Float(*x + y);
                }
                Self::Str(y) => {
                    *self = {
                        if (*x - 0.0).abs() > ATOL {
                            Self::Str(format!("({:e} + {})", x, &y))
                        } else {
                            Self::Str(y)
                        }
                    }
                }
            },
            Self::Str(x) => match other_from {
                Self::Float(y) => {
                    *self = {
                        if y != 0.0 {
                            Self::Str(format!("({x} + {y:e})"))
                        } else {
                            Self::Str(x.to_owned())
                        }
                    }
                }
                Self::Str(y) => *self = Self::Str(format!("({} + {})", x, &y)),
            },
        }
    }
}

/// Implement `+` (add) for &CalculatorFloat and generic type `T`.
///
/// # Arguments
///
/// * `other` - Any type T for which CalculatorFloat::From<T> trait is implemented
///
impl<T> ops::Add<T> for &CalculatorFloat
where
    CalculatorFloat: From<T>,
{
    type Output = CalculatorFloat;
    fn add(self, other: T) -> CalculatorFloat {
        let other_from = CalculatorFloat::from(other);
        match self {
            CalculatorFloat::Float(x) => match other_from {
                CalculatorFloat::Float(y) => CalculatorFloat::Float(x + y),
                CalculatorFloat::Str(y) => {
                    if (x - 0.0).abs() > ATOL {
                        CalculatorFloat::Str(format!("({:e} + {})", x, &y))
                    } else {
                        CalculatorFloat::Str(y)
                    }
                }
            },
            CalculatorFloat::Str(x) => match other_from {
                CalculatorFloat::Float(y) => {
                    if y != 0.0 {
                        CalculatorFloat::Str(format!("({x} + {y:e})"))
                    } else {
                        CalculatorFloat::Str(x.to_owned())
                    }
                }
                CalculatorFloat::Str(y) => CalculatorFloat::Str(format!("({} + {})", x, &y)),
            },
        }
    }
}

/// Implement `/` (divide) for CalculatorFloat and generic type `T`.
///
/// # Arguments
///
/// * `other` - Any type T for which CalculatorFloat::From<T> trait is implemented
///
/// # Panics
///
/// Panics on division by zero.
/// Division by zero is only detected when other is converted to CalculatorFloat::Float
///
impl<T> ops::Div<T> for CalculatorFloat
where
    T: Into<CalculatorFloat>,
{
    type Output = Self;
    fn div(self, other: T) -> Self {
        let other_from: CalculatorFloat = other.into();
        match self {
            Self::Float(x) => match other_from {
                Self::Float(y) => {
                    if y == 0.0 {
                        panic!("Division by zero")
                    } else {
                        Self::Float(x / y)
                    }
                }
                Self::Str(y) => {
                    if x == 0.0 {
                        Self::Float(0.0)
                    } else {
                        Self::Str(format!("({:e} / {})", x, &y))
                    }
                }
            },
            Self::Str(x) => match other_from {
                Self::Float(y) => {
                    if y == 0.0 {
                        panic!("Division by zero")
                    } else if (y - 1.0).abs() < ATOL {
                        Self::Str(x)
                    } else {
                        Self::Str(format!("({} / {:e})", &x, y))
                    }
                }
                Self::Str(y) => Self::Str(format!("({} / {})", &x, &y)),
            },
        }
    }
}

/// Implement `/=` (divide) for CalculatorFloat and generic type `T`.
///
/// # Arguments
///
/// * `other` - Any type T for which CalculatorFloat::From<T> trait is implemented
///
/// # Panics
///
/// Panics on division by zero.
/// Division by zero is only detected when other is converted to CalculatorFloat::Float
///
impl<T> ops::DivAssign<T> for CalculatorFloat
where
    T: Into<CalculatorFloat>,
{
    fn div_assign(&mut self, other: T) {
        let other_from: CalculatorFloat = other.into();
        match self {
            Self::Float(x) => match other_from {
                Self::Float(y) => {
                    *self = {
                        if y == 0.0 {
                            panic!("Division by zero")
                        } else {
                            Self::Float(*x / y)
                        }
                    }
                }
                Self::Str(y) => {
                    *self = {
                        if (*x - 0.0).abs() < ATOL {
                            Self::Float(0.0)
                        } else {
                            Self::Str(format!("({:e} / {})", x, &y))
                        }
                    }
                }
            },
            Self::Str(x) => match other_from {
                Self::Float(y) => {
                    *self = {
                        if y == 0.0 {
                            panic!("Division by zero")
                        } else if (y - 1.0).abs() < ATOL {
                            Self::Str(x.to_owned())
                        } else {
                            Self::Str(format!("({x} / {y:e})"))
                        }
                    }
                }
                Self::Str(y) => *self = Self::Str(format!("({} / {})", x, &y)),
            },
        }
    }
}

/// Implement `*` (multiply) for CalculatorFloat and generic type `T`.
///
/// # Arguments
///
/// * `other` - Any type T for which CalculatorFloat::From<T> trait is implemented
///
impl<T> ops::Mul<T> for CalculatorFloat
where
    T: Into<CalculatorFloat>,
{
    type Output = Self;
    fn mul(self, other: T) -> Self {
        let other_from: CalculatorFloat = other.into();
        match self {
            Self::Float(x) => match other_from {
                Self::Float(y) => Self::Float(x * y),
                Self::Str(y) => {
                    if x == 0.0 {
                        Self::Float(0.0)
                    } else if (x - 1.0).abs() < ATOL {
                        Self::Str(y)
                    } else {
                        Self::Str(format!("({:e} * {})", x, &y))
                    }
                }
            },
            Self::Str(x) => match other_from {
                Self::Float(y) => {
                    if y == 0.0 {
                        Self::Float(0.0)
                    } else if (y - 1.0).abs() < ATOL {
                        Self::Str(x)
                    } else {
                        Self::Str(format!("({} * {:e})", &x, y))
                    }
                }
                Self::Str(y) => Self::Str(format!("({x} * {y})")),
            },
        }
    }
}

/// Implement `*` (multiply) for CalculatorFloat and generic type `T`.
///
/// # Arguments
///
/// * `other` - Any type T for which CalculatorFloat::From<T> trait is implemented
///
impl<T> ops::Mul<T> for &CalculatorFloat
where
    T: Into<CalculatorFloat>,
{
    type Output = CalculatorFloat;
    fn mul(self, other: T) -> CalculatorFloat {
        let other_from: CalculatorFloat = other.into();
        match self {
            CalculatorFloat::Float(x) => match other_from {
                CalculatorFloat::Float(y) => CalculatorFloat::Float(x * y),
                CalculatorFloat::Str(y) => {
                    if *x == 0.0 {
                        CalculatorFloat::Float(0.0)
                    } else if (x - 1.0).abs() < ATOL {
                        CalculatorFloat::Str(y)
                    } else {
                        CalculatorFloat::Str(format!("({:e} * {})", x, &y))
                    }
                }
            },
            CalculatorFloat::Str(x) => match other_from {
                CalculatorFloat::Float(y) => {
                    if y == 0.0 {
                        CalculatorFloat::Float(0.0)
                    } else if (y - 1.0).abs() < ATOL {
                        CalculatorFloat::Str(x.to_string())
                    } else {
                        CalculatorFloat::Str(format!("({} * {:e})", &x, y))
                    }
                }
                CalculatorFloat::Str(y) => CalculatorFloat::Str(format!("({x} * {y})")),
            },
        }
    }
}

/// Implement `*=` (multiply) for CalculatorFloat and generic type `T`.
///
/// # Arguments
///
/// * `other` - Any type T for which CalculatorFloat::From<T> trait is implemented
///
impl<T> ops::MulAssign<T> for CalculatorFloat
where
    T: Into<CalculatorFloat>,
{
    fn mul_assign(&mut self, other: T) {
        let other_from: CalculatorFloat = other.into();
        match self {
            Self::Float(x) => match other_from {
                Self::Float(y) => {
                    *self = Self::Float(*x * y);
                }
                Self::Str(y) => {
                    *self = {
                        if (*x - 0.0).abs() < ATOL {
                            Self::Float(0.0)
                        } else if (*x - 1.0).abs() < ATOL {
                            Self::Str(y)
                        } else {
                            Self::Str(format!("({x:e} * {y})"))
                        }
                    }
                }
            },
            Self::Str(x) => match other_from {
                Self::Float(y) => {
                    *self = {
                        if y == 0.0 {
                            Self::Float(0.0)
                        } else if (y - 1.0).abs() < ATOL {
                            Self::Str(x.to_string())
                        } else {
                            Self::Str(format!("({x} * {y:e})"))
                        }
                    }
                }
                Self::Str(y) => *self = Self::Str(format!("({x} * {y})")),
            },
        }
    }
}

/// Implement `-` (subtract) for CalculatorFloat and generic type `T`.
///
/// # Arguments
///
/// * `other` - Any type T for which CalculatorFloat::From<T> trait is implemented
///
impl<T> ops::Sub<T> for CalculatorFloat
where
    T: Into<CalculatorFloat>,
{
    type Output = Self;
    fn sub(self, other: T) -> Self {
        let other_from: CalculatorFloat = other.into();
        match self {
            CalculatorFloat::Float(x) => match other_from {
                CalculatorFloat::Float(y) => CalculatorFloat::Float(x - y),
                CalculatorFloat::Str(y) => {
                    if x != 0.0 {
                        CalculatorFloat::Str(format!("({x:e} - {y})"))
                    } else {
                        CalculatorFloat::Str(format!("(-{})", &y))
                    }
                }
            },
            CalculatorFloat::Str(x) => match other_from {
                CalculatorFloat::Float(y) => {
                    if y != 0.0 {
                        CalculatorFloat::Str(format!("({x} - {y:e})"))
                    } else {
                        CalculatorFloat::Str(x)
                    }
                }
                CalculatorFloat::Str(y) => CalculatorFloat::Str(format!("({x} - {y})")),
            },
        }
    }
}

/// Implement `-=` (subtract) for CalculatorFloat and generic type `T`.
///
/// # Arguments
///
/// * `other` - Any type T for which CalculatorFloat::From<T> trait is implemented
///
impl<T> ops::SubAssign<T> for CalculatorFloat
where
    T: Into<CalculatorFloat>,
{
    fn sub_assign(&mut self, other: T) {
        let other_from: CalculatorFloat = other.into();
        match self {
            Self::Float(x) => match other_from {
                Self::Float(y) => {
                    *self = Self::Float(*x - y);
                }
                Self::Str(y) => {
                    *self = {
                        if (*x - 0.0).abs() > ATOL {
                            Self::Str(format!("({x:e} - {y})"))
                        } else {
                            Self::Str(format!("(-{y})"))
                        }
                    }
                }
            },
            Self::Str(x) => match other_from {
                Self::Float(y) => {
                    *self = {
                        if y != 0.0 {
                            Self::Str(format!("({x} - {y:e})"))
                        } else {
                            Self::Str(x.to_owned())
                        }
                    }
                }
                Self::Str(y) => *self = Self::Str(format!("({x} - {y})")),
            },
        }
    }
}

/// Implement minus sign for CalculatorFloat.
impl ops::Neg for CalculatorFloat {
    type Output = CalculatorFloat;

    fn neg(self) -> Self {
        match self {
            Self::Float(x) => Self::Float(-x),
            Self::Str(y) => Self::Str(format!("(-{y})")),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::CalculatorFloat;
    #[cfg(feature = "json_schema")]
    use schemars::schema_for;
    use serde_test::{assert_tokens, Configure, Token};
    use std::{convert::TryFrom, str::FromStr};

    // Test the serialization/deserialization of CalculatorFloat from string
    #[test]
    fn ser_de_string() {
        let x = CalculatorFloat::from("test+(1/3)");
        assert_tokens(&x.readable(), &[Token::String("test+(1/3)")]);
    }

    // Test the serialization/deserialization of CalculatorFloat from float
    #[test]
    fn ser_de_float() {
        let x = CalculatorFloat::from(3.0);
        assert_tokens(&x.readable(), &[Token::F64(3.0)]);
    }

    // Test the serialization/deserialization of CalculatorFloat from integer
    #[test]
    fn ser_de_int() {
        let x = CalculatorFloat::from(0);
        assert_tokens(&x.readable(), &[Token::F64(0.0)]);
    }

    #[test]
    fn ser_de_string_compact() {
        let x = CalculatorFloat::from("test+(1/3)");
        assert_tokens(
            &x.compact(),
            &[
                Token::NewtypeVariant {
                    name: "CalculatorFloat",
                    variant: "Str",
                },
                Token::String("test+(1/3)"),
            ],
        );
    }

    // Test the serialization/deserialization of CalculatorFloat from float
    #[test]
    fn ser_de_float_compact() {
        let x = CalculatorFloat::from(3.0);
        assert_tokens(
            &x.compact(),
            &[
                Token::NewtypeVariant {
                    name: "CalculatorFloat",
                    variant: "Float",
                },
                Token::F64(3.0),
            ],
        );
    }

    // Test the serialization/deserialization of CalculatorFloat from integer
    #[test]
    fn ser_de_int_compact() {
        let x = CalculatorFloat::from(0);
        assert_tokens(
            &x.compact(),
            &[
                Token::NewtypeVariant {
                    name: "CalculatorFloat",
                    variant: "Float",
                },
                Token::F64(0.0),
            ],
        );
    }

    #[cfg(feature = "json_schema")]
    #[test]
    fn test_json_schema_support() {
        let schema = schema_for!(CalculatorFloat);
        let serialized = serde_json::to_string(&schema).unwrap();
        assert_eq!(serialized.as_str(), "{\"$schema\":\"http://json-schema.org/draft-07/schema#\",\"title\":\"CalculatorFloat\",\"oneOf\":[{\"type\":\"number\",\"format\":\"double\"},{\"type\":\"string\"}]}");
    }

    // Test the initialisation of CalculatorFloat from all possible input types
    #[test]
    fn from() {
        // Float (f64, &f64, String but is float) init
        let x = CalculatorFloat::from(3.0);
        if let CalculatorFloat::Float(y) = x {
            assert!((y - 3.0).abs() < f64::EPSILON)
        }
        assert!(x.is_float());

        // u64 init
        let x = CalculatorFloat::from(3u64);
        if let CalculatorFloat::Float(y) = x {
            assert!((y - 3.0).abs() < f64::EPSILON)
        }
        assert!(x.is_float());

        // &u64 init
        let x = CalculatorFloat::from(&3u64);
        if let CalculatorFloat::Float(y) = x {
            assert!((y - 3.0).abs() < f64::EPSILON)
        }
        assert!(x.is_float());

        // i64 init
        let x = CalculatorFloat::from(3i64);
        if let CalculatorFloat::Float(y) = x {
            assert!((y - 3.0).abs() < f64::EPSILON)
        }
        assert!(x.is_float());

        // &i64 init
        let x = CalculatorFloat::from(&3i64);
        if let CalculatorFloat::Float(y) = x {
            assert!((y - 3.0).abs() < f64::EPSILON)
        }
        assert!(x.is_float());

        let x = CalculatorFloat::from(&3.0);
        if let CalculatorFloat::Float(y) = x {
            assert!((y - 3.0).abs() < f64::EPSILON)
        }
        assert!(x.is_float());

        // Integer (i32, u32, &i32, &u32) init
        let x = CalculatorFloat::from(-3);
        if let CalculatorFloat::Float(y) = x {
            assert!((y + 3.0).abs() < f64::EPSILON)
        }
        assert!(x.is_float());

        let x = CalculatorFloat::from(3u32);
        if let CalculatorFloat::Float(y) = x {
            assert!((y - 3.0).abs() < f64::EPSILON)
        }
        assert!(x.is_float());

        let x = CalculatorFloat::from(&-3);
        if let CalculatorFloat::Float(y) = x {
            assert!((y + 3.0).abs() < f64::EPSILON)
        }
        assert!(x.is_float());

        let x = CalculatorFloat::from(&3u32);
        if let CalculatorFloat::Float(y) = x {
            assert!((y - 3.0).abs() < f64::EPSILON)
        }
        assert!(x.is_float());

        // String (String, &String, &str) init
        let inp: &str = "3t";
        let x = CalculatorFloat::from(inp);
        if let CalculatorFloat::Str(y) = x.clone() {
            assert_eq!(y, "3t")
        }
        assert!(!x.is_float());

        let inp2: &str = "3";
        let x2 = CalculatorFloat::from(inp2);
        assert_eq!(x2, CalculatorFloat::from(3));
        assert!(x2.is_float());

        let mut test_string = String::from("3t");
        let x = CalculatorFloat::from(&test_string);
        test_string.push_str(&String::from("2t"));
        if let CalculatorFloat::Str(y) = x.clone() {
            assert_eq!(y, "3t")
        }
        assert!(!x.is_float());

        let test_string = String::from("3t");
        let x = CalculatorFloat::from(test_string);
        if let CalculatorFloat::Str(y) = x.clone() {
            assert_eq!(y, "3t")
        }
        assert!(!x.is_float());

        let mut test_string = String::new();
        test_string.push_str("3t");
        let x = CalculatorFloat::from(test_string);
        if let CalculatorFloat::Str(y) = x.clone() {
            assert_eq!(y, "3t")
        }
        assert!(!x.is_float());

        let inp2 = String::from("3");
        let x2 = CalculatorFloat::from(inp2);
        assert_eq!(x2, CalculatorFloat::from(3));
        assert!(x2.is_float());

        let inp2 = &String::from("3");
        let x2 = CalculatorFloat::from(inp2);
        assert_eq!(x2, CalculatorFloat::from(3));
        assert!(x2.is_float());
    }

    // Test the reverse from functions: T::from(CalculatorFloat)
    #[test]
    fn from_reversed() {
        // Float (f64, &f64, String but is float) init
        let x2 = CalculatorFloat::from("3t");
        assert_eq!(String::from(x2), "3t");
    }

    // Test the initialisation of CalculatorFloat from string,
    // which panics when converted into float
    #[test]
    #[should_panic]
    fn fail_try_from() {
        let x2 = CalculatorFloat::from("test");
        f64::try_from(x2).unwrap();
    }

    #[test]
    fn try_from() {
        let x2 = CalculatorFloat::from("2");
        let x: f64 = 2.0;
        assert!((x - f64::try_from(x2).unwrap()).abs() < f64::EPSILON);
        let x3 = CalculatorFloat::from(2.0);
        assert!((x - f64::try_from(x3).unwrap()).abs() < f64::EPSILON);
    }

    // Test the add functionality of CalculatorFloat with all possible input types
    #[test]
    fn add() {
        // Test simple add function with &CalculatorFloat: &x + y
        let x3 = &CalculatorFloat::from(3);
        let x2 = &CalculatorFloat::from(2.0);
        assert_eq!(x2 + x3, CalculatorFloat::Float(5.0));
        assert_eq!(x3 + 2, CalculatorFloat::Float(5.0));
        assert_eq!(x3 + 2.0, CalculatorFloat::Float(5.0));

        let x2 = &CalculatorFloat::from(0.0);
        assert_eq!(x2 + "3t", CalculatorFloat::Str(String::from("3t")));

        let x2s = &CalculatorFloat::from("3t");
        assert_eq!(x2s + 0.0, CalculatorFloat::Str(String::from("3t")));
        assert_eq!(x2s + 1.0, CalculatorFloat::Str(String::from("(3t + 1e0)")));
        assert_eq!(x2s + "2x", CalculatorFloat::Str(String::from("(3t + 2x)")));

        // Test simple add function: x + y
        let mut x3 = CalculatorFloat::from(3);
        let x2 = CalculatorFloat::from(2.0);
        if let CalculatorFloat::Float(y) = x3.clone() + x2 {
            assert!((y - 5.0).abs() < f64::EPSILON)
        }
        if let CalculatorFloat::Float(y) = x3.clone() + 2 {
            assert!((y - 5.0).abs() < f64::EPSILON)
        }
        if let CalculatorFloat::Float(y) = x3.clone() + 2.0 {
            assert!((y - 5.0).abs() < f64::EPSILON)
        }

        let x2 = CalculatorFloat::from(0.0);
        if let CalculatorFloat::Str(y) = x2 + "3t" {
            assert_eq!(y, "3t")
        }

        let x2 = CalculatorFloat::from("3t");
        if let CalculatorFloat::Str(y) = x2.clone() + 0.0 {
            assert_eq!(y, "3t")
        }
        if let CalculatorFloat::Str(y) = x2 + "2x" {
            assert_eq!(y, "(3t + 2x)")
        }

        // Test add_assign function: x += y
        let x2 = CalculatorFloat::from(2.0);
        x3 += x2.clone();
        if let CalculatorFloat::Float(y) = x3.clone() {
            assert!((y - 5.0).abs() < f64::EPSILON)
        }
        x3 += "x";
        if let CalculatorFloat::Str(y) = x3.clone() {
            assert_eq!(y, "(5e0 + x)")
        }
        let mut x3 = CalculatorFloat::from(0.0);
        x3 += "x";
        if let CalculatorFloat::Str(y) = x3.clone() {
            assert_eq!(y, "x")
        }
        let mut x3s = CalculatorFloat::from("3t");
        if let CalculatorFloat::Str(y) = x3s.clone() + x2.clone() {
            assert_eq!(y, "(3t + 2e0)")
        }
        if let CalculatorFloat::Str(y) = x3s.clone() + "2e0" {
            assert_eq!(y, "(3t + 2e0)")
        }
        if let CalculatorFloat::Str(y) = x3s.clone() + x2.clone() {
            assert_eq!(y, "(3t + 2e0)")
        }

        x3s += x2;
        if let CalculatorFloat::Str(y) = x3s.clone() {
            assert_eq!(y, "(3t + 2e0)")
        }
        x3s += 0.0;
        if let CalculatorFloat::Str(y) = x3s.clone() {
            assert_eq!(y, "(3t + 2e0)")
        }
        x3s += "x";
        if let CalculatorFloat::Str(y) = x3s.clone() {
            assert_eq!(y, "((3t + 2e0) + x)")
        }
    }

    // Test the divide functionality of CalculatorFloat with all possible input types
    #[test]
    fn div() {
        // Test simple divide function: x / y
        let mut x3 = CalculatorFloat::from(3);
        let x2 = CalculatorFloat::from(3.0);
        assert_eq!(x3.clone() / x2.clone(), CalculatorFloat::Float(1.0));
        assert_eq!(x3.clone() / 3, CalculatorFloat::Float(1.0));
        assert_eq!(x3.clone() / 3.0, CalculatorFloat::Float(1.0));
        assert_eq!(
            x3.clone() / "x",
            CalculatorFloat::Str(String::from("(3e0 / x)"))
        );

        let mut x0 = CalculatorFloat::from(0.0);
        assert_eq!(x0.clone() / "3t", x0);

        let mut x3s = CalculatorFloat::from("3t");
        assert_eq!(
            x3s.clone() / x2.clone(),
            CalculatorFloat::Str(String::from("(3t / 3e0)"))
        );
        assert_eq!(
            x3s.clone() / 2.0,
            CalculatorFloat::Str(String::from("(3t / 2e0)"))
        );
        assert_eq!(
            x3s.clone() / 2.0,
            CalculatorFloat::Str(String::from("(3t / 2e0)"))
        );
        assert_eq!(
            x3s.clone() / "2.0",
            CalculatorFloat::Str(String::from("(3t / 2e0)"))
        );
        assert_eq!(x3s.clone() / 1.0, x3s);

        let x2s = CalculatorFloat::from("2x");
        assert_eq!(
            x3s.clone() / x2s.clone(),
            CalculatorFloat::Str(String::from("(3t / 2x)"))
        );

        // Test div_assign function: x /= y
        x3 /= x2.clone();
        assert_eq!(x3, CalculatorFloat::Float(1.0));

        x0 /= "x";
        assert_eq!(x0, CalculatorFloat::Float(0.0));

        x3 /= x2s.clone();
        assert_eq!(x3, CalculatorFloat::Str(String::from("(1e0 / 2x)")));
        x3s /= 1.0;
        assert_eq!(x3s, CalculatorFloat::Str(String::from("3t")));
        x3s /= x2;
        assert_eq!(x3s, CalculatorFloat::Str(String::from("(3t / 3e0)")));
        x3s /= x2s;
        assert_eq!(x3s, CalculatorFloat::Str(String::from("((3t / 3e0) / 2x)")));
    }

    // Test the division of CalculatorFloat from float by zero (should panic)
    #[test]
    #[should_panic]
    fn fail_div_by_zero_float() {
        let x1 = CalculatorFloat::from(1.0);
        let _x3 = x1 / 0.0;
    }

    // Test the division of CalculatorFloat from string by zero (should panic)
    #[test]
    #[should_panic]
    fn fail_div_by_zero_str() {
        let x2 = CalculatorFloat::from("x");
        let _x4 = x2 / 0.0;
    }

    // Test the div_assign of CalculatorFloat from float by zero (should panic)
    #[test]
    #[should_panic]
    fn fail_div_assign_by_zero_float() {
        let mut x1 = CalculatorFloat::from(1.0);
        x1 /= 0.0;
    }

    // Test the div_assign of CalculatorFloat from string by zero (should panic)
    #[test]
    #[should_panic]
    fn fail_div_assign_by_zero_str() {
        let mut x2 = CalculatorFloat::from("x");
        x2 /= 0.0;
    }

    // Test the multiply functionality of CalculatorFloat with all possible input types
    #[test]
    fn mult() {
        // Test simple multiply function: x * y
        let mut x3 = CalculatorFloat::from(3);
        let x2 = CalculatorFloat::from(3.0);
        assert_eq!(x3.clone() * x2, CalculatorFloat::Float(9.0));
        assert_eq!(x3.clone() * 3, CalculatorFloat::Float(9.0));
        assert_eq!(x3.clone() * 3.0, CalculatorFloat::Float(9.0));
        assert_eq!(
            x3.clone() * "x",
            CalculatorFloat::Str(String::from("(3e0 * x)"))
        );

        let x2 = CalculatorFloat::from(0.0);
        assert_eq!(x2 * "x", CalculatorFloat::Float(0.0));

        let x2 = CalculatorFloat::from(1.0);
        assert_eq!(x2 * "x", CalculatorFloat::Str(String::from("x")));

        let mut x3s = CalculatorFloat::from("3t");
        let x2 = CalculatorFloat::from(3.0);
        assert_eq!(
            x3s.clone() * x2.clone(),
            CalculatorFloat::Str(String::from("(3t * 3e0)"))
        );
        assert_eq!(
            x3s.clone() * 2.0,
            CalculatorFloat::Str(String::from("(3t * 2e0)"))
        );
        assert_eq!(x3s.clone() * 0.0, CalculatorFloat::Float(0.0));
        assert_eq!(x3s.clone() * 1.0, CalculatorFloat::Str(String::from("3t")));
        assert_eq!(
            x3s.clone() * "2x",
            CalculatorFloat::Str(String::from("(3t * 2x)"))
        );

        // Test mul_assign function: x *= y
        x3 *= x2.clone();
        assert_eq!(x3, CalculatorFloat::Float(9.0));

        let mut x3 = CalculatorFloat::from(0.0);
        x3 *= "x";
        assert_eq!(x3, CalculatorFloat::Float(0.0));

        let mut x3 = CalculatorFloat::from(1.0);
        x3 *= "x";
        assert_eq!(x3, CalculatorFloat::Str(String::from("x")));

        let mut x3 = CalculatorFloat::from(3.0);
        x3 *= "x";
        assert_eq!(x3, CalculatorFloat::Str(String::from("(3e0 * x)")));

        x3s *= 1.0;
        assert_eq!(x3s, CalculatorFloat::Str(String::from("3t")));

        x3s *= x2;
        assert_eq!(x3s, CalculatorFloat::Str(String::from("(3t * 3e0)")));

        x3s *= "2x";
        assert_eq!(x3s, CalculatorFloat::Str(String::from("((3t * 3e0) * 2x)")));

        x3s *= 0.0;
        assert_eq!(x3s, CalculatorFloat::Float(0.0));

        let x4 = &CalculatorFloat::from(4.0);
        let x5 = &CalculatorFloat::from(5.0);

        assert_eq!(x4 * x5, CalculatorFloat::Float(20.0));
    }

    #[test]
    fn default() {
        let a = CalculatorFloat::default();
        assert_eq!(a, CalculatorFloat::Float(0.0));
    }

    #[test]
    fn from_str() {
        let expression = "=";
        let result = CalculatorFloat::from_str(expression);
        assert!(result.is_err());
        let expression = "a=3";
        let result = CalculatorFloat::from_str(expression);
        assert!(result.is_err());
        let expression = "?";
        let result = CalculatorFloat::from_str(expression);
        assert!(result.is_err());
        let expression = "a+2";
        let result = CalculatorFloat::from_str(expression);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), CalculatorFloat::Str("a+2".to_string()))
    }

    // Test the subtract functionality of CalculatorFloat with all possible input types
    #[test]
    fn sub() {
        // Test simple subtract function: x - y
        let x3 = CalculatorFloat::from(3);
        let x2 = CalculatorFloat::from(3.0);
        assert_eq!(x3.clone() - x2.clone(), CalculatorFloat::Float(0.0));
        assert_eq!(x3.clone() - 3, CalculatorFloat::Float(0.0));
        assert_eq!(x3.clone() - 3.0, CalculatorFloat::Float(0.0));
        assert_eq!(x3 - "x", CalculatorFloat::Str(String::from("(3e0 - x)")));

        let x3 = CalculatorFloat::from(0.0);
        assert_eq!(x3 - "x", CalculatorFloat::Str(String::from("(-x)")));

        let mut x3s = CalculatorFloat::from("3t");
        assert_eq!(
            x3s.clone() - x2.clone(),
            CalculatorFloat::Str(String::from("(3t - 3e0)"))
        );
        assert_eq!(
            x3s.clone() - 2.0,
            CalculatorFloat::Str(String::from("(3t - 2e0)"))
        );
        assert_eq!(x3s.clone() - 0.0, CalculatorFloat::Str(String::from("3t")));
        assert_eq!(
            x3s.clone() - "2.0",
            CalculatorFloat::Str(String::from("(3t - 2e0)"))
        );
        assert_eq!(
            x3s.clone() - "2x",
            CalculatorFloat::Str(String::from("(3t - 2x)"))
        );

        // Test sub_assign function: x -= y
        let mut x3 = CalculatorFloat::from(3);
        x3 -= x2.clone();
        assert_eq!(x3, CalculatorFloat::Float(0.0));
        x3 -= "x";
        assert_eq!(x3, CalculatorFloat::Str(String::from("(-x)")));

        let mut x3 = CalculatorFloat::from(3);
        x3 -= "x";
        assert_eq!(x3, CalculatorFloat::Str(String::from("(3e0 - x)")));

        x3s -= x2;
        assert_eq!(x3s, CalculatorFloat::Str(String::from("(3t - 3e0)")));
        x3s -= 0.0;
        assert_eq!(x3s, CalculatorFloat::Str(String::from("(3t - 3e0)")));
        x3s -= "x";
        assert_eq!(x3s, CalculatorFloat::Str(String::from("((3t - 3e0) - x)")));
    }

    // Test the negative (*-1) functionality of CalculatorFloat with all possible input types
    #[test]
    fn neg() {
        let x3 = CalculatorFloat::from(3);
        let x2 = -x3;
        assert_eq!(x2, CalculatorFloat::Float(-3.0));
        let x3s = CalculatorFloat::from("3t");
        let x2 = -x3s;
        assert_eq!(x2, CalculatorFloat::Str(String::from("(-3t)")));
    }

    // Test the square root functionality of CalculatorFloat with all possible input types
    #[test]
    fn sqrt() {
        let x3 = CalculatorFloat::from(3);
        let x2: f64 = 3.0;
        assert_eq!(CalculatorFloat::Float(x2.sqrt()), x3.sqrt());
        let x3s = CalculatorFloat::from("3t");
        assert_eq!(x3s.sqrt(), CalculatorFloat::Str(String::from("sqrt(3t)")));
    }

    // Test the arccosine functionality of CalculatorFloat with all possible input types
    #[test]
    fn acos() {
        let x3 = CalculatorFloat::from(1);
        let x2: f64 = 1.0;
        assert_eq!(CalculatorFloat::Float(x2.acos()), x3.acos());
        let x3s = CalculatorFloat::from("1t");
        assert_eq!(x3s.acos(), CalculatorFloat::Str(String::from("acos(1t)")));
    }

    // Test the exponential functionality of CalculatorFloat with all possible input types
    #[test]
    fn exp() {
        let x3 = CalculatorFloat::from(3);
        let x2: f64 = 3.0;
        assert_eq!(CalculatorFloat::Float(x2.exp()), x3.exp());
        let x3s = CalculatorFloat::from("3t");
        assert_eq!(x3s.exp(), CalculatorFloat::Str(String::from("exp(3t)")));
    }

    // Test the absolute value functionality of CalculatorFloat with all possible input types
    #[test]
    fn abs() {
        let x3 = CalculatorFloat::from(-3);
        let x2: f64 = -3.0;
        assert_eq!(CalculatorFloat::Float(x2.abs()), x3.abs());
        let x3s = CalculatorFloat::from("-3t");
        assert_eq!(x3s.abs(), CalculatorFloat::Str(String::from("abs(-3t)")));
    }

    // Test the cosine functionality of CalculatorFloat with all possible input types
    #[test]
    fn cos() {
        let x3 = CalculatorFloat::from(-3);
        let x2: f64 = -3.0;
        assert_eq!(CalculatorFloat::Float(x2.cos()), x3.cos());
        let x3s = CalculatorFloat::from("-3t");
        assert_eq!(x3s.cos(), CalculatorFloat::Str(String::from("cos(-3t)")));
    }

    // Test the sine functionality of CalculatorFloat with all possible input types
    #[test]
    fn sin() {
        let x3 = CalculatorFloat::from(-3);
        let x2: f64 = -3.0;
        assert_eq!(CalculatorFloat::Float(x2.sin()), x3.sin());
        let x3s = CalculatorFloat::from("-3t");
        assert_eq!(x3s.sin(), CalculatorFloat::Str(String::from("sin(-3t)")));
    }

    // Test the arctangent functionality of CalculatorFloat with all possible input types
    #[test]
    fn atan2() {
        // Test atan2
        let x3 = CalculatorFloat::from(-3);
        let x2: f64 = -3.0;
        assert_eq!(CalculatorFloat::Float(x2.atan2(2.0)), x3.atan2(2.0));
        let x3s = CalculatorFloat::from("-3t");
        assert_eq!(
            x3s.atan2("test"),
            CalculatorFloat::Str(String::from("atan2(-3t, test)"))
        );
        assert_eq!(
            x3s.atan2(1.0),
            CalculatorFloat::Str(String::from("atan2(-3t, 1e0)"))
        );
        assert_eq!(
            x3.atan2("test"),
            CalculatorFloat::Str(String::from("atan2(-3e0, test)"))
        );
    }

    // Test the sign functionality of CalculatorFloat with all possible input types
    #[test]
    fn signum() {
        let x2 = CalculatorFloat::from(-3);
        let x3 = CalculatorFloat::from("-3t");
        assert_eq!(x2.signum(), CalculatorFloat::Float(-1.0));
        assert_eq!(x3.signum(), CalculatorFloat::Str(String::from("sign(-3t)")));
    }

    // Test the power functionality of CalculatorFloat with all possible input types
    #[test]
    fn powf() {
        let x1 = CalculatorFloat::from(2.0);
        let x1s = CalculatorFloat::from("2x");
        assert_eq!(x1.powf(2.0), CalculatorFloat::from(4.0));
        assert_eq!(
            x1.powf("t"),
            CalculatorFloat::Str(String::from("(2e0 ^ t)"))
        );
        assert_eq!(
            x1s.powf(2.0),
            CalculatorFloat::Str(String::from("(2x ^ 2e0)"))
        );
        assert_eq!(
            x1s.powf("t"),
            CalculatorFloat::Str(String::from("(2x ^ t)"))
        );
    }

    // Test the inverse/reciprocal functionality of CalculatorFloat with all possible input types
    #[test]
    fn recip() {
        let x1 = CalculatorFloat::from(2.0);
        let x1s = CalculatorFloat::from("2x");
        let x1_recip = x1.recip();
        let x1s_recip = x1s.recip();
        assert_eq!(x1_recip, CalculatorFloat::from(0.5));
        assert_eq!(x1s_recip, CalculatorFloat::Str(String::from("(1 / 2x)")));
    }

    // Test the Display functionality of CalculatorFloat with all possible input types
    #[test]
    fn display() {
        let x2 = CalculatorFloat::from(-3);
        let x3 = CalculatorFloat::from("-3t");
        assert_eq!(format!("{x2}"), "-3e0");
        assert_eq!(format!("{x3}"), "-3t");
    }

    // Test the isclose functionality of CalculatorFloat with all possible input types
    #[test]
    fn isclose() {
        let x2 = CalculatorFloat::from(-3);
        let x3 = CalculatorFloat::from("-3t");
        assert!(x2.isclose(-3.000000001));
        assert!(!x3.isclose("-3.000000001t"));
        assert!(!x3.isclose(-3.000000001));
        assert!(!x2.isclose("-3.000000001t"));
    }

    // Test the adding with reference input functionality of CalculatorFloat
    // with all possible input types
    #[test]
    fn add_ref() {
        let mut x3 = CalculatorFloat::from(3);
        let x2 = CalculatorFloat::from(2.0);
        assert_eq!(&x3 + &x2, CalculatorFloat::Float(5.0));
        assert_eq!(&x3 + 2, CalculatorFloat::Float(5.0));
        assert_eq!(&x3 + 2.0, CalculatorFloat::Float(5.0));

        x3 += &x2;
        assert_eq!(x3, CalculatorFloat::Float(5.0));
        let mut x3s = CalculatorFloat::from("3t");
        assert_eq!(
            x3s.clone() + x2.clone(),
            CalculatorFloat::Str(String::from("(3t + 2e0)"))
        );
        assert_eq!(
            x3s.clone() + 2.0,
            CalculatorFloat::Str(String::from("(3t + 2e0)"))
        );
        assert_eq!(
            x3s.clone() + 2.0,
            CalculatorFloat::Str(String::from("(3t + 2e0)"))
        );
        assert_eq!(
            x3s.clone() + "2.0",
            CalculatorFloat::Str(String::from("(3t + 2e0)"))
        );
        x3s += x2;
        assert_eq!(x3s, CalculatorFloat::Str(String::from("(3t + 2e0)")));
    }

    // Test the Debug trait for CalculatorFloat
    #[test]
    fn debug() {
        let x = CalculatorFloat::from(3.0);
        assert_eq!(format!("{x:?}"), "Float(3.0)");

        let xs = CalculatorFloat::from("3x");
        assert_eq!(format!("{xs:?}"), "Str(\"3x\")");
    }

    // Test the Clone trait for CalculatorFloat
    #[test]
    fn clone_trait() {
        let x = CalculatorFloat::from(3.0);
        assert_eq!(x.clone(), x);

        let xs = CalculatorFloat::from("3x");
        assert_eq!(xs.clone(), xs);
    }

    // Test the PartialEq trait for CalculatorFloat
    #[test]
    fn partial_eq() {
        let x1 = CalculatorFloat::from(3.0);
        let x2 = CalculatorFloat::from(3.0);
        assert!(x1 == x2);
        assert!(x2 == x1);

        let x1s = CalculatorFloat::from("3x");
        let x2s = CalculatorFloat::from("3x");
        assert!(x1s == x2s);
        assert!(x2s == x1s);
    }
}
// End of tests
