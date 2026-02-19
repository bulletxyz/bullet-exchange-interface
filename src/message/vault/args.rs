//! Argument types for vault operations.

use crate::decimals::PositiveDecimal;

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
pub struct UpdateVaultConfigArgs {
    pub deposit_limit: Option<PositiveDecimal>,
    pub withdraw_lockup_period_hours: Option<u8>,
    pub profit_share_percentage: Option<u8>,
}
