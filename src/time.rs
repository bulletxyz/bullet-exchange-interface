use crate::define_simple_type;
use crate::error::{ArithmeticError, ArithmeticOperation};

// Stores microseconds since Unix Epoch.
define_simple_type!(UnixTimestampMicros(i64));

const MICROSECONDS_PER_HOUR: i64 = 3600_000_000;

impl UnixTimestampMicros {
    pub const ZERO: Self = Self(0);
    pub fn from_secs(secs: i64) -> Result<Self, ArithmeticError> {
        let micros = secs
            .checked_mul(1_000_000)
            .ok_or(ArithmeticError::IntegerFailed {
                operation: ArithmeticOperation::Multiplication,
                left: secs as i128,
                right: 1_000_000,
            })?;
        Ok(Self(micros))
    }

    pub fn from_secs_u32(secs: u32) -> Self {
        #[allow(clippy::arithmetic_side_effects)]
        Self((secs as i64) * 1_000_000)
    }

    pub fn from_micros(micros: i64) -> Self {
        Self(micros)
    }

    pub fn from_millis(millis: i64) -> Result<Self, ArithmeticError> {
        millis
            .checked_mul(1000)
            .map(Self)
            .ok_or(ArithmeticError::IntegerFailed {
                operation: ArithmeticOperation::Multiplication,
                left: millis as i128,
                right: 1000,
            })
    }
    pub fn from_nanos(nanos: u128) -> Result<Self, ArithmeticError> {
        (nanos / 1000)
            .try_into()
            .map(Self)
            .map_err(|_| ArithmeticError::IntegerFailed {
                operation: ArithmeticOperation::Division,
                left: nanos as i128,
                right: 1000,
            })
    }

    pub fn as_secs(&self) -> i64 {
        self.0 / 1_000_000
    }

    pub fn as_micros(&self) -> i64 {
        self.0
    }

    pub fn as_hour(&self) -> i64 {
        self.0 / MICROSECONDS_PER_HOUR
    }

    pub fn checked_add_secs(&self, other: i64) -> Result<Self, ArithmeticError> {
        self.checked_add(Self::from_secs(other)?)
    }
    pub fn checked_add(&self, other: UnixTimestampMicros) -> Result<Self, ArithmeticError> {
        self.0
            .checked_add(other.0)
            .map(Self)
            .ok_or(ArithmeticError::IntegerFailed {
                operation: ArithmeticOperation::Addition,
                left: self.0 as i128,
                right: other.0 as i128,
            })
    }

    /// Returns the seconds elapsed. Or zero on errors.
    pub fn elapsed_secs(self, other: UnixTimestampMicros) -> u64 {
        let delta = self
            .as_secs()
            .checked_sub(other.as_secs())
            .unwrap_or(0)
            .max(0);
        // Safe to cast to u64 as it will be 0 or larger
        delta as u64
    }
}
