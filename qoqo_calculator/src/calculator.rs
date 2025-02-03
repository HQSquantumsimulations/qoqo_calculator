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

//! calculator module
//!
//! Provides Calculator struct for parsing string expressions to floats.

//! calculator module
//!
//! Provides Calculator struct for parsing string expressions to floats.

use crate::{CalculatorError, CalculatorFloat};
use std::collections::HashMap;
use std::fmt;
use std::str::FromStr;
use std::vec::Vec;
static ATOL: f64 = f64::EPSILON;

/// Match name of function to number of arguments.
/// Returns result with CalculatorError when function name is not known.
fn function_argument_numbers(input: &str) -> Result<usize, CalculatorError> {
    match input {
        "sin" => Ok(1),
        "cos" => Ok(1),
        "abs" => Ok(1),
        "tan" => Ok(1),
        "acos" => Ok(1),
        "asin" => Ok(1),
        "atan" => Ok(1),
        "cosh" => Ok(1),
        "sinh" => Ok(1),
        "tanh" => Ok(1),
        "acosh" => Ok(1),
        "asinh" => Ok(1),
        "atanh" => Ok(1),
        "arcosh" => Ok(1),
        "arsinh" => Ok(1),
        "artanh" => Ok(1),
        "exp" => Ok(1),
        "exp2" => Ok(1),
        "expm1" => Ok(1), //< exponential minus Ok(1)
        "log" => Ok(1),
        "log10" => Ok(1),
        "sqrt" => Ok(1),
        "cbrt" => Ok(1), //< cubic root
        "ceil" => Ok(1),
        "floor" => Ok(1),
        "fract" => Ok(1),
        "round" => Ok(1),
        "erf" => Ok(1),
        "tgamma" => Ok(1),
        "lgamma" => Ok(1),
        "sign" => Ok(1),
        "delta" => Ok(1),
        "theta" => Ok(1),
        "parity" => Ok(1),
        "atan2" => Ok(2),
        "hypot" => Ok(2),
        "pow" => Ok(2),
        "max" => Ok(2),
        "min" => Ok(2),
        _ => Err(CalculatorError::FunctionNotFound {
            fct: input.to_string(),
        }),
    }
}

/// Match name of function with one argument to Rust function and return Result.
fn function_1_argument(input: &str, arg0: f64) -> Result<f64, CalculatorError> {
    match input {
        "sin" => Ok(arg0.sin()),
        "cos" => Ok(arg0.cos()),
        "abs" => Ok(arg0.abs()),
        "tan" => Ok(arg0.tan()),
        "acos" => Ok(arg0.acos()),
        "asin" => Ok(arg0.asin()),
        "atan" => Ok(arg0.atan()),
        "cosh" => Ok(arg0.cosh()),
        "sinh" => Ok(arg0.sinh()),
        "tanh" => Ok(arg0.tanh()),
        "acosh" => Ok(arg0.acosh()),
        "asinh" => Ok(arg0.asinh()),
        "atanh" => Ok(arg0.atanh()),
        "arcosh" => Ok(arg0.acosh()),
        "arsinh" => Ok(arg0.asinh()),
        "artanh" => Ok(arg0.atanh()),
        "exp" => Ok(arg0.exp()),
        "exp2" => Ok(arg0.exp2()),
        "expm1" => Ok(arg0.exp_m1()), //< exponential minus 1
        "log" => Ok(arg0.ln()),
        "log10" => Ok(arg0.log10()),
        "sqrt" => Ok(arg0.sqrt()),
        "cbrt" => Ok(arg0.cbrt()), //< cubic root
        "ceil" => Ok(arg0.ceil()),
        "floor" => Ok(arg0.floor()),
        "fract" => Ok(arg0.fract()),
        "round" => Ok(arg0.round()),
        "sign" => Ok(arg0.signum()),
        "delta" => {
            if (arg0 - 0.0).abs() < ATOL {
                Ok(1.0)
            } else {
                Ok(0.0)
            }
        }
        "theta" => {
            if (arg0 - 0.0).abs() < ATOL {
                Ok(0.5)
            } else if arg0 < 0.0 {
                Ok(0.0)
            } else {
                Ok(1.0)
            }
        }
        //"parity" => {let m = i64::from((arg0+0.5).floor());
        //     if m.overflowing_rem(2) {Ok(-1.0)} else {Ok(1.0)}},
        _ => Err(CalculatorError::FunctionNotFound {
            fct: input.to_string(),
        }),
    }
}

/// Match name of function with two arguments to Rust function and return Result.
fn function_2_arguments(input: &str, arg0: f64, arg1: f64) -> Result<f64, CalculatorError> {
    match input {
        "atan2" => Ok(arg0.atan2(arg1)),
        "hypot" => Ok(arg0.hypot(arg1)),
        "pow" => Ok(arg0.powf(arg1)),
        "max" => Ok(arg0.max(arg1)),
        "min" => Ok(arg0.min(arg1)),
        _ => Err(CalculatorError::FunctionNotFound {
            fct: input.to_string(),
        }),
    }
}

/// Struct for parsing string expressions to floats.
#[derive(Debug, Clone)]
pub struct Calculator {
    ///  HashMap of variables in current Calculator
    pub variables: HashMap<String, f64>,
}

/// Define the default value of Calculator.
impl Default for Calculator {
    fn default() -> Self {
        Self::new()
    }
}

impl Calculator {
    /// Create new Calculator.
    pub fn new() -> Self {
        Calculator {
            variables: HashMap::new(),
        }
    }
    /// Set variable for Calculator.
    ///
    /// # Arguments
    ///
    /// * `name` - Name of the variable
    /// * `value` - Float value of the variable
    ///
    pub fn set_variable(&mut self, name: &str, value: f64) {
        self.variables.insert(name.to_string(), value);
    }

    /// Get variable for Calculator.
    ///
    /// # Arguments
    ///
    /// * `name` - Name of the variable
    ///
    /// # Returns
    ///
    /// `value` - Result
    ///
    pub fn get_variable(&self, name: &str) -> Result<f64, CalculatorError> {
        Ok(*self
            .variables
            .get(name)
            .ok_or(CalculatorError::VariableNotSet {
                name: name.to_string(),
            })?)
    }

