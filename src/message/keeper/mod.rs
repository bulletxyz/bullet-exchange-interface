//! Keeper operations.

use crate::decimals::PositiveDecimal;
use crate::time::UnixTimestampMicros;
use crate::types::{AdminType, AssetId, FeeTier, MarketId};
mod args;
pub use args::*;

/// Keeper operations requiring specific admin privileges.
///
/// These operations are typically called by automated keepers/bots
/// and require specific admin types (Pricing, Funding, Credits,
/// FeeTier, Referrals). However, one can also update all admin
/// addresses with it.
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
    strum::AsRefStr,
)]
#[serde(rename_all = "snake_case")]
#[borsh(use_discriminant = true)]
#[repr(u8)]
pub enum KeeperAction<Address> {
    // =========================================================================
    // Pricing Admin Operations (0-9)
    // =========================================================================
    /// Update oracle prices (PricingAdmin).
    UpdateOraclePrices {
        prices: Vec<OraclePriceUpdateArgs>,
        publish_timestamp: UnixTimestampMicros,
    } = 0,

    /// Update mark prices (PricingAdmin).
    UpdateMarkPrices {
        prices: Vec<MarkPriceUpdateArgs>,
        publish_timestamp: UnixTimestampMicros,
    } = 1,

    /// Update premium indexes for markets (PricingAdmin).
    UpdatePremiumIndexes { market_ids: Vec<MarketId> } = 2,
    // Reserved: 3-9

    // =========================================================================
    // Funding Admin Operations (10-19)
    // =========================================================================
    /// Update funding rates for markets (FundingAdmin).
    UpdateFunding { market_ids: Vec<MarketId> } = 10,
    // Reserved: 11-19

    // =========================================================================
    // Credits Admin Operations (20-29)
    // =========================================================================
    /// Add trading credits to a user (CreditsAdmin).
    AddTradingCredits {
        user_address: Address,
        amount: PositiveDecimal,
    } = 20,

    /// Remove trading credits from a user (CreditsAdmin).
    RemoveTradingCredits {
        user_address: Address,
        amount: PositiveDecimal,
    } = 21,
    // Reserved: 22-29

    // =========================================================================
    // FeeTier Admin Operations (30-39)
    // =========================================================================
    /// Update user's fee tier (FeeTierAdmin).
    UpdateUserFeeTier { address: Address, fee_tier: FeeTier } = 30,

    /// Update a given user's fee discount (in bps) (FeeTierAdmin).
    UpdateUserFeeDiscountBps {
        address: Address,
        fee_discount_bps: u16,
    } = 31,
    // Reserved: 32-39

    // =========================================================================
    // Referrals Admin Operations (40-49)
    // =========================================================================
    /// Set a user's cumulative referral rewards to an absolute amount (ReferralsAdmin).
    SetCumulativeReferralRewards {
        address: Address,
        asset_id: AssetId,
        amount: PositiveDecimal,
    } = 40,
    // Reserved: 41-49
}

impl<Address> KeeperAction<Address> {
    /// Returns the specific admin type required for this keeper operation.
    #[must_use]
    pub fn required_admin_type(&self) -> AdminType {
        match self {
            Self::UpdateOraclePrices { .. }
            | Self::UpdateMarkPrices { .. }
            | Self::UpdatePremiumIndexes { .. } => AdminType::Pricing,
            Self::UpdateFunding { .. } => AdminType::Funding,
            Self::AddTradingCredits { .. } | Self::RemoveTradingCredits { .. } => {
                AdminType::Credits
            }
            Self::UpdateUserFeeTier { .. } | Self::UpdateUserFeeDiscountBps { .. } => {
                AdminType::FeeTier
            }
            Self::SetCumulativeReferralRewards { .. } => AdminType::Referrals,
        }
    }
}
