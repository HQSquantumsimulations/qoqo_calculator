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
    #[error("Input cannot be converted to CalculatorFloat")]
    NotConvertable,
    #[error("Symbolic value {val:?} can not be converted to float")]
    FloatSymbolicNotConvertable { val: String },
    #[error("Symbolic value {val:?} can not be converted to complex")]
    ComplexSymbolicNotConvertable { val: CalculatorComplex },
    #[error("Imaginary part of CalculatorComplex {val:?} not zero")]
    ComplexCanNotBeConvertedToFloat { val: CalculatorComplex },
    #[error("Parsing error: {msg:?}")]
    ParsingError { msg: &'static str },
    #[error("Function {fct:?} not implemented.")]
    NotImplementedError { fct: &'static str },
    #[error("Function {fct:?} not found.")]
    FunctionNotFound { fct: String },
    #[error("Variable {name:?} not set.")]
    VariableNotSet { name: String },
    #[error("Parsing error: Unexpected end of expression")]
    UnexpectedEndOfExpression,
    #[error("Division by zero error")]
    DivisionByZero,
    #[error("Parsing Expression did not return value as expected.")]
    NoValueReturnedParsing,
    #[error("Not enough function arguments.")]
    NotEnoughFunctionArguments,
}

#[cfg(test)]
mod tests {
    use super::CalculatorComplex;
    use super::CalculatorError;

    // Test all CalculatorErrors give the correct output (debug)
    #[test]
    fn test_debug() {
        let not_conv = CalculatorError::NotConvertable;
        assert_eq!(format!("{:?}", not_conv), "NotConvertable");

        let float_sym = CalculatorError::FloatSymbolicNotConvertable {
            val: String::from("2x"),
        };
        assert_eq!(
            format!("{:?}", float_sym),
            "FloatSymbolicNotConvertable { val: \"2x\" }"
        );

        let complex_sym = CalculatorError::ComplexSymbolicNotConvertable {
            val: CalculatorComplex::from("2x"),
        };
        assert_eq!(
            format!("{:?}", complex_sym),
            "ComplexSymbolicNotConvertable { val: CalculatorComplex { re: Str(\"2x\"), im: Float(0.0) } }"
        );

        let complex_im_sym = CalculatorError::ComplexSymbolicNotConvertable {
            val: CalculatorComplex::new(1, 3),
        };
        assert_eq!(
            format!("{:?}", complex_im_sym),
            "ComplexSymbolicNotConvertable { val: CalculatorComplex { re: Float(1.0), im: Float(3.0) } }"
        );

        let parse = CalculatorError::ParsingError { msg: "test" };
        assert_eq!(format!("{:?}", parse), "ParsingError { msg: \"test\" }");

        let not_impl = CalculatorError::NotImplementedError { fct: "Test" };
        assert_eq!(
            format!("{:?}", not_impl),
            "NotImplementedError { fct: \"Test\" }"
        );

        let func_not_found = CalculatorError::FunctionNotFound {
            fct: String::from("Test"),
        };
        assert_eq!(
            format!("{:?}", func_not_found),
            "FunctionNotFound { fct: \"Test\" }"
        );

        let var_not_set = CalculatorError::VariableNotSet {
            name: String::from("Test"),
        };
        assert_eq!(
            format!("{:?}", var_not_set),
            "VariableNotSet { name: \"Test\" }"
        );

        let end_of_exp = CalculatorError::UnexpectedEndOfExpression;
        assert_eq!(format!("{:?}", end_of_exp), "UnexpectedEndOfExpression");

        let div_zero = CalculatorError::DivisionByZero;
        assert_eq!(format!("{:?}", div_zero), "DivisionByZero");

        let parsing_no_val = CalculatorError::NoValueReturnedParsing;
        assert_eq!(format!("{:?}", parsing_no_val), "NoValueReturnedParsing");

        let func_args = CalculatorError::NotEnoughFunctionArguments;
        assert_eq!(format!("{:?}", func_args), "NotEnoughFunctionArguments");
    }
}
