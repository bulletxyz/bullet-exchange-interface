//! Argument types for user operations.

use crate::decimals::PositiveDecimal;
use crate::define_struct;
use crate::string::CustomString;
use crate::types::{
    AssetId, ClientOrderId, MarketId, OrderId, OrderType, Side, TriggerDirection,
    TriggerPriceCondition,
};

define_struct! {
    struct NewOrderArgs {
        price: PositiveDecimal,
        size: PositiveDecimal,
        side: Side,
        order_type: OrderType,
        reduce_only: bool,
        client_order_id: Option<ClientOrderId>,
        pending_tpsl_pair: Option<PendingTpslPair>,
    }
}

impl NewOrderArgs {
    pub fn new_liquidate(price: PositiveDecimal, size: PositiveDecimal, side: Side) -> Self {
        Self {
            price,
            size,
            side,
            order_type: OrderType::ImmediateOrCancel,
            reduce_only: true,
            client_order_id: None,
            pending_tpsl_pair: None,
        }
    }
}

define_struct! {
    struct AmendOrderArgs {
        cancel: CancelOrderArgs,
        place: NewOrderArgs,
    }
}

define_struct! {
    struct CancelOrderArgs {
        order_id: Option<OrderId>,
        client_order_id: Option<ClientOrderId>,
    }
}

define_struct! {
    struct PendingTpslPair {
        tpsl_pair: TpslPair,
        dynamic_size: bool,
    }
}

define_struct! {
    struct TpslPair {
        tp: Option<Tpsl>,
        sl: Option<Tpsl>,
    }
}

define_struct! {
    struct Tpsl {
        order_price: PositiveDecimal,
        trigger_price: PositiveDecimal,
        price_condition: TriggerPriceCondition,
        order_type: OrderType,
    }
}

define_struct! {
    struct NewTriggerOrderArgs {
        side: Side,
        order_price: PositiveDecimal,
        trigger_price: PositiveDecimal,
        trigger_direction: TriggerDirection,
        price_condition: TriggerPriceCondition,
        size: Option<PositiveDecimal>,
        order_type: OrderType,
    }
}

define_struct! {
    struct NewTwapOrderArgs {
        side: Side,
        total_size: PositiveDecimal,
        reduce_only: bool,
        total_duration_seconds: u64,
    }
}

define_struct! {
    struct BackstopLiquidatePerpPositionArgs {
        market_id: MarketId,
        size: PositiveDecimal,
    }
}

define_struct! {
    struct CreateVaultArgs<Address> {
        name: CustomString,
        description: CustomString,
        leader: Address,
        deposit_asset_ids: Vec<AssetId>,
        withdraw_asset_id: AssetId,
        withdraw_lockup_period_hours: u8,
        whitelist_deposits: bool,
        profit_share_percentage: u8,
        withdrawal_fee_bps: u8,
        deposit_limit: PositiveDecimal,
    }
}
