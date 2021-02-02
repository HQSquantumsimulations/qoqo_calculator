
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

mod calculator_float;
pub use calculator_float::CalculatorFloat;
mod calculator;
pub use calculator::Calculator;
mod calculator_complex;
pub use calculator_complex::CalculatorComplex;
use thiserror::Error;

/// Defines custom errors for Calculator
#[derive(Error, Debug, PartialEq)]
pub enum CalculatorError {
    #[error("Input can not be converted to CalculatorFloat")]
    NotConvertable,
    #[error("Symbolic value {val:?} can not be converted to float")]
    FloatSymbolicNotConvertable {
        val: String,
    },
    #[error("Symbolic value {val:?} can not be converted to complex")]
    ComplexSymbolicNotConvertable {
        val: CalculatorComplex,
    },
    #[error("Imaginary part of CalculatorComplex {val:?} not zero")]
    ComplexCanNotBeConvertedToFloat{
        val: CalculatorComplex,
    },
    #[error("Parsing error: {msg:?}")]
    ParsingError{
        msg: &'static str,
    },
    #[error("Function {fct:?} not implemented.")]
    NotImplementedError{
        fct: &'static str,
    },
    #[error("Function {fct:?} not found.")]
    FunctionNotFound{
        fct: String,
    },
    #[error("Variable {name:?} not set.")]
    VariableNotSet{
        name: String,
    },
    #[error("Parsing error: Unexpected end of expression")]
    UnexpetedEndOfExpression,
    #[error("Division by zero error")]
    DivisionByZero,
    #[error("Parsing Expression did not return value as expected.")]
    NoValueReturnedParsing,
    #[error("Not enough function arguments.")]
    NotEnoughFunctionArguments,
}