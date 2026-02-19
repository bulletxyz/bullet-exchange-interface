//! Argument types for admin operations.

use std::collections::BTreeMap;

use sov_rollup_interface::BasicAddress;

use crate::decimals::{PositiveDecimal, SurrogateDecimal};
use crate::string::CustomString;
use crate::types::{AssetId, MarketId, TokenId, TradingMode};

// =============================================================================
// Market Args
// =============================================================================

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
pub struct InitPerpMarketArgs {
    pub market_id: MarketId,
    pub base_asset_id: AssetId,
    pub name: CustomString,
    pub trading_mode: TradingMode,
    pub min_tick_size: PositiveDecimal,
    pub min_lot_size: PositiveDecimal,
    pub max_orders_per_side: u16,
    pub max_orders_per_user: u16,
    pub max_trigger_orders_per_user: u16,
    pub min_interest_rate_clamp: SurrogateDecimal,
    pub max_interest_rate_clamp: SurrogateDecimal,
    pub min_funding_rate_clamp: SurrogateDecimal,
    pub max_funding_rate_clamp: SurrogateDecimal,
    pub max_oi_notional: PositiveDecimal,
    pub max_order_to_mark_price_deviation_ratio: PositiveDecimal,
    pub max_trigger_to_comparison_price_deviation_ratio: PositiveDecimal,
    pub max_order_to_trigger_price_deviation_ratio: PositiveDecimal,
    pub impact_margin: PositiveDecimal,
    pub interest_rate: SurrogateDecimal,
    pub leverage_table_args: SurrogateLeverageTableArgs,
    pub taker_fees_tenth_bps: Vec<i16>,
    pub maker_fees_tenth_bps: Vec<i16>,
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
pub struct UpdatePerpMarketArgs {
    pub market_id: MarketId,
    pub impact_margin: Option<PositiveDecimal>,
    pub interest_rate: Option<SurrogateDecimal>,
    pub leverage_table_args: Option<SurrogateLeverageTableArgs>,
    pub maker_fees_tenth_bps: Option<Vec<i16>>,
    pub max_funding_rate_clamp: Option<SurrogateDecimal>,
    pub max_interest_rate_clamp: Option<SurrogateDecimal>,
    pub max_oi_notional: Option<PositiveDecimal>,
    pub max_order_to_mark_price_deviation_ratio: Option<PositiveDecimal>,
    pub max_order_to_trigger_price_deviation_ratio: Option<PositiveDecimal>,
    pub max_orders_per_side: Option<u16>,
    pub max_orders_per_user: Option<u16>,
    pub max_trigger_orders_per_user: Option<u16>,
    pub max_trigger_to_comparison_price_deviation_ratio: Option<PositiveDecimal>,
    pub min_funding_rate_clamp: Option<SurrogateDecimal>,
    pub min_interest_rate_clamp: Option<SurrogateDecimal>,
    pub min_lot_size: Option<PositiveDecimal>,
    pub min_tick_size: Option<PositiveDecimal>,
    pub taker_fees_tenth_bps: Option<Vec<i16>>,
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
pub struct InitSpotMarketArgs {
    pub market_id: MarketId,
    pub base_asset_id: AssetId,
    pub quote_asset_id: AssetId,
    pub base_min_lot_size: PositiveDecimal,
    pub quote_min_lot_size: PositiveDecimal,
    pub max_orders_per_side: u16,
    pub max_orders_per_user: u16,
    pub max_trigger_orders_per_user: u16,
    pub taker_fees_tenth_bps: Vec<i16>,
    pub maker_fees_tenth_bps: Vec<i16>,
    pub max_order_to_trigger_price_deviation_ratio: PositiveDecimal,
    pub name: CustomString,
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
pub struct UpdateSpotMarketArgs {
    pub market_id: MarketId,
    pub base_min_lot_size: Option<PositiveDecimal>,
    pub quote_min_lot_size: Option<PositiveDecimal>,
    pub max_orders_per_side: Option<u16>,
    pub max_orders_per_user: Option<u16>,
    pub max_trigger_orders_per_user: Option<u16>,
    pub taker_fees_tenth_bps: Option<Vec<i16>>,
    pub maker_fees_tenth_bps: Option<Vec<i16>>,
    pub max_order_to_trigger_price_deviation_ratio: Option<PositiveDecimal>,
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
pub struct LeverageTableArgs {
    pub table: BTreeMap<PositiveDecimal, u16>,
}

#[derive(
    borsh::BorshDeserialize,
    borsh::BorshSerialize,
    serde::Serialize,
    serde::Deserialize,
    Debug,
    PartialEq,
    Eq,
    Clone,
    Hash,
    Ord,
    PartialOrd,
    schemars::JsonSchema,
    sov_universal_wallet::UniversalWallet,
)]
pub struct SurrogateLeverageTableArgs {
    pub table: BTreeMap<CustomString, u16>,
}

// =============================================================================
// Asset Args
// =============================================================================

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
pub struct InitAssetInfoArgs {
    pub asset_id: AssetId,
    pub asset_name: CustomString,
    pub token_id: Option<TokenId>,
    pub decimals: u8,
    pub withdraw_fee: PositiveDecimal,
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
pub struct UpdateAssetInfoArgs {
    pub asset_id: AssetId,
    pub withdraw_fee: PositiveDecimal,
}

// =============================================================================
// Borrow/Lend Pool Args
// =============================================================================

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
pub struct InitBorrowLendPoolArgs {
    pub asset_id: AssetId,
    pub optimal_utilization_rate: PositiveDecimal,
    pub min_borrow_rate: PositiveDecimal,
    pub max_borrow_rate: PositiveDecimal,
    pub optimal_borrow_rate: PositiveDecimal,
    pub asset_weight: PositiveDecimal,
    pub initial_liability_weight: PositiveDecimal,
    pub maintenance_liability_weight: PositiveDecimal,
    pub deposit_limit: PositiveDecimal,
    pub borrow_limit: PositiveDecimal,
    pub max_utilization_rate: PositiveDecimal,
    pub liquidation_total_reward_ratio: PositiveDecimal,
    pub protocol_reward_ratio: PositiveDecimal,
    pub liability_liquidation_limit_ratio: PositiveDecimal,
    pub interest_fee_tenth_bps: u16,
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
pub struct UpdateBorrowLendPoolArgs {
    pub asset_id: AssetId,
    pub optimal_utilization_rate: Option<PositiveDecimal>,
    pub min_borrow_rate: Option<PositiveDecimal>,
    pub max_borrow_rate: Option<PositiveDecimal>,
    pub optimal_borrow_rate: Option<PositiveDecimal>,
    pub asset_weight: Option<PositiveDecimal>,
    pub initial_liability_weight: Option<PositiveDecimal>,
    pub maintenance_liability_weight: Option<PositiveDecimal>,
    pub deposit_limit: Option<PositiveDecimal>,
    pub borrow_limit: Option<PositiveDecimal>,
    pub max_utilization_rate: Option<PositiveDecimal>,
    pub liquidation_total_reward_ratio: Option<PositiveDecimal>,
    pub protocol_reward_ratio: Option<PositiveDecimal>,
    pub liability_liquidation_limit_ratio: Option<PositiveDecimal>,
    pub interest_fee_tenth_bps: Option<u16>,
}

// =============================================================================
// Config Args
// =============================================================================

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
pub struct UpdatePerpLiquidationConfigArgs {
    pub liquidation_fee: Option<PositiveDecimal>,
    pub liquidation_ioc_buffer: Option<PositiveDecimal>,
    pub backstop_liquidation_threshold: Option<PositiveDecimal>,
    pub liquidation_protocol_reward_ratio: Option<PositiveDecimal>,
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
#[serde(bound = "Address: BasicAddress")]
pub struct UpdateGlobalConfigArgs<Address: BasicAddress> {
    pub max_orders_per_user: Option<u16>,
    pub max_trigger_orders_per_user: Option<u16>,
    pub max_orders_per_batch_msg: Option<u16>,
    pub max_trigger_orders_to_execute_per_msg: Option<u16>,
    pub min_notional_twap_value: Option<PositiveDecimal>,
    pub min_notional_twap_value_per_order: Option<PositiveDecimal>,
    pub twap_execution_interval_seconds: Option<u64>,
    pub deposit_limits_per_user: Option<Vec<(AssetId, PositiveDecimal)>>,
    pub whitelisted_users_for_deposit: Option<Vec<Address>>,
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
pub struct UpdateGlobalVaultConfigArgs {
    pub leader_minimum_holding_percentage: Option<u8>,
    pub creation_fee_usdc: Option<PositiveDecimal>,
    pub min_deposit_value: Option<PositiveDecimal>,
}
