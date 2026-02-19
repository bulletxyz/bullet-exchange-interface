//! Argument types for user operations.

use crate::decimals::PositiveDecimal;
use crate::string::CustomString;
use crate::types::{
    AssetId, ClientOrderId, MarketId, OrderId, OrderType, Side, TriggerDirection,
    TriggerPriceCondition,
};

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
pub struct NewOrderArgs {
    pub price: PositiveDecimal,
    pub size: PositiveDecimal,
    pub side: Side,
    pub order_type: OrderType,
    pub reduce_only: bool,
    pub client_order_id: Option<ClientOrderId>,
    pub pending_tpsl_pair: Option<PendingTpslPair>,
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
pub struct AmendOrderArgs {
    pub cancel: CancelOrderArgs,
    pub place: NewOrderArgs,
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
pub struct CancelOrderArgs {
    pub order_id: Option<OrderId>,
    pub client_order_id: Option<ClientOrderId>,
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
pub struct PendingTpslPair {
    pub tpsl_pair: TpslPair,
    pub dynamic_size: bool,
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
pub struct TpslPair {
    pub tp: Option<Tpsl>,
    pub sl: Option<Tpsl>,
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
pub struct Tpsl {
    pub order_price: PositiveDecimal,
    pub trigger_price: PositiveDecimal,
    pub price_condition: TriggerPriceCondition,
    pub order_type: OrderType,
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
pub struct NewTriggerOrderArgs {
    pub side: Side,
    pub order_price: PositiveDecimal,
    pub trigger_price: PositiveDecimal,
    pub trigger_direction: TriggerDirection,
    pub price_condition: TriggerPriceCondition,
    pub size: Option<PositiveDecimal>,
    pub order_type: OrderType,
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
pub struct NewTwapOrderArgs {
    pub side: Side,
    pub total_size: PositiveDecimal,
    pub reduce_only: bool,
    pub total_duration_seconds: u64,
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
pub struct BackstopLiquidatePerpPositionArgs {
    pub market_id: MarketId,
    pub size: PositiveDecimal,
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
pub struct CreateVaultArgs<Address> {
    pub name: CustomString,
    pub description: CustomString,
    pub leader: Address,
    pub deposit_asset_ids: Vec<AssetId>,
    pub withdraw_asset_id: AssetId,
    pub withdraw_lockup_period_hours: u8,
    pub whitelist_deposits: bool,
    pub profit_share_percentage: u8,
    pub withdrawal_fee_bps: u8,
    pub deposit_limit: PositiveDecimal,
}
