use std::str::FromStr;

use rust_decimal::{Decimal, MathematicalOps};

use crate::error::{ArithmeticError, ArithmeticOperation, ConfigError};

pub trait TryDecimalOps {
    fn try_from_str(value: &str) -> Result<Self, ConfigError>
    where
        Self: Sized;
    fn try_add(self, v: impl Into<Decimal>) -> Result<Self, ArithmeticError>
    where
        Self: Sized;
    fn try_sub(self, v: impl Into<Decimal>) -> Result<Self, ArithmeticError>
    where
        Self: Sized;
    fn try_mul(self, v: impl Into<Decimal>) -> Result<Self, ArithmeticError>
    where
        Self: Sized;
    fn try_div(self, v: impl Into<Decimal>) -> Result<Self, ArithmeticError>
    where
        Self: Sized;
    fn try_exp(self) -> Result<Self, ArithmeticError>
    where
        Self: Sized;
}

impl TryDecimalOps for Decimal {
    #[inline]
    fn try_from_str(value: &str) -> Result<Self, ConfigError> {
        Decimal::from_str(value).map_err(|_| ConfigError::FailedToParseInput {
            input: value.to_string(),
            reason: format!(
                "Provided string value for {value} cannot be converted to the underlying type (Decimal)",
            )
	})
    }

    #[inline]
    fn try_add(self, v: impl Into<Decimal>) -> Result<Self, ArithmeticError> {
        let v = v.into();
        self.checked_add(v).ok_or(ArithmeticError::DecimalFailed {
            operation: ArithmeticOperation::Addition,
            left: self,
            right: v,
        })
    }

    #[inline]
    fn try_sub(self, v: impl Into<Decimal>) -> Result<Self, ArithmeticError> {
        let v = v.into();
        self.checked_sub(v).ok_or(ArithmeticError::DecimalFailed {
            operation: ArithmeticOperation::Subtraction,
            left: self,
            right: v,
        })
    }

    #[inline]
    fn try_mul(self, v: impl Into<Decimal>) -> Result<Self, ArithmeticError> {
        let v = v.into();
        self.checked_mul(v).ok_or(ArithmeticError::DecimalFailed {
            operation: ArithmeticOperation::Multiplication,
            left: self,
            right: v,
        })
    }

    #[inline]
    fn try_div(self, v: impl Into<Decimal>) -> Result<Self, ArithmeticError> {
        let v = v.into();
        self.checked_div(v).ok_or(ArithmeticError::DecimalFailed {
            operation: ArithmeticOperation::Division,
            left: self,
            right: v,
        })
    }

    #[inline]
    fn try_exp(self) -> Result<Self, ArithmeticError> {
        self.checked_exp().ok_or(ArithmeticError::DecimalFailed {
            operation: ArithmeticOperation::Exponentiation,
            left: self,
            right: Decimal::ZERO,
        })
    }
}
