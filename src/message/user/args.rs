//! Argument types for user operations.

use crate::decimals::PositiveDecimal;
use crate::string::CustomString;
use crate::types::{
    AssetId, BalanceBucket, ClientOrderId, MarketId, OrderId, OrderType, Side, TriggerDirection,
    TriggerPriceCondition,
};
use crate::{define_enum, define_struct};

define_struct! {
    /// A transfer location within an account: an optional sub-account and a
    /// balance bucket. `sub_account_index: None` targets the master account.
    struct TransferEndpoint {
        sub_account_index: Option<u8>,
        balance: BalanceBucket,
    }
}

define_struct! {
    /// A perp position within an account: an optional sub-account and the market
    /// whose ledger holds the position. `sub_account_index: None` targets the
    /// master account.
    ///
    /// The counterpart to [`TransferEndpoint`] for operations that move value
    /// between *positions* rather than between balance buckets. There is no
    /// bucket to name because unrealized PnL is not a balance — it lives in the
    /// position's cost basis.
    struct PerpPositionEndpoint {
        sub_account_index: Option<u8>,
        market_id: MarketId,
    }
}

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

define_enum! {
    /// The operation of a [`super::UserAction::DegenTrade`] message.
    #[non_exhaustive]
    #[strum_discriminants(non_exhaustive)]
    enum DegenAction {
        /// Queue an open against the market's degen vault. It fills at the oracle
        /// price of a later run, two oracle updates after it lands, so no price
        /// is given and none can be known in advance.
        ///
        /// `Side::Bid` opens a long, `Side::Ask` a short. `size` must be non-zero
        /// and a multiple of the market's lot size. `amount_to_transfer` is the
        /// margin, at least 1 USDC, taken from the cross balance now and returned
        /// if the entry expires. At the fill `size * price` must be at most 1000x
        /// the margin. The position settles at the oracle: the whole margin to the
        /// vault once 95% of it is lost, margin plus the win cap (100%) to the
        /// user at the win barrier, the clamped PnL on a close or timeout.
        Open {
            size: PositiveDecimal,
            side: Side,
            amount_to_transfer: PositiveDecimal,
        } = 0,
        /// Queue a close of the market's position; it settles at the oracle
        /// price of the run it fills in, two oracle updates after it lands. A
        /// legacy orderbook degen position is instead closed at once with an IOC
        /// order.
        Close = 1,
        /// Rejected: a queued open fills at the oracle price, so there is no
        /// fill to bound.
        OpenV1 {
            size: PositiveDecimal,
            side: Side,
            amount_to_transfer: PositiveDecimal,
            max_slippage_bps: Option<u16>,
        } = 2,
        /// Rejected, as [`DegenAction::OpenV1`].
        CloseV1 { max_slippage_bps: Option<u16> } = 3,
    }
}

define_struct! {
    struct BackstopLiquidatePerpPositionArgs {
        market_id: MarketId,
        size: PositiveDecimal,
    }
}

define_struct! {
    struct BackstopLiquidatePerpPositionArgsV1 {
        market_id: MarketId,
        size: PositiveDecimal,
        liquidator_side: Side,
        takeover_price: PositiveDecimal,
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
