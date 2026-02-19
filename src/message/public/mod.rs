//! Permissionless operations.

use sov_rollup_interface::BasicAddress;

use crate::types::MarketId;

/// Permissionless operations anyone can call.
///
/// These operations have no authorization checks - anyone can trigger them.
/// Typically used for MEV opportunities (liquidations) or protocol maintenance
/// (interest accrual, funding application).
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
#[serde(rename_all = "snake_case", bound = "Address: BasicAddress")]
#[schemars(bound = "Address: BasicAddress")]
#[borsh(use_discriminant = true)]
#[repr(u8)]
pub enum PublicAction<Address: BasicAddress> {
    /// Liquidate perp positions for an underwater account (permissionless).
    LiquidatePerpPositions { address: Address } = 0,

    /// Force cancel orders for a liquidatable user (permissionless).
    ForceCancelOrders { user_address: Address } = 1,

    /// Execute active trigger orders (permissionless).
    ExecuteTriggerOrders { market_id: MarketId } = 2,

    /// Apply funding to user accounts (permissionless).
    ApplyFunding { addresses: Vec<Address> } = 3,

    /// Accrue borrow/lend interest (permissionless).
    AccrueBorrowLendInterest {} = 4,

    /// Execute TWAP orders  (permissionless)
    ExecuteTwapOrders { market_id: MarketId } = 5,

    /// Activate TWAP orders  (permissionless)
    ActivateTwapOrders { market_ids: Vec<MarketId> } = 6,
    // Reserved: 7-255
}
