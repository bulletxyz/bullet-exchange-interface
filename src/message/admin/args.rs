//! Argument types for admin operations.

use std::collections::BTreeMap;

use crate::decimals::{PositiveDecimal, SurrogateDecimal};
use crate::define_struct;
use crate::string::CustomString;
use crate::types::{AssetId, MarketId, TokenId, TradingMode};

// =============================================================================
// Market Args
// =============================================================================

define_struct! {
    struct InitPerpMarketArgs {
        market_id: MarketId,
        base_asset_id: AssetId,
        name: CustomString,
        trading_mode: TradingMode,
        min_tick_size: PositiveDecimal,
        min_lot_size: PositiveDecimal,
        max_orders_per_side: u16,
        max_orders_per_user: u16,
        max_trigger_orders_per_user: u16,
        min_interest_rate_clamp: SurrogateDecimal,
        max_interest_rate_clamp: SurrogateDecimal,
        min_funding_rate_clamp: SurrogateDecimal,
        max_funding_rate_clamp: SurrogateDecimal,
        max_oi_notional: PositiveDecimal,
        max_order_to_mark_price_deviation_ratio: PositiveDecimal,
        max_trigger_to_comparison_price_deviation_ratio: PositiveDecimal,
        max_order_to_trigger_price_deviation_ratio: PositiveDecimal,
        impact_margin: PositiveDecimal,
        interest_rate: SurrogateDecimal,
        leverage_table_args: SurrogateLeverageTableArgs,
        taker_fees_tenth_bps: Vec<i16>,
        maker_fees_tenth_bps: Vec<i16>,
    }
}

define_struct! {
    struct UpdatePerpMarketArgs {
        market_id: MarketId,
        impact_margin: Option<PositiveDecimal>,
        interest_rate: Option<SurrogateDecimal>,
        leverage_table_args: Option<SurrogateLeverageTableArgs>,
        maker_fees_tenth_bps: Option<Vec<i16>>,
        max_funding_rate_clamp: Option<SurrogateDecimal>,
        max_interest_rate_clamp: Option<SurrogateDecimal>,
        max_oi_notional: Option<PositiveDecimal>,
        max_order_to_mark_price_deviation_ratio: Option<PositiveDecimal>,
        max_order_to_trigger_price_deviation_ratio: Option<PositiveDecimal>,
        max_orders_per_side: Option<u16>,
        max_orders_per_user: Option<u16>,
        max_trigger_orders_per_user: Option<u16>,
        max_trigger_to_comparison_price_deviation_ratio: Option<PositiveDecimal>,
        min_funding_rate_clamp: Option<SurrogateDecimal>,
        min_interest_rate_clamp: Option<SurrogateDecimal>,
        min_lot_size: Option<PositiveDecimal>,
        min_tick_size: Option<PositiveDecimal>,
        taker_fees_tenth_bps: Option<Vec<i16>>,
    }
}

define_struct! {
    struct InitSpotMarketArgs {
        market_id: MarketId,
        base_asset_id: AssetId,
        quote_asset_id: AssetId,
        base_min_lot_size: PositiveDecimal,
        quote_min_lot_size: PositiveDecimal,
        max_orders_per_side: u16,
        max_orders_per_user: u16,
        max_trigger_orders_per_user: u16,
        taker_fees_tenth_bps: Vec<i16>,
        maker_fees_tenth_bps: Vec<i16>,
        max_order_to_trigger_price_deviation_ratio: PositiveDecimal,
        name: CustomString,
    }
}

define_struct! {
    struct UpdateSpotMarketArgs {
        market_id: MarketId,
        base_min_lot_size: Option<PositiveDecimal>,
        quote_min_lot_size: Option<PositiveDecimal>,
        max_orders_per_side: Option<u16>,
        max_orders_per_user: Option<u16>,
        max_trigger_orders_per_user: Option<u16>,
        taker_fees_tenth_bps: Option<Vec<i16>>,
        maker_fees_tenth_bps: Option<Vec<i16>>,
        max_order_to_trigger_price_deviation_ratio: Option<PositiveDecimal>,
    }
}

define_struct! {
    struct LeverageTableArgs {
        table: BTreeMap<PositiveDecimal, u16>,
    }
}

define_struct! {
    struct SurrogateLeverageTableArgs {
        table: BTreeMap<CustomString, u16>,
    }
}
// =============================================================================
// Asset Args
// =============================================================================

define_struct! {
    struct InitAssetInfoArgs {
        asset_id: AssetId,
        asset_name: CustomString,
        token_id: Option<TokenId>,
        decimals: u8,
        withdraw_fee: PositiveDecimal,
    }
}

