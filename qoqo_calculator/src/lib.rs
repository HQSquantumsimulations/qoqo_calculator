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

#![deny(missing_docs)]
#![warn(rustdoc::private_intra_doc_links)]
#![warn(rustdoc::missing_crate_level_docs)]
#![warn(rustdoc::private_doc_tests)]
#![deny(missing_debug_implementations)]

//! qoqo_calculator module
//!
//! Provides CalculatorError enum for all custom errors relating to
//! Calculator, CalculatorFloat and CalculatorComplex.

mod calculator_float;
pub use calculator_float::CalculatorFloat;
mod calculator;
pub use calculator::Calculator;
mod calculator_complex;
pub use calculator_complex::CalculatorComplex;
use thiserror::Error;

/// Define custom errors for Calculator.
#[derive(Error, Debug, PartialEq)]
pub enum CalculatorError {
    /// An input cannot be converted to CalculatorFloat
    #[error("Input cannot be converted to CalculatorFloat")]
    NotConvertable,
    /// A symbolic input cannot be converted to CalculatorFloat
    #[error("Symbolic value {val:?} can not be converted to float")]
    FloatSymbolicNotConvertable {
        /// Value that can not be converted
        val: String,
    },
    /// A symbolic input cannot be converted to CalculatorComplex
    #[error("Symbolic value {val:?} can not be converted to complex")]
    ComplexSymbolicNotConvertable {
        /// Value that cannot be converted
        val: CalculatorComplex,
    },
    /// A complex value cannot be converted to float because imaginary part is not zero
    #[error("Imaginary part of CalculatorComplex {val:?} not zero")]
    ComplexCanNotBeConvertedToFloat {
        /// Value of the CalculatorComplex that cannot be converted
        val: CalculatorComplex,
    },
    #[error("Parsing error: {msg:?}")]
    /// Parsing error when using Calculator
    ParsingError {
        /// Parsing error
        msg: &'static str,
    },
    /// Function not implemented in Calculator
    #[error("Function {fct:?} not implemented.")]
    NotImplementedError {
        /// Function that is not implemented
        fct: &'static str,
    },
    /// Function not found in Calculator
    #[error("Function {fct:?} not found.")]
    FunctionNotFound {
        /// Name of function that cannot be found
        fct: String,
    },
    /// A variable is not set
    #[error("Variable {name:?} not set.")]
    VariableNotSet {
        /// Name of the variable that is not set
        name: String,
    },
    /// Parsed expression ended unexpectedly
    #[error("Parsing error: Unexpected end of expression")]
    UnexpectedEndOfExpression,
    /// Trying to divide by zero
    #[error("Division by zero error")]
    DivisionByZero,
    /// A parsed value did not return a value.
    #[error("Parsing Expression did not return value as expected.")]
    NoValueReturnedParsing,
    /// Not enough function arguments provided in parsed expression.
    #[error("Not enough function arguments.")]
    NotEnoughFunctionArguments,
    /// Trying to assign variable in side-effect free parsing.
    #[error("Trying to assign variable {variable_name} in side-effect free parsing. Set variable in Calculator with .set_variable, replace with number in str or use parse_str_assign to resolve error.")]
    ForbiddenAssign {
        /// Name of the variable that is being assigned
        variable_name: String,
    },
    /// Error raised when checking if a String-CalculatorFloat is valid and can be parsed
    #[error("CalculatorFloat::Str is not a valid expression that can be parsed: Variable assignment to {variable_name}")]
    NotParsableAssign {
        /// Name of the variable that is being assigned
        variable_name: String,
    },
    /// Error raised when checking if a String-CalculatorFloat is valid and can be parsed
    #[error("CalculatorFloat::Str is not a valid expression that can be parsed: Urecognized elements in expression")]
    NotParsableUnrecognized,
    /// Error raised when checking if a String-CalculatorFloat is valid and can be parsed
    #[error("CalculatorFloat::Str is not a valid expression that can be parsed: Assign operator `=` found in expression")]
    NotParsableSingleAssign,
}

#[cfg(test)]
mod tests {
    use super::CalculatorComplex;
    use super::CalculatorError;

    // Test all CalculatorErrors give the correct output (debug)
    #[test]
    fn test_debug() {
        let not_conv = CalculatorError::NotConvertable;
        assert_eq!(format!("{not_conv:?}"), "NotConvertable");

        let float_sym = CalculatorError::FloatSymbolicNotConvertable {
            val: String::from("2x"),
        };
        assert_eq!(
            format!("{float_sym:?}"),
            "FloatSymbolicNotConvertable { val: \"2x\" }"
        );

        let complex_sym = CalculatorError::ComplexSymbolicNotConvertable {
            val: CalculatorComplex::from("2x"),
        };
        assert_eq!(
            format!("{complex_sym:?}"),
            "ComplexSymbolicNotConvertable { val: CalculatorComplex { re: Str(\"2x\"), im: Float(0.0) } }"
        );

        let complex_im_sym = CalculatorError::ComplexSymbolicNotConvertable {
            val: CalculatorComplex::new(1, 3),
        };
        assert_eq!(
            format!("{complex_im_sym:?}"),
            "ComplexSymbolicNotConvertable { val: CalculatorComplex { re: Float(1.0), im: Float(3.0) } }"
        );

        let parse = CalculatorError::ParsingError { msg: "test" };
        assert_eq!(format!("{parse:?}"), "ParsingError { msg: \"test\" }");

        let not_impl = CalculatorError::NotImplementedError { fct: "Test" };
        assert_eq!(
            format!("{not_impl:?}"),
            "NotImplementedError { fct: \"Test\" }"
        );

        let func_not_found = CalculatorError::FunctionNotFound {
            fct: String::from("Test"),
        };
        assert_eq!(
            format!("{func_not_found:?}"),
            "FunctionNotFound { fct: \"Test\" }"
        );

        let var_not_set = CalculatorError::VariableNotSet {
            name: String::from("Test"),
        };
        assert_eq!(
            format!("{var_not_set:?}"),
            "VariableNotSet { name: \"Test\" }"
        );

        let end_of_exp = CalculatorError::UnexpectedEndOfExpression;
        assert_eq!(format!("{end_of_exp:?}"), "UnexpectedEndOfExpression");

        let div_zero = CalculatorError::DivisionByZero;
        assert_eq!(format!("{div_zero:?}"), "DivisionByZero");

        let parsing_no_val = CalculatorError::NoValueReturnedParsing;
        assert_eq!(format!("{parsing_no_val:?}"), "NoValueReturnedParsing");

        let func_args = CalculatorError::NotEnoughFunctionArguments;
        assert_eq!(format!("{func_args:?}"), "NotEnoughFunctionArguments");
    }
}