    ///  Parse a string expression.
    ///
    /// # Arguments
    ///
    /// * `expression` - Expression that is parsed
    ///
    pub fn parse_str(&self, expression: &str) -> Result<f64, CalculatorError> {
        let mut parser = ParserEnum::new_immutable(expression, self);
        let end_value = parser.evaluate_all_tokens()?;
        match end_value {
            None => Err(CalculatorError::NoValueReturnedParsing),
            Some(x) => Ok(x),
        }
    }

    ///  Parse a string expression allowing variable assignments.
    ///
    ///
    ///
    /// # Arguments
    ///
    /// * `expression` - Expression that is parsed
    ///
    pub fn parse_str_assign(&mut self, expression: &str) -> Result<f64, CalculatorError> {
        let mut parser = ParserEnum::new_mutable(expression, self);
        let end_value = parser.evaluate_all_tokens()?;
        match end_value {
            None => Err(CalculatorError::NoValueReturnedParsing),
            Some(x) => Ok(x),
        }
    }

    /// Parse a CalculatorFloat to float.
    ///
    /// # Arguments
    ///
    /// * `parse_variable` - Parsed string CalculatorFloat or returns float value
    ///
    pub fn parse_get(&self, parse_variable: CalculatorFloat) -> Result<f64, CalculatorError> {
        match parse_variable {
            CalculatorFloat::Float(x) => Ok(x),
            CalculatorFloat::Str(expression) => self.parse_str(&expression),
        }
    }
}

/// Enum combining different types of Tokens in an Expression.
#[derive(Debug, Clone, PartialEq)]
pub enum Token {
    /// A float or integer
    Number(f64),
    /// A variable
    Variable(String),
    /// A  known function
    Function(String),
    /// Plus
    Plus,
    /// Minus
    Minus,
    /// Multiply
    Multiply,
    /// Divice
    Divide,
    /// Poser
    Power,
    /// Factorial
    Factorial,
    /// DoubleFactorial
    DoubleFactorial,
    /// A bracket opening
    BracketOpen,
    /// A bracket closing
    BracketClose,
    /// Assign operator
    Assign,
    /// Assignment of a variable
    VariableAssign(String),
    /// Comma
    Comma,
    /// End of Expression
    EndOfExpression,
    /// End of parsed string
    EndOfString,
    /// No Token has been recognized in string
    Unrecognized,
}

/// Standard print implementation for Rust.
impl fmt::Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Token::Number(x) => write!(f, "Token::Number({x:e})"),
            Token::VariableAssign(y) => write!(f, "Token::VariableAssign({y})"),
            Token::Variable(y) => write!(f, "Token::Variable({y})"),
            Token::Function(y) => write!(f, "Token::Function({y})"),
            Token::Plus => write!(f, "Token::Plus"),
            Token::Minus => write!(f, "Token::Minus"),
            Token::Multiply => write!(f, "Token::Multiply"),
            Token::Divide => write!(f, "Token::Divide"),
            Token::Power => write!(f, "Token::Power"),
            Token::Factorial => write!(f, "Token::Factorial"),
            Token::DoubleFactorial => write!(f, "Token::DoubleFactorial"),
            Token::BracketOpen => write!(f, "Token::BracketOpen"),
            Token::BracketClose => write!(f, "Token::BracketClose"),
            Token::Assign => write!(f, "Token::Assign"),
            Token::Comma => write!(f, "Token::Comma"),
            Token::EndOfExpression => write!(f, "Token::EndOfExpression"),
            Token::EndOfString => write!(f, "Token::EndOfString"),
            Token::Unrecognized => write!(f, "Token::Unrecognized"),
        }
    }
}

/// Struct implementing Iterator trait to lex string
/// to computational Tokens.
pub struct TokenIterator<'a> {
    // Save current expression as a slice of a string so we do not
    // need to copy but only modify (shorten) the slice.
    //
    /// Current str expression being lexed
    pub current_expression: &'a str,
}

