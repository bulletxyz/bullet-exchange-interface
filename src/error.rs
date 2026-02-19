use serde::Serialize;
use thiserror::Error;

#[derive(Debug, Error, Serialize)]
#[serde(tag = "code", rename_all = "snake_case")]
pub enum ConfigError {
    #[error("Failed to parse input: {input} - {reason}")]
    FailedToParseInput { input: String, reason: String },
}

/// Arithmetic operation errors with detailed context
#[derive(Debug, Error, Serialize)]
#[serde(tag = "code", rename_all = "snake_case")]
pub enum ArithmeticError {
    #[error("Decimal arithmetic failed: {operation} with {left} and {right}")]
    DecimalFailed {
        operation: ArithmeticOperation,
        left: rust_decimal::Decimal,
        right: rust_decimal::Decimal,
    },

    #[error("Integer arithmetic failed: {operation} with {left} and {right}")]
    IntegerFailed {
        operation: ArithmeticOperation,
        left: i128,
        right: i128,
    },
}

/// Types of arithmetic operations
#[derive(Debug, Clone, PartialEq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum ArithmeticOperation {
    Addition,
    Subtraction,
    Multiplication,
    Division,
    Exponentiation,
}

impl std::fmt::Display for ArithmeticOperation {
    fn fmt(&self, fmt: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        match self {
            Self::Addition => write!(fmt, "Addition"),
            Self::Subtraction => write!(fmt, "Subtraction"),
            Self::Multiplication => write!(fmt, "Multiplication"),
            Self::Division => write!(fmt, "Division"),
            Self::Exponentiation => write!(fmt, "Exponentiation"),
        }
    }
}
