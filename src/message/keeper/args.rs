//! Argument types for keeper operations.

use crate::decimals::{PositiveDecimal, SurrogateDecimal};
use crate::types::{AssetId, MarketId};

#[derive(
    Clone,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
    borsh::BorshDeserialize,
    borsh::BorshSerialize,
    schemars::JsonSchema,
    serde::Deserialize,
    serde::Serialize,
    sov_universal_wallet::UniversalWallet,
)]
pub struct OraclePriceUpdateArgs {
    pub asset_id: AssetId,
    pub oracle_price: PositiveDecimal,
}

#[derive(
    Clone,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
    borsh::BorshDeserialize,
    borsh::BorshSerialize,
    schemars::JsonSchema,
    serde::Deserialize,
    serde::Serialize,
    sov_universal_wallet::UniversalWallet,
)]
pub struct MarkPriceUpdateArgs {
    pub market_id: MarketId,
    pub median_cex_price: PositiveDecimal,
    pub diff_ema: SurrogateDecimal,
}
