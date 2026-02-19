use std::str::FromStr;

use borsh::{BorshDeserialize, BorshSerialize};
use rust_decimal::Decimal;
use schemars;
use sov_universal_wallet::UniversalWallet;

use super::TryDecimalOps;
use crate::error::ConfigError;

#[derive(
    BorshDeserialize,
    BorshSerialize,
    Clone,
    Copy,
    Default,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
    UniversalWallet,
)]
#[allow(dead_code)]
pub struct SurrogateDecimal {
    flags: u32,
    hi: u32,
    lo: u32,
    mid: u32,
}

impl std::fmt::Debug for SurrogateDecimal {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "SurrogateDecimal({})", Decimal::from(*self))
    }
}

impl std::fmt::Display for SurrogateDecimal {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", Decimal::from(*self))
    }
}

// Implement Serialize and Deserialize for SurrogateDecimal to mirror Decimal's behavior
impl serde::Serialize for SurrogateDecimal {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serde::Serialize::serialize(&Decimal::from(*self), serializer)
    }
}

impl<'de> serde::Deserialize<'de> for SurrogateDecimal {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let decimal: Decimal = serde::Deserialize::deserialize(deserializer)?;
        Ok(Self::from(decimal))
    }
}

impl From<SurrogateDecimal> for Decimal {
    fn from(surrogate: SurrogateDecimal) -> Decimal {
        // Use Decimal's internal constructor if available, or
        unsafe { std::mem::transmute::<SurrogateDecimal, Decimal>(surrogate) }
    }
}

impl From<Decimal> for SurrogateDecimal {
    fn from(decimal: Decimal) -> Self {
        unsafe { std::mem::transmute::<Decimal, Self>(decimal) }
    }
}

impl FromStr for SurrogateDecimal {
    type Err = ConfigError;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Decimal::try_from_str(s).map(|x| x.into())
    }
}

impl schemars::JsonSchema for SurrogateDecimal {
    fn schema_name() -> String {
        "Decimal".to_string()
    }

    fn json_schema(generator: &mut schemars::r#gen::SchemaGenerator) -> schemars::schema::Schema {
        Decimal::json_schema(generator)
    }
}