// Implement the Iterator Trait for TokenIterator so it can be used as standard rust iterator.
impl Iterator for TokenIterator<'_> {
    type Item = Token;

    // Define next method for Token iterator
    fn next(&mut self) -> Option<Token> {
        if self.current_expression.is_empty() {
            None
        } else {
            // Loop to remove whitespace and comments
            loop {
                if self.current_expression.starts_with(' ') {
                    let end = self
                        .current_expression
                        .char_indices()
                        .find_map(|(ind, c)| if c.is_whitespace() { None } else { Some(ind) })
                        .unwrap_or(self.current_expression.len());
                    self.cut_current_expression(end);
                    if self.current_expression.is_empty() {
                        return Some(Token::EndOfString);
                    }
                    continue;
                } else if self.current_expression.starts_with('#') {
                    let end = self
                        .current_expression
                        .char_indices()
                        .find_map(|(ind, c)| if c != '\u{000A}' { None } else { Some(ind + 1) })
                        .unwrap_or(self.current_expression.len());
                    self.cut_current_expression(end);
                    if self.current_expression.is_empty() {
                        return Some(Token::EndOfString);
                    }
                    continue;
                }
                break;
            }
            // Test if head of current_expression is a letter char
            if self
                .current_expression
                .chars()
                .next()
                .unwrap()
                .is_alphabetic()
            {
                // Find end of symbolic expression (not alphanumeric or '_')
                let end = self
                    .current_expression
                    .char_indices()
                    .find_map(|(ind, c)| {
                        if c.is_alphanumeric() || c == '_' {
                            None
                        } else {
                            Some(ind)
                        }
                    })
                    .unwrap_or(self.current_expression.len());
                // Get next token from TokenIterator with shortened expression
                let next_token = if end >= self.current_expression.len() {
                    TokenIterator {
                        current_expression: "",
                    }
                    .next()
                } else {
                    TokenIterator {
                        current_expression: &self.current_expression[end..],
                    }
                    .next()
                };
                // Depending on next token currently lexed string current_expression[..end] creates different tokens
                // Token contains current_expression[..end] for later processing
                return Some(match next_token {
                    Some(Token::Assign) => {
                        let vs = self.current_expression[..end].to_owned();
                        self.cut_current_expression(end + 1);
                        Token::VariableAssign(vs)
                    }
                    Some(Token::BracketOpen) => {
                        let vs = self.current_expression[..end].to_owned();
                        self.cut_current_expression(end + 1);
                        Token::Function(vs)
                    }
                    _ => {
                        let vs = self.current_expression[..end].to_owned();
                        self.cut_current_expression(end);
                        Token::Variable(vs)
                    }
                });
            }
            // Lex string that contains a number.
            // Test if current expression starts with ascii number
            if self
                .current_expression
                .chars()
                .next()
                .unwrap()
                .is_ascii_digit()
                || self.current_expression.starts_with('.')
            {
                // find end of number expression
                let (end, next_char) = self
                    .current_expression
                    .char_indices()
                    .find(|(_, c)| !c.is_ascii_digit() && c != &'.')
                    .unwrap_or((self.current_expression.len(), ' '));
                let mut end_offset = 0;
                let mut start: usize = 0;
                // Handle scientific notation.
                // Starts with e or E for scientific notation
                if next_char == 'e' || next_char == 'E' {
                    // offset for just 'e' or 'E'
                    start = 1;
                    if self
                        .current_expression
                        .chars()
                        .nth(end + start)
                        .unwrap_or(' ')
                        == '+'
                        || self
                            .current_expression
                            .chars()
                            .nth(end + start)
                            .unwrap_or(' ')
                            == '-'
                    {
                        // offset if exponent has sign
                        start = 2;
                    }
                    // Find end of exponent
                    end_offset = self.current_expression[end + start..]
                        .char_indices()
                        .find_map(|(ind, c)| if c.is_ascii_digit() { None } else { Some(ind) })
                        .unwrap_or(self.current_expression.len() - (end + start));
                }
                let end_total = end + start + end_offset;
                let number_expression = &self.current_expression[..end_total];
                // Use inbuilt rust string -> number conversion to get number and handle errors
                self.cut_current_expression(end_total);
                return Some(match f64::from_str(number_expression) {
                    Err(_) => Token::Unrecognized,
                    Ok(f) => Token::Number(f.to_owned()),
                });
            };
            // Create symbol tokens
            let symbol = self.current_expression.chars().next().unwrap();
            self.current_expression = &self.current_expression[1..];
            Some(match symbol {
                '+' => Token::Plus,
                '-' => Token::Minus,
                '*' => match self.current_expression.chars().next().unwrap_or(' ') {
                    '*' => {
                        self.current_expression = &self.current_expression[1..];
                        Token::Power
                    }
                    _ => Token::Multiply,
                },
                '/' => Token::Divide,
                '^' => Token::Power,
                '(' => Token::BracketOpen,
                ')' => Token::BracketClose,
                '=' => Token::Assign,
                ',' => Token::Comma,
                ';' => Token::EndOfExpression,
                '!' => match self.current_expression.chars().next().unwrap_or(' ') {
                    '!' => {
                        self.current_expression = &self.current_expression[1..];
                        Token::DoubleFactorial
                    }
                    _ => Token::Factorial,
                },
                _ => Token::Unrecognized,
            })
        }
    }
}

// Helper methods not in standard iterator trait.
impl<'a> TokenIterator<'a> {
    // Return the next token and the current token (in string form).
    fn next_token_and_str(&mut self) -> (Option<Token>, &'a str) {
        let next_token = self.next();
        let next_str = self.current_expression;
        (next_token, next_str)
    }

    // Modify the current expression to be a slice of the current expression.
    fn cut_current_expression(&mut self, end: usize) {
        if end == self.current_expression.len() {
            self.current_expression = "";
        } else {
            self.current_expression = &self.current_expression[end..];
        }
    }
}

// /// Parser from &str to f64 using TokenIterator lexer.
// struct Parser<'a> {
//     /// Expression that has not been parsed yet
//     remaining_expression: &'a str,
//     /// Token that is currently parsed
//     current_token: Token,
//     /// Calculator that contains set variables
//     calculator: &'a mut Calculator,
// }

/// Parser from &str to f64 using TokenIterator lexer.
enum ParserEnum<'a> {
    MutableCalculator {
        /// Expression that has not been parsed yet
        remaining_expression: &'a str,
        /// Token that is currently parsed
        current_token: Token,
        /// Calculator that contains set variables
        calculator: &'a mut Calculator,
    },
    ImmutableCalculator {
        /// Expression that has not been parsed yet
        remaining_expression: &'a str,
        /// Token that is currently parsed
        current_token: Token,
        /// Calculator that contains set variables
        calculator: &'a Calculator,
    },
}