define_struct! {
    struct InitAssetInfoArgsV1 {
        asset_id: AssetId,
        asset_name: CustomString,
        token_id: Option<TokenId>,
        decimals: u8,
        withdraw_fee: PositiveDecimal,
        pyth_lazer_feed_id: Option<u32>,
        pyth_lazer_quote_feed_id: Option<u32>,
    }
}

define_struct! {
    struct UpdateAssetInfoArgs {
        asset_id: AssetId,
        withdraw_fee: PositiveDecimal,
    }
}

define_struct! {
    struct UpdateAssetInfoArgsV1 {
        asset_id: AssetId,
        withdraw_fee: PositiveDecimal,
        pyth_lazer_feed_id: Option<u32>,
        pyth_lazer_quote_feed_id: Option<u32>,
    }
}

// =============================================================================
// Borrow/Lend Pool Args
// =============================================================================

define_struct! {
    struct InitBorrowLendPoolArgs {
        asset_id: AssetId,
        optimal_utilization_rate: PositiveDecimal,
        min_borrow_rate: PositiveDecimal,
        max_borrow_rate: PositiveDecimal,
        optimal_borrow_rate: PositiveDecimal,
        asset_weight: PositiveDecimal,
        initial_liability_weight: PositiveDecimal,
        maintenance_liability_weight: PositiveDecimal,
        deposit_limit: PositiveDecimal,
        borrow_limit: PositiveDecimal,
        max_utilization_rate: PositiveDecimal,
        liquidation_total_reward_ratio: PositiveDecimal,
        protocol_reward_ratio: PositiveDecimal,
        liability_liquidation_limit_ratio: PositiveDecimal,
        interest_fee_tenth_bps: u16,
    }
}

define_struct! {
    struct UpdateBorrowLendPoolArgs {
        asset_id: AssetId,
        optimal_utilization_rate: Option<PositiveDecimal>,
        min_borrow_rate: Option<PositiveDecimal>,
        max_borrow_rate: Option<PositiveDecimal>,
        optimal_borrow_rate: Option<PositiveDecimal>,
        asset_weight: Option<PositiveDecimal>,
        initial_liability_weight: Option<PositiveDecimal>,
        maintenance_liability_weight: Option<PositiveDecimal>,
        deposit_limit: Option<PositiveDecimal>,
        borrow_limit: Option<PositiveDecimal>,
        max_utilization_rate: Option<PositiveDecimal>,
        liquidation_total_reward_ratio: Option<PositiveDecimal>,
        protocol_reward_ratio: Option<PositiveDecimal>,
        liability_liquidation_limit_ratio: Option<PositiveDecimal>,
        interest_fee_tenth_bps: Option<u16>,
    }
}

// =============================================================================
// Config Args
// =============================================================================

define_struct! {
    struct UpdatePerpLiquidationConfigArgs {
        liquidation_fee: Option<PositiveDecimal>,
        liquidation_ioc_buffer: Option<PositiveDecimal>,
        backstop_liquidation_threshold: Option<PositiveDecimal>,
        liquidation_protocol_reward_ratio: Option<PositiveDecimal>,
    }
}

define_struct! {
    struct UpdateGlobalConfigArgs<Address> {
        max_orders_per_user: Option<u16>,
        max_trigger_orders_per_user: Option<u16>,
        max_orders_per_batch_msg: Option<u16>,
        max_trigger_orders_to_execute_per_msg: Option<u16>,
        min_notional_twap_value: Option<PositiveDecimal>,
        min_notional_twap_value_per_order: Option<PositiveDecimal>,
        twap_execution_interval_seconds: Option<u64>,
        deposit_limits_per_user: Option<Vec<(AssetId, PositiveDecimal)>>,
        whitelisted_users_for_deposit: Option<Vec<Address>>,
    }
}

define_struct! {
    struct UpdateGlobalConfigArgsV1<Address> {
        max_orders_per_user: Option<u16>,
        max_trigger_orders_per_user: Option<u16>,
        max_orders_per_batch_msg: Option<u16>,
        max_trigger_orders_to_execute_per_msg: Option<u16>,
        min_notional_twap_value: Option<PositiveDecimal>,
        min_notional_twap_value_per_order: Option<PositiveDecimal>,
        twap_execution_interval_seconds: Option<u64>,
        deposit_limits_per_user: Option<Vec<(AssetId, PositiveDecimal)>>,
        whitelisted_users_for_deposit: Option<Vec<Address>>,
        pyth_lazer_trusted_signers: Option<Vec<[u8; 32]>>,
    }
}

define_struct! {
    struct UpdateGlobalVaultConfigArgs {
        leader_minimum_holding_percentage: Option<u8>,
        creation_fee_usdc: Option<PositiveDecimal>,
        min_deposit_value: Option<PositiveDecimal>,
    }
}
