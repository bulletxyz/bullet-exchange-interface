use std::io::Read;
use std::ops::{Add, Div, Mul, Neg, Sub};
use std::str::FromStr;

use borsh::{BorshDeserialize, BorshSerialize};
use rust_decimal::{Decimal, MathematicalOps};
use serde::{Deserialize, Serialize};
use sov_universal_wallet::UniversalWallet;

use super::{FixedPositiveDecimal, RoundingMode, SurrogateDecimal, TryDecimalOps};
use crate::error::{ArithmeticError, ArithmeticOperation, ConfigError};

#[derive(
    BorshSerialize,
    Clone,
    Copy,
    Debug,
    Default,
    Deserialize,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
    Serialize,
    UniversalWallet,
)]
#[serde(into = "Decimal", try_from = "Decimal")]
pub struct PositiveDecimal(#[sov_wallet(as_ty = "SurrogateDecimal")] Decimal);

impl std::fmt::Display for PositiveDecimal {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl FromStr for PositiveDecimal {
    type Err = ConfigError;

    fn from_str(s: &str) -> Result<Self, <Self as FromStr>::Err> {
        let decimal = Decimal::try_from_str(s)?;
        Self::try_from(decimal)
    }
}

impl From<u8> for PositiveDecimal {
    fn from(value: u8) -> Self {
        Self(Decimal::from(value))
    }
}

impl From<u16> for PositiveDecimal {
    fn from(value: u16) -> Self {
        Self(Decimal::from(value))
    }
}

impl From<u32> for PositiveDecimal {
    fn from(value: u32) -> Self {
        Self(Decimal::from(value))
    }
}

impl From<u64> for PositiveDecimal {
    fn from(value: u64) -> Self {
        Self(Decimal::from(value))
    }
}

impl TryFrom<Decimal> for PositiveDecimal {
    type Error = ConfigError;

    fn try_from(value: Decimal) -> Result<Self, ConfigError> {
        Self::new(value).ok_or_else(|| ConfigError::FailedToParseInput {
            input: value.to_string(),
            reason: format!(
                "Provided decimal value for {value} cannot be converted to the underlying type (PositiveDecimal).",
            ),
        })
    }
}

impl From<PositiveDecimal> for Decimal {
    #[inline]
    fn from(value: PositiveDecimal) -> Self {
        value.0
    }
}

impl BorshDeserialize for PositiveDecimal {
    fn deserialize_reader<R: Read>(reader: &mut R) -> std::io::Result<Self> {
        let decimal = Decimal::deserialize_reader(reader)?;
        Self::new(decimal).ok_or_else(|| {
            std::io::Error::new(
                std::io::ErrorKind::InvalidData,
                format!("PositiveDecimal cannot be negative: {decimal}"),
            )
        })
    }
}

impl schemars::JsonSchema for PositiveDecimal {
    fn schema_name() -> String {
        "PositiveDecimal".to_string()
    }

    fn json_schema(generator: &mut schemars::r#gen::SchemaGenerator) -> schemars::schema::Schema {
        Decimal::json_schema(generator)
    }
}

impl TryDecimalOps for PositiveDecimal {
    #[inline]
    fn try_from_str(value: &str) -> Result<Self, ConfigError> {
        Self::from_str(value)
    }

    #[inline]
    fn try_add(self, v: impl Into<Decimal>) -> Result<Self, ArithmeticError> {
        let v = v.into();

        self.as_dec()
            .checked_add(v)
            .and_then(Self::new)
            .ok_or_else(|| ArithmeticError::DecimalFailed {
                operation: ArithmeticOperation::Addition,
                left: self.as_dec(),
                right: v,
            })
    }

    #[inline]
    fn try_sub(self, v: impl Into<Decimal>) -> Result<Self, ArithmeticError> {
	let v = v.into();
        self.as_dec()
            .checked_sub(v)
            .and_then(Self::new)
            .ok_or_else(|| ArithmeticError::DecimalFailed {
                operation: ArithmeticOperation::Subtraction,
                left: self.as_dec(),
                right: v,
            })
    }

    #[inline]
    fn try_mul(self, v: impl Into<Decimal>) -> Result<Self, ArithmeticError> {
        let v = v.into();
        if v.is_sign_negative() {
            return Err(ArithmeticError::DecimalFailed {
                operation: ArithmeticOperation::Multiplication,
                left: self.as_dec(),
                right: v,
            });
        }
        self.as_dec()
            .checked_mul(v)
            .and_then(Self::new)
            .ok_or_else(|| ArithmeticError::DecimalFailed {
                operation: ArithmeticOperation::Multiplication,
                left: self.as_dec(),
                right: v,
            })
    }

    #[inline]
    fn try_div(self, v: impl Into<Decimal>) -> Result<Self, ArithmeticError> {
        let v = v.into();
        if v.is_sign_negative() {
            return Err(ArithmeticError::DecimalFailed {
                operation: ArithmeticOperation::Division,
                left: self.as_dec(),
                right: v,
            });
        }
        self.as_dec()
            .checked_div(v)
            .and_then(Self::new)
            .ok_or_else(|| ArithmeticError::DecimalFailed {
                operation: ArithmeticOperation::Division,
                left: self.as_dec(),
                right: v,
            })
    }

    #[inline]
    fn try_exp(self) -> Result<Self, ArithmeticError> {
        self.as_dec()
            .checked_exp()
            .and_then(Self::new)
            .ok_or_else(|| ArithmeticError::DecimalFailed {
                operation: ArithmeticOperation::Exponentiation,
                left: self.as_dec(),
                right: Decimal::ZERO,
            })
    }
}

impl PositiveDecimal {
    /// The smallest positive value that can be represented by PositiveDecimal.
    pub const ZERO: PositiveDecimal = PositiveDecimal(Decimal::ZERO);

    pub const ONE: PositiveDecimal = PositiveDecimal(Decimal::ONE);

    pub const TWO: PositiveDecimal = PositiveDecimal(Decimal::TWO);

    pub const TEN: PositiveDecimal = PositiveDecimal(Decimal::TEN);

    pub const ONE_HUNDRED: PositiveDecimal = PositiveDecimal(Decimal::ONE_HUNDRED);

    pub const MAX: PositiveDecimal = PositiveDecimal(Decimal::MAX);

    #[inline]
    pub fn new(value: Decimal) -> Option<Self> {
        if value.is_sign_positive() {
            Some(PositiveDecimal(value))
        } else {
            None
        }
    }

    #[inline]
    pub fn to_fixed(&self, rounding_mode: RoundingMode) -> FixedPositiveDecimal {
        FixedPositiveDecimal::new(*self, rounding_mode)
    }

    #[inline]
    pub fn as_dec(&self) -> Decimal {
        self.0
    }

    #[inline]
    pub fn is_zero(&self) -> bool {
        self.0.is_zero()
    }

    #[inline]
    pub fn fract(&self) -> Decimal {
        self.0.fract()
    }

    #[inline]
    pub fn try_pow_i64(&self, v: i64) -> Result<PositiveDecimal, ArithmeticError> {
        self.as_dec()
            .checked_powi(v)
            .and_then(Self::new)
            .ok_or_else(|| ArithmeticError::DecimalFailed {
                operation: ArithmeticOperation::Exponentiation,
                left: self.as_dec(),
                right: Decimal::from(v),
            })
    }

    #[inline]
    pub fn try_with_precision(&self, precision: i64) -> Result<PositiveDecimal, ArithmeticError> {
        let scaling_factor = Self::TEN.try_pow_i64(precision)?;
        self.try_mul(scaling_factor)
    }

    /// Performs a saturating subtraction for PositiveDecimal, this is for intentional saturating subtractions hence no warnings
    /// We don't return a result because we know the saturating subtraction result will always be non-negative
    #[inline]
    pub fn saturating_sub(self, v: impl Into<Decimal>) -> Self {
        let v = v.into();
        if self.as_dec().lt(&v) {
            Self::ZERO
        } else {
            // Safe because:
            // 1. We checked self >= v above
            // 2. Decimal subtraction can only fail on overflow
            // 3. When subtracting a smaller number from a larger number, overflow is impossible
            Self(self.as_dec().sub(v))
        }
    }
}

impl Neg for PositiveDecimal {
    type Output = Decimal;

    fn neg(self) -> Decimal {
        self.as_dec().neg()
    }
}

// These are mostly just for conveniece for tests, do not use in production
impl Add for PositiveDecimal {
    type Output = Self;

    fn add(self, other: Self) -> Self::Output {
        Self(self.as_dec().add(other.as_dec()))
    }
}

// Can potentially be negative, so return a Decimal
impl Sub for PositiveDecimal {
    type Output = Decimal;

    fn sub(self, other: Self) -> Self::Output {
        self.as_dec().sub(other.as_dec())
    }
}

impl Mul for PositiveDecimal {
    type Output = Self;

    fn mul(self, other: Self) -> Self::Output {
        Self(self.as_dec().mul(other.as_dec()))
    }
}

impl Div for PositiveDecimal {
    type Output = Self;

    fn div(self, other: Self) -> Self::Output {
        Self(self.as_dec().div(other.as_dec()))
    }
}

#[cfg(test)]
mod tests {
    use borsh::to_vec;

    use super::*;

    #[test]
    fn serde_rejects_negative_decimal() {
        let json = r#""-123.45""#;
        let result = serde_json::from_str::<PositiveDecimal>(json);
        assert!(result.is_err(), "Should reject negative decimal via serde");
    }

    #[test]
    fn serde_accepts_positive_decimal() {
        let json = r#""123.45""#;
        let result = serde_json::from_str::<PositiveDecimal>(json);
        assert!(result.is_ok(), "Should accept positive decimal via serde");
        assert_eq!(result.unwrap().to_string(), "123.45");
    }

    #[test]
    fn serde_accepts_zero() {
        let json = r#""0""#;
        let result = serde_json::from_str::<PositiveDecimal>(json);
        assert!(result.is_ok(), "Should accept zero via serde");
    }

    #[test]
    fn borsh_rejects_negative_decimal() {
        // Serialize a negative Decimal directly, then try to deserialize as PositiveDecimal
        let negative = Decimal::new(-12345, 2); // -123.45
        let bytes = to_vec(&negative).expect("serialize negative decimal");

        let result = PositiveDecimal::try_from_slice(&bytes);
        assert!(result.is_err(), "Should reject negative decimal via borsh");
    }

    #[test]
    fn borsh_accepts_positive_decimal() {
        let positive = Decimal::new(12345, 2); // 123.45
        let bytes = to_vec(&positive).expect("serialize positive decimal");

        let result = PositiveDecimal::try_from_slice(&bytes);
        assert!(result.is_ok(), "Should accept positive decimal via borsh");
        assert_eq!(result.unwrap().to_string(), "123.45");
    }

    #[test]
    fn borsh_roundtrip_preserves_value() {
        let original = PositiveDecimal::new(Decimal::new(12345, 2)).unwrap();
        let bytes = to_vec(&original).expect("serialize");
        let restored = PositiveDecimal::try_from_slice(&bytes).expect("deserialize");
        assert_eq!(original, restored);
    }
}