impl<'a, 'b> ParserEnum<'a>
where
    'b: 'a,
{
    /// Get variable for Calculator.
    ///
    /// # Arguments
    ///
    /// * `name` - Name of the variable
    ///
    /// # Returns
    ///
    /// `value` - Result
    ///
    #[inline]
    pub fn get_variable(&self, name: &str) -> Result<f64, CalculatorError> {
        match self {
            Self::MutableCalculator { calculator, .. } => calculator.get_variable(name),
            Self::ImmutableCalculator { calculator, .. } => calculator.get_variable(name),
        }
    }

    /// Set variable for Calculator.
    ///
    /// # Arguments
    ///
    /// * `name` - Name of the variable
    /// * `value` - Float value of the variable
    #[inline]
    pub fn set_variable(&mut self, name: &str, value: f64) -> Result<(), CalculatorError> {
        match self {
            Self::MutableCalculator { calculator, .. } => calculator.set_variable(name, value),
            Self::ImmutableCalculator { .. } => {
                return Err(CalculatorError::ParsingError {
                    msg: "Assign operation not allowed when using immutable Calculator",
                })
            }
        }
        Ok(())
    }

    fn new_mutable(expression: &'a str, calculator: &'b mut Calculator) -> Self {
        let (next_token, next_str) = (TokenIterator {
            current_expression: expression,
        })
        .next_token_and_str();
        ParserEnum::MutableCalculator {
            remaining_expression: next_str,
            current_token: next_token.unwrap(),
            calculator,
        }
    }

    fn new_immutable(expression: &'a str, calculator: &'b Calculator) -> Self {
        let (next_token, next_str) = (TokenIterator {
            current_expression: expression,
        })
        .next_token_and_str();
        ParserEnum::ImmutableCalculator {
            remaining_expression: next_str,
            current_token: next_token.unwrap(),
            calculator,
        }
    }

    fn remaining_expression(&mut self) -> &'a str {
        match self {
            ParserEnum::MutableCalculator {
                remaining_expression,
                ..
            } => remaining_expression,
            ParserEnum::ImmutableCalculator {
                remaining_expression,
                ..
            } => remaining_expression,
        }
    }

    fn current_token(&self) -> &Token {
        match self {
            ParserEnum::MutableCalculator { current_token, .. } => current_token,
            ParserEnum::ImmutableCalculator { current_token, .. } => current_token,
        }
    }

    /// Get next token via TokenIterator.
    fn next_token(&mut self) {
        let (next_token, next_str) = (TokenIterator {
            current_expression: self.remaining_expression(),
        })
        .next_token_and_str();
        match next_token {
            None => match self {
                ParserEnum::MutableCalculator {
                    remaining_expression,
                    current_token,
                    ..
                } => {
                    *current_token = Token::EndOfString;
                    *remaining_expression = "";
                }
                ParserEnum::ImmutableCalculator {
                    remaining_expression,
                    current_token,
                    ..
                } => {
                    *current_token = Token::EndOfString;
                    *remaining_expression = "";
                }
            },
            Some(t) => match self {
                ParserEnum::MutableCalculator {
                    remaining_expression,
                    current_token,
                    ..
                } => {
                    *current_token = t;
                    *remaining_expression = next_str;
                }
                ParserEnum::ImmutableCalculator {
                    remaining_expression,
                    current_token,
                    ..
                } => {
                    *current_token = t;
                    *remaining_expression = next_str;
                }
            },
        }
    }

    /// Evaluate all Tokens to real value, None (for not returning expressions)
    /// or return error.
    fn evaluate_all_tokens(&mut self) -> Result<Option<f64>, CalculatorError> {
        let mut current_value: Option<f64> = None;
        while self.current_token() != &Token::EndOfString {
            current_value = self.evaluate_init()?;
            while self.current_token() == &Token::EndOfExpression {
                self.next_token();
            }
        }
        Ok(current_value)
    }

    /// Initialize the evaluation of an expression.
    fn evaluate_init(&mut self) -> Result<Option<f64>, CalculatorError> {
        if self.current_token() == &Token::EndOfExpression
            || self.current_token() == &Token::EndOfString
        {
            Err(CalculatorError::UnexpectedEndOfExpression)
        } else {
            if let Token::VariableAssign(ref vs) = (*self).current_token() {
                match self {
                    ParserEnum::MutableCalculator { .. } => (),
                    ParserEnum::ImmutableCalculator { .. } => {
                        return Err(CalculatorError::ForbiddenAssign {
                            variable_name: vs.to_owned(),
                        })
                    }
                }
                let vsnew = vs.to_owned();
                self.next_token();
                let res = self.evaluate_binary_1()?;
                self.set_variable(&vsnew, res)?;
                return Ok(Some(res));
            }
            Ok(Some(self.evaluate_binary_1()?))
        }
    }

    /// Evaluate least preference binary expression (+, -).
    fn evaluate_binary_1(&mut self) -> Result<f64, CalculatorError> {
        let mut res = self.evaluate_binary_2()?;
        while self.current_token() == &Token::Plus || self.current_token() == &Token::Minus {
            let bsum: bool = self.current_token() == &Token::Plus;
            self.next_token();
            let val = self.evaluate_binary_2()?;
            if bsum {
                res += val;
            } else {
                res -= val;
            }
        }
        Ok(res)
    }

    /// Evaluate middle preference binary expression (*, /).
    fn evaluate_binary_2(&mut self) -> Result<f64, CalculatorError> {
        let mut res = self.evaluate_binary_3()?;
        while self.current_token() == &Token::Multiply || self.current_token() == &Token::Divide {
            let bmul: bool = self.current_token() == &Token::Multiply;
            self.next_token();
            let val = self.evaluate_binary_3()?;
            if bmul {
                res *= val;
            } else {
                if val == 0.0 {
                    return Err(CalculatorError::DivisionByZero);
                }
                res /= val;
            }
        }
        Ok(res)
    }

    /// Evaluate least preference binary expression (^, !).
    fn evaluate_binary_3(&mut self) -> Result<f64, CalculatorError> {
        let mut res = self.evaluate_unary()?;
        match self.current_token() {
            Token::DoubleFactorial => {
                return Err(CalculatorError::NotImplementedError {
                    fct: "DoubleFactorial",
                })
            }
            Token::Factorial => {
                return Err(CalculatorError::NotImplementedError { fct: "Factorial" })
            }
            Token::Power => {
                self.next_token();
                res = res.powf(self.evaluate_unary()?);
            }
            _ => (),
        }
        Ok(res)
    }

    /// Handle any unary + or - signs.
    fn evaluate_unary(&mut self) -> Result<f64, CalculatorError> {
        let mut prefactor: f64 = 1.0;
        match self.current_token() {
            Token::Minus => {
                self.next_token();
                prefactor = -1.0;
            }
            Token::Plus => {
                self.next_token();
            }
            _ => (),
        }
        Ok(prefactor * self.evaluate()?)
    }

    /// Handle numbers, variables, functions and parentheses.
    fn evaluate(&mut self) -> Result<f64, CalculatorError> {
        match self.current_token().clone() {
            Token::BracketOpen => {
                self.next_token();
                let res_init = self.evaluate_init()?.ok_or(CalculatorError::ParsingError {
                    msg: "Unexpected None return",
                })?;
                //self.next_token()?;
                if self.current_token() != &Token::BracketClose {
                    Err(CalculatorError::ParsingError {
                        msg: "Expected Braket close",
                    })
                } else {
                    self.next_token();
                    Ok(res_init)
                }
            }
            Token::Number(vf) => {
                self.next_token();
                Ok(vf)
            }
            Token::Variable(ref vs) => {
                let vsnew = vs.to_owned();
                self.next_token();
                self.get_variable(&vsnew)
            }
            Token::Function(ref vs) => {
                let vsnew = vs.to_owned();
                self.next_token();
                let mut heap = Vec::new();
                let number_arguments = function_argument_numbers(&vsnew)?;
                for argument_number in 0..number_arguments {
                    heap.push(
                        self.evaluate_init()?
                            .ok_or(CalculatorError::NoValueReturnedParsing)?,
                    );
                    // Swallow commas in function arguments
                    if argument_number < number_arguments - 1 {
                        if self.current_token() != &Token::Comma {
                            return Err(CalculatorError::ParsingError {
                                msg: "expected comma in function arguments",
                            });
                        } else {
                            self.next_token();
                        }
                    }
                    //self.next_token()?;
                }
                if self.current_token() != &Token::BracketClose {
                    return Err(CalculatorError::ParsingError {
                        msg: "Expected braket close.",
                    });
                }
                self.next_token();
                match number_arguments {
                    1 => function_1_argument(
                        &vsnew,
                        *(heap
                            .first()
                            .ok_or(CalculatorError::NotEnoughFunctionArguments)?),
                    ),
                    2 => function_2_arguments(
                        &vsnew,
                        *(heap
                            .first()
                            .ok_or(CalculatorError::NotEnoughFunctionArguments)?),
                        *(heap
                            .get(1)
                            .ok_or(CalculatorError::NotEnoughFunctionArguments)?),
                    ),
                    _ => Err(CalculatorError::ParsingError {
                        msg: "Unsupported number of arguments.",
                    }),
                }
            }
            _ => Err(CalculatorError::ParsingError {
                msg: "Bad_Position",
            }),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::function_1_argument;
    use super::function_2_arguments;
    use super::function_argument_numbers;
    use super::Calculator;
    use super::CalculatorFloat;
    use super::Token;
    use super::TokenIterator;

    // Test the next function of the TokenIterator for an end of string Token
    #[test]
    fn test_end_of_string() {
        let mut t_iterator = TokenIterator {
            current_expression: " ",
        };
        assert_eq!(t_iterator.next().unwrap(), Token::EndOfString);
        let mut t_iterator2 = TokenIterator {
            current_expression: "#aaabbb+.3+.4e-10*5-1; ",
        };
        assert_eq!(t_iterator2.next().unwrap(), Token::EndOfString);
    }

    // Test the next function of the TokenIterator for a plus/minus Token
    #[test]
    fn test_plus_minus() {
        let mut t_iterator = TokenIterator {
            current_expression: " +",
        };
        assert_eq!(t_iterator.next().unwrap(), Token::Plus);
        let mut t_iterator2 = TokenIterator {
            current_expression: "#a \n +",
        };
        assert_eq!(t_iterator2.next().unwrap(), Token::Plus);
        let mut t_iterator3 = TokenIterator {
            current_expression: "-",
        };
        assert_eq!(t_iterator3.next().unwrap(), Token::Minus);
    }

    // Test the next function of the TokenIterator for a number Token
    #[test]
    fn test_number() {
        let mut t_iterator = TokenIterator {
            current_expression: "1e0",
        };
        assert_eq!(t_iterator.next().unwrap(), Token::Number(1.0));
        let mut t_iterator2 = TokenIterator {
            current_expression: "(1+2e0)",
        };
        assert_eq!(t_iterator2.nth(3).unwrap(), Token::Number(2.0));
        let mut t_iterator3 = TokenIterator {
            current_expression: "2e+10",
        };
        assert_eq!(t_iterator3.next().unwrap(), Token::Number(2.0e+10));
        let mut t_iterator4 = TokenIterator {
            current_expression: "1.74E-10",
        };
        assert_eq!(t_iterator4.next().unwrap(), Token::Number(1.74E-10));
    }

    // Test the next function of the TokenIterator for a multiply Token
    #[test]
    fn test_multiply() {
        let mut t_iterator = TokenIterator {
            current_expression: " *",
        };
        assert_eq!(t_iterator.next().unwrap(), Token::Multiply);
    }

    // Test the next function of the TokenIterator for a divide Token
    #[test]
    fn test_divide() {
        let mut t_iterator = TokenIterator {
            current_expression: " /",
        };
        assert_eq!(t_iterator.next().unwrap(), Token::Divide);
    }

    // Test the next function of the TokenIterator for a power (^ and **) Token
    #[test]
    fn test_power() {
        let mut t_iterator = TokenIterator {
            current_expression: " **",
        };
        assert_eq!(t_iterator.next().unwrap(), Token::Power);
        let mut t_iterator2 = TokenIterator {
            current_expression: " ^",
        };
        assert_eq!(t_iterator2.next().unwrap(), Token::Power);
    }

    // Test the next function of the TokenIterator for a bracket (open and close) Token
    #[test]
    fn test_brackets() {
        let mut t_iterator = TokenIterator {
            current_expression: " (",
        };
        assert_eq!(t_iterator.next().unwrap(), Token::BracketOpen);
        let mut t_iterator = TokenIterator {
            current_expression: ") ",
        };
        assert_eq!(t_iterator.next().unwrap(), Token::BracketClose);
    }

    // Test the next function of the TokenIterator for an assign Token
    #[test]
    fn test_assign() {
        let mut t_iterator = TokenIterator {
            current_expression: " =",
        };
        assert_eq!(t_iterator.next().unwrap(), Token::Assign);
    }

    // Test the next function of the TokenIterator for a comma Token
    #[test]
    fn test_comma() {
        let mut t_iterator = TokenIterator {
            current_expression: ", ",
        };
        assert_eq!(t_iterator.next().unwrap(), Token::Comma);
    }

    // Test the next function of the TokenIterator for a semi-colon Token
    #[test]
    fn test_semi_colon() {
        let mut t_iterator = TokenIterator {
            current_expression: ";",
        };
        assert_eq!(t_iterator.next().unwrap(), Token::EndOfExpression);
    }

    // Test the next function of the TokenIterator for a (double) factorial Token
    #[test]
    fn test_factorial() {
        let mut t_iterator = TokenIterator {
            current_expression: "!",
        };
        assert_eq!(t_iterator.next().unwrap(), Token::Factorial);
        let mut t_iterator = TokenIterator {
            current_expression: "!!",
        };
        assert_eq!(t_iterator.next().unwrap(), Token::DoubleFactorial);
    }

    // Test the next function of the TokenIterator for an unrecognised Token
    #[test]
    fn test_unrecognized() {
        let mut t_iterator = TokenIterator {
            current_expression: "|",
        };
        assert_eq!(t_iterator.next().unwrap(), Token::Unrecognized);
    }

    // Test the next function of the TokenIterator for a variable Token
    #[test]
    fn test_variable() {
        let mut t_iterator = TokenIterator {
            current_expression: "test",
        };
        let next_token = t_iterator.next().expect("next token throws error");
        assert_eq!(next_token, Token::Variable("test".to_owned()));
        let mut t_iterator = TokenIterator {
            current_expression: "test;",
        };
        let next_token = t_iterator.next().expect("next token throws error");
        assert_eq!(next_token, Token::Variable("test".to_owned()));
        let mut t_iterator = TokenIterator {
            current_expression: "test+",
        };
        let next_token = t_iterator.next().expect("next token throws error");
        assert_eq!(next_token, Token::Variable("test".to_owned()));
    }

    // Test the next function of the TokenIterator for a variable assign Token
    #[test]
    fn test_variable_assign() {
        let mut t_iterator = TokenIterator {
            current_expression: "test=",
        };
        let next_token = t_iterator.next().expect("next token throws error");
        assert_eq!(next_token, Token::VariableAssign("test".to_owned()));
    }

    // Test the next function of the TokenIterator for a function Token
    #[test]
    fn test_functions() {
        let mut t_iterator = TokenIterator {
            current_expression: "test(",
        };
        let next_token = t_iterator.next().expect("next token throws error");
        assert_eq!(next_token, Token::Function("test".to_owned()));
    }

    // Test the default function of Calculator
    #[test]
    fn test_calculator_new() {
        let _calculator = Calculator::new();
    }

    #[test]
    fn test_calculator_default() {
        let _calculator = Calculator::default();
        assert_eq!(_calculator.variables, Calculator::new().variables);
    }

    // Test the Debug macro for Calculator
    #[test]
    fn test_calculator_debug() {
        let mut calculator = Calculator::new();
        calculator.set_variable("x", 0.1);
        assert_eq!(
            format!("{calculator:?}"),
            "Calculator { variables: {\"x\": 0.1} }"
        );
    }

    // Test the Clone macro for Calculator
    #[test]
    fn test_calculator_clone() {
        let mut calculator = Calculator::new();
        calculator.set_variable("x", 0.1);
        let c_clone = calculator.clone();
        assert_eq!(c_clone.get_variable("x").unwrap(), 0.1);
        assert_eq!(calculator.variables, c_clone.variables);
    }

    // Test set_variable function
    #[test]
    fn test_set_value() {
        let mut calculator = Calculator::new();
        calculator.set_variable("test", 0.1);
        assert_eq!(*calculator.variables.get("test").unwrap(), 0.1);
    }

    // Test get_variable function (float and error return)
    #[test]
    fn test_get_value() {
        let mut calculator = Calculator::new();
        calculator.set_variable("test", 0.1);
        assert_eq!(calculator.get_variable("test").unwrap(), 0.1);
        assert!(calculator.get_variable("test2").is_err());
    }

    // Test parse_string for a variable Token
    #[test]
    fn test_parse_variable() {
        let calculator = Calculator::new();
        let value = calculator.parse_str("a=3; 2*(a+1);");
        assert!(value.is_err());
        let value = calculator.parse_str("2*(3+1);");
        assert_eq!(value.unwrap(), 8.0);
    }

    // Test parse_string for a variable Token
    #[test]
    fn test_parse_assign_variable() {
        let mut calculator = Calculator::new();
        let value = calculator.parse_str_assign("a=3; 2*(a+1);");
        assert_eq!(value.unwrap(), 8.0);
        assert_eq!(calculator.get_variable("a").unwrap(), 3.0);
    }

    // Test parse_string for a variable Token with an underscore in it
    #[test]
    fn test_parse_variable_underscore() {
        let mut calculator = Calculator::new();
        let value = calculator.parse_str_assign("a_1=3; 2*(a_1+1);");
        assert_eq!(value.unwrap(), 8.0);
        assert_eq!(calculator.get_variable("a_1").unwrap(), 3.0);
    }

    // Test parse_string for a function Token
    #[test]
    fn test_parse_assing_function() {
        let f: f64 = 4.0;
        let mut calculator = Calculator::new();
        let value = calculator.parse_str_assign("a=3; sin(a+1);");
        assert_eq!(value.unwrap(), f.sin());
        assert_eq!(calculator.get_variable("a").unwrap(), 3.0);

        let value = calculator.parse_str_assign("atan2(a+1,2e0);");
        assert_eq!(value.unwrap(), f.atan2(2e0));
        assert_eq!(calculator.get_variable("a").unwrap(), 3.0);
    }

    // Test parse_get function
    #[test]
    fn test_parse_get() {
        let calculator = Calculator::new();

        let cf_float = CalculatorFloat::from(3.0);
        let value_cf_float = calculator.parse_get(cf_float);
        assert_eq!(value_cf_float.unwrap(), 3.0);

        let cf_string = CalculatorFloat::from("3+0");
        let value_cf_string = calculator.parse_get(cf_string);
        assert_eq!(value_cf_string.unwrap(), 3.0);
    }

    // Test that all evaluate functions match statements return the expected float/error
    #[test]
    fn test_evaluate_functions() {
        let calculator = Calculator::new();

        // Evaluate unary function: + and -
        let value = calculator.parse_str("-2");
        assert_eq!(value.unwrap(), -2.0);
        let value = calculator.parse_str("+2");
        assert_eq!(value.unwrap(), 2.0);

        // Evaluate binary3 function: **/^, ! and !!
        let value = calculator.parse_str("2^3");
        assert_eq!(value.unwrap(), 8.0);
        let value = calculator.parse_str("2**3");
        assert_eq!(value.unwrap(), 8.0);
        let value = calculator.parse_str("3!");
        assert!(value.is_err());
        let value = calculator.parse_str("3!!");
        assert!(value.is_err());

        // Evaluate binary2 function: * and /
        let value = calculator.parse_str("2*3");
        assert_eq!(value.unwrap(), 6.0);
        let value = calculator.parse_str("3/2");
        assert_eq!(value.unwrap(), 1.5);
        let value = calculator.parse_str("3/0");
        assert!(value.is_err());

        // Evaluate binary1 function: + and -
        let value = calculator.parse_str("1+1");
        assert_eq!(value.unwrap(), 2.0);
        let value = calculator.parse_str("1-1");
        assert_eq!(value.unwrap(), 0.0);

        // Evaluate initialization function
        let value = calculator.parse_str(";3");
        assert!(value.is_err());

        // Evaluate function
        let value = calculator.parse_str("(3");
        assert!(value.is_err());
        let value = calculator.parse_str("(3)");
        assert_eq!(value.unwrap(), 3.0);
        let value = calculator.parse_str("3");
        assert_eq!(value.unwrap(), 3.0);
        let value = calculator.parse_str("x=3; x+2");
        assert!(value.is_err());
        let value = calculator.parse_str("max(3, 2)");
        assert_eq!(value.unwrap(), 3.0);
        let value = calculator.parse_str("max(3 2)");
        assert!(value.is_err());
        let value = calculator.parse_str("max(3, 2");
        assert!(value.is_err());
        let value = calculator.parse_str(")");
        assert!(value.is_err());
    }

    // Testing that all functions get matched with the correct nummber of arguments (1 or 2)
    #[test]
    fn test_function_argument_numbers() {
        assert_eq!(function_argument_numbers("sin").unwrap(), 1);
        assert_eq!(function_argument_numbers("cos").unwrap(), 1);
        assert_eq!(function_argument_numbers("abs").unwrap(), 1);
        assert_eq!(function_argument_numbers("tan").unwrap(), 1);
        assert_eq!(function_argument_numbers("acos").unwrap(), 1);
        assert_eq!(function_argument_numbers("asin").unwrap(), 1);
        assert_eq!(function_argument_numbers("atan").unwrap(), 1);
        assert_eq!(function_argument_numbers("cosh").unwrap(), 1);
        assert_eq!(function_argument_numbers("sinh").unwrap(), 1);
        assert_eq!(function_argument_numbers("tanh").unwrap(), 1);
        assert_eq!(function_argument_numbers("acosh").unwrap(), 1);
        assert_eq!(function_argument_numbers("asinh").unwrap(), 1);
        assert_eq!(function_argument_numbers("atanh").unwrap(), 1);
        assert_eq!(function_argument_numbers("arcosh").unwrap(), 1);
        assert_eq!(function_argument_numbers("arsinh").unwrap(), 1);
        assert_eq!(function_argument_numbers("artanh").unwrap(), 1);
        assert_eq!(function_argument_numbers("exp").unwrap(), 1);
        assert_eq!(function_argument_numbers("exp2").unwrap(), 1);
        assert_eq!(function_argument_numbers("expm1").unwrap(), 1);
        assert_eq!(function_argument_numbers("log").unwrap(), 1);
        assert_eq!(function_argument_numbers("log10").unwrap(), 1);
        assert_eq!(function_argument_numbers("sqrt").unwrap(), 1);
        assert_eq!(function_argument_numbers("cbrt").unwrap(), 1);
        assert_eq!(function_argument_numbers("ceil").unwrap(), 1);
        assert_eq!(function_argument_numbers("floor").unwrap(), 1);
        assert_eq!(function_argument_numbers("fract").unwrap(), 1);
        assert_eq!(function_argument_numbers("round").unwrap(), 1);
        assert_eq!(function_argument_numbers("erf").unwrap(), 1);
        assert_eq!(function_argument_numbers("tgamma").unwrap(), 1);
        assert_eq!(function_argument_numbers("lgamma").unwrap(), 1);
        assert_eq!(function_argument_numbers("sign").unwrap(), 1);
        assert_eq!(function_argument_numbers("delta").unwrap(), 1);
        assert_eq!(function_argument_numbers("theta").unwrap(), 1);
        assert_eq!(function_argument_numbers("parity").unwrap(), 1);
        assert_eq!(function_argument_numbers("atan2").unwrap(), 2);
        assert_eq!(function_argument_numbers("hypot").unwrap(), 2);
        assert_eq!(function_argument_numbers("pow").unwrap(), 2);
        assert_eq!(function_argument_numbers("max").unwrap(), 2);
        assert_eq!(function_argument_numbers("min").unwrap(), 2);
        assert!(function_argument_numbers("test").is_err());
    }

    // Testing that all functions with 1 argument get matched with the correct Rust function
    #[test]
    fn test_function_1_argument() {
        let f: f64 = 0.1;
        let f1: f64 = 1.5;
        assert_eq!(function_1_argument("sin", 0.1).unwrap(), f.sin());
        assert_eq!(function_1_argument("cos", 0.1).unwrap(), f.cos());
        assert_eq!(function_1_argument("abs", 0.1).unwrap(), f.abs());
        assert_eq!(function_1_argument("tan", 0.1).unwrap(), f.tan());
        assert_eq!(function_1_argument("acos", 0.1).unwrap(), f.acos());
        assert_eq!(function_1_argument("asin", 0.1).unwrap(), f.asin());
        assert_eq!(function_1_argument("atan", 0.1).unwrap(), f.atan());
        assert_eq!(function_1_argument("cosh", 0.1).unwrap(), f.cosh());
        assert_eq!(function_1_argument("sinh", 0.1).unwrap(), f.sinh());
        assert_eq!(function_1_argument("tanh", 0.1).unwrap(), f.tanh());
        assert_eq!(function_1_argument("acosh", 1.5).unwrap(), f1.acosh());
        assert_eq!(function_1_argument("asinh", 0.1).unwrap(), f.asinh());
        assert_eq!(function_1_argument("atanh", 0.1).unwrap(), f.atanh());
        assert_eq!(function_1_argument("arcosh", 1.5).unwrap(), f1.acosh());
        assert_eq!(function_1_argument("arsinh", 0.1).unwrap(), f.asinh());
        assert_eq!(function_1_argument("artanh", 0.1).unwrap(), f.atanh());
        assert_eq!(function_1_argument("exp", 0.1).unwrap(), f.exp());
        assert_eq!(function_1_argument("exp2", 0.1).unwrap(), f.exp2());
        assert_eq!(function_1_argument("expm1", 0.1).unwrap(), f.exp_m1());
        assert_eq!(function_1_argument("log", 0.1).unwrap(), f.ln());
        assert_eq!(function_1_argument("log10", 0.1).unwrap(), f.log10());
        assert_eq!(function_1_argument("sqrt", 0.1).unwrap(), f.sqrt());
        assert_eq!(function_1_argument("cbrt", 0.1).unwrap(), f.cbrt());
        assert_eq!(function_1_argument("ceil", 0.1).unwrap(), f.ceil());
        assert_eq!(function_1_argument("floor", 0.1).unwrap(), f.floor());
        assert_eq!(function_1_argument("fract", 0.1).unwrap(), f.fract());
        assert_eq!(function_1_argument("round", 0.1).unwrap(), f.round());
        assert_eq!(function_1_argument("sign", 0.1).unwrap(), f.signum());
        assert_eq!(function_1_argument("delta", 0.0).unwrap(), 1.0);
        assert_eq!(function_1_argument("delta", 0.1).unwrap(), 0.0);
        assert_eq!(function_1_argument("theta", 0.0).unwrap(), 0.5);
        assert_eq!(function_1_argument("theta", -0.1).unwrap(), 0.0);
        assert_eq!(function_1_argument("theta", 0.1).unwrap(), 1.0);
        assert!(function_1_argument("test", 1.0).is_err());
    }

    // Testing that all functions with 2 arguments get matched with the correct Rust function
    #[test]
    fn test_function_2_argument() {
        let f: f64 = 0.1;
        assert_eq!(
            function_2_arguments("atan2", 0.1, 0.2).unwrap(),
            f.atan2(0.2)
        );
        assert_eq!(
            function_2_arguments("hypot", 0.1, 0.2).unwrap(),
            f.hypot(0.2)
        );
        assert_eq!(function_2_arguments("pow", 0.1, 0.2).unwrap(), f.powf(0.2));
        assert_eq!(function_2_arguments("max", 0.1, 0.2).unwrap(), f.max(0.2));
        assert_eq!(function_2_arguments("min", 0.1, 0.2).unwrap(), f.min(0.2));
        assert!(function_2_arguments("test", 1.0, 1.0).is_err());
    }

    // Testing display function for all possible inputs
    #[test]
    fn test_display() {
        let f = Token::Number(0.1);
        let f_formatted = format!("{f}");
        assert_eq!(f_formatted, "Token::Number(1e-1)");

        let f = Token::VariableAssign(String::from("x"));
        let f_formatted = format!("{f}");
        assert_eq!(f_formatted, "Token::VariableAssign(x)");

        let f = Token::Variable(String::from("3t"));
        let f_formatted = format!("{f}");
        assert_eq!(f_formatted, "Token::Variable(3t)");

        let f = Token::Function(String::from("2s"));
        let f_formatted = format!("{f}");
        assert_eq!(f_formatted, "Token::Function(2s)");

        let f = Token::Plus;
        let f_formatted = format!("{f}");
        assert_eq!(f_formatted, "Token::Plus");

        let f = Token::Minus;
        let f_formatted = format!("{f}");
        assert_eq!(f_formatted, "Token::Minus");

        let f = Token::Multiply;
        let f_formatted = format!("{f}");
        assert_eq!(f_formatted, "Token::Multiply");

        let f = Token::Divide;
        let f_formatted = format!("{f}");
        assert_eq!(f_formatted, "Token::Divide");

        let f = Token::Power;
        let f_formatted = format!("{f}");
        assert_eq!(f_formatted, "Token::Power");

        let f = Token::Factorial;
        let f_formatted = format!("{f}");
        assert_eq!(f_formatted, "Token::Factorial");

        let f = Token::DoubleFactorial;
        let f_formatted = format!("{f}");
        assert_eq!(f_formatted, "Token::DoubleFactorial");

        let f = Token::BracketOpen;
        let f_formatted = format!("{f}");
        assert_eq!(f_formatted, "Token::BracketOpen");

        let f = Token::BracketClose;
        let f_formatted = format!("{f}");
        assert_eq!(f_formatted, "Token::BracketClose");

        let f = Token::Assign;
        let f_formatted = format!("{f}");
        assert_eq!(f_formatted, "Token::Assign");

        let f = Token::Comma;
        let f_formatted = format!("{f}");
        assert_eq!(f_formatted, "Token::Comma");

        let f = Token::EndOfExpression;
        let f_formatted = format!("{f}");
        assert_eq!(f_formatted, "Token::EndOfExpression");

        let f = Token::EndOfString;
        let f_formatted = format!("{f}");
        assert_eq!(f_formatted, "Token::EndOfString");

        let f = Token::Unrecognized;
        let f_formatted = format!("{f}");
        assert_eq!(f_formatted, "Token::Unrecognized");
    }
}
// End of tests
