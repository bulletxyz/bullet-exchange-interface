use rust_decimal::Decimal;

use crate::decimals::PositiveDecimal;
use crate::time::UnixTimestampMicros;
use crate::types::{
    AssetId, BorrowType, ClientOrderId, FeeTier, MarketId, OrderId, OrderType, RepayType, Side,
    TakeFromInsuranceFundReason, TradeId, TriggerDirection, TriggerOrderId, TriggerPriceCondition,
    TwapId,
};

#[derive(
    borsh::BorshDeserialize,
    borsh::BorshSerialize,
    serde::Serialize,
    serde::Deserialize,
    PartialEq,
    Clone,
    Debug,
    schemars::JsonSchema,
    strum::AsRefStr,
    strum::Display,
)]
#[serde(rename_all = "snake_case")]
#[schemars(rename = "FillType")]
pub enum FillType {
    Orderbook,
    Liquidation,
    BackstopLiquidation,
    #[serde(rename = "adl")]
    ADL,
}

#[derive(
    borsh::BorshDeserialize,
    borsh::BorshSerialize,
    serde::Serialize,
    serde::Deserialize,
    PartialEq,
    Eq,
    Clone,
    Copy,
    Debug,
    schemars::JsonSchema,
    strum::AsRefStr,
    strum::Display,
)]
#[serde(rename_all = "snake_case")]
#[schemars(rename = "CancelReason")]
pub enum CancelReason {
    // user-initiated
    /// User invoked a cancel action directly (CancelOrders / CancelMarketOrders /
    /// CancelAllOrders / CancelTriggerOrders / CancelTwapOrder)
    UserRequested,
    /// User amended an order (cancel + replace)
    Amended,
    /// User placed orders with replace=true; existing market orders wiped
    Replaced,

    // admin-initiated
    /// AdminAction::CancelOrders / AdminCancelTriggerOrders
    AdminRequested,
    /// Admin pruned the market
    MarketPruned,
    /// Market was halted; resting orders cleared
    MarketHalted,

    // risk-driven
    /// Account undercollateralized — covers cross-margin
    /// (PublicAction::ForceCancelOrders) and iso-margin
    /// (PublicAction::ForceCancelIsoOrders). The cancelled order's margin
    /// mode disambiguates which one fired.
    MarginCall,
    /// User's resting orders cancelled as part of regular liquidation
    /// (public/liquidate_perp_positions, public/liquidate_iso_perp_position)
    Liquidation,
    /// User's resting orders cancelled as part of backstop / insurance-fund
    /// liquidation. Applies to liquidatee and any liquidator whose trigger
    /// orders on that market are cleared during takeover.
    BackstopLiquidation,

    // matching-engine / lifecycle
    /// Reduce-only order auto-canceled because the position it would have
    /// reduced shrank to zero
    ReduceOnlyZeroSize,

    /// A trigger that would have opened/added position was
    /// auto-canceled because a fill made it stale
    OpeningTriggerOrphaned,

    /// Linked TPSL sibling canceled because its pair leg fired, was
    /// rejected, or failed during execution
    TpslSiblingCancelled,

    /// Trigger fired but couldn't execute — no position to close,
    /// post-execution margin failure, no liquidity, or runtime error.
    /// The accompanying RejectTriggerOrder / FailureExecuteTriggerOrder
    /// event in the same tx carries the specific cause.
    TriggerExecutionFailed,

    /// TWAP slice schedule exhausted (next-slice size rounded to zero)
    TwapCompleted,

    /// Order evicted to make room when orderbook hit its capacity limit
    OrderbookOverflow,

    /// TWAP slice fired but couldn't execute — runtime error, no liquidity, etc
    TwapExecutionFailed,

    /// User's resting orders cancelled because their position is being
    /// force-closed via auto-deleverage (last-resort protocol action when
    /// backstop liquidation can't absorb the loss). Applies to both ADL
    /// counterparties.
    AutoDeleverage,
}

#[derive(
    borsh::BorshDeserialize,
    borsh::BorshSerialize,
    serde::Serialize,
    serde::Deserialize,
    PartialEq,
    Clone,
    Debug,
    schemars::JsonSchema,
    strum::AsRefStr,
    strum::Display,
)]
#[serde(rename_all = "snake_case")]
#[schemars(rename = "Event")]
pub enum Event<Address> {
    /// Market initialized
    InitializePerpMarket {
        market_id: MarketId,
        execution_timestamp: UnixTimestampMicros,
    },
    /// Market updated
    UpdateMarket {
        market_id: MarketId,
        execution_timestamp: UnixTimestampMicros,
    },
    /// Market deleted
    DeleteMarket {
        market_id: MarketId,
        execution_timestamp: UnixTimestampMicros,
    },
    /// Borrow lend initialized
    InitializeBorrowLendPool {
        asset_id: AssetId,
        execution_timestamp: UnixTimestampMicros,
    },
    /// Borrow lend updated
    UpdateBorrowLendPool {
        asset_id: AssetId,
        execution_timestamp: UnixTimestampMicros,
    },
    /// Deposit
    Deposit {
        user_address: Address,
        asset_id: AssetId,
        amount: PositiveDecimal,
        amount_notional: PositiveDecimal,
        execution_timestamp: UnixTimestampMicros,
    },
    /// Order placed
    PlaceOrder {
        user_address: Address,
        order_id: OrderId,
        market_id: MarketId,
        price: PositiveDecimal,
        size: PositiveDecimal,
        side: Side,
        order_type: OrderType,
        source: OrderSource,
        execution_timestamp: UnixTimestampMicros,
        client_order_id: Option<ClientOrderId>,
    },
    /// Order canceled
    CancelOrder {
        user_address: Address,
        order_id: OrderId,
        market_id: MarketId,
        execution_timestamp: UnixTimestampMicros,
        client_order_id: Option<ClientOrderId>,
    },
    /// Force cancel orders, occurs when a user's account is at risk
    ForceCancelOrders {
        user_address: Address,
        cancelled_orders: Vec<(MarketId, Vec<OrderId>)>,
        cancelled_trigger_orders: Vec<(MarketId, Vec<TriggerOrderId>)>,
        execution_timestamp: UnixTimestampMicros,
    },
    /// Trigger order created
    CreateTriggerOrder {
        user_address: Address,
        trigger_order_id: TriggerOrderId,
        market_id: MarketId,
        order_price: PositiveDecimal,
        trigger_price: PositiveDecimal,
        trigger_direction: TriggerDirection,
        price_condition: TriggerPriceCondition,
        size: Option<PositiveDecimal>,
        side: Side,
        order_type: OrderType,
        execution_timestamp: UnixTimestampMicros,
    },
    /// Trigger order activated for execution
    ActivateTriggerOrder {
        trigger_order_id: TriggerOrderId,
        market_id: MarketId,
        price_condition: TriggerPriceCondition,
        execution_timestamp: UnixTimestampMicros,
    },
    /// Edit existing trigger order active / inactive
    EditTriggerOrder {
        user_address: Address,
        trigger_order_id: TriggerOrderId,
        size_added: Option<PositiveDecimal>,
        execution_timestamp: UnixTimestampMicros,
    },
    /// Active trigger order executed from queue
    TryExecuteTriggerOrder {
        user_address: Address,
        trigger_order_id: TriggerOrderId,
        execution_timestamp: UnixTimestampMicros,
    },
    /// Active trigger order successfully executed
    SuccessfulExecuteTriggerOrder {
        user_address: Address,
        trigger_order_id: TriggerOrderId,
        market_id: MarketId,
        order_price: PositiveDecimal,
        trigger_price: PositiveDecimal,
        trigger_direction: TriggerDirection,
        price_condition: TriggerPriceCondition,
        executed_size: PositiveDecimal,
        side: Side,
        order_type: OrderType,
        execution_timestamp: UnixTimestampMicros,
    },
    FailureExecuteTriggerOrder {
        user_address: Address,
        trigger_order_id: TriggerOrderId,
        reason: String,
        execution_timestamp: UnixTimestampMicros,
    },
    /// Trigger order cancelled (active / inactive)
    CancelTriggerOrder {
        user_address: Address,
        trigger_order_id: TriggerOrderId,
        market_id: MarketId,
        execution_timestamp: UnixTimestampMicros,
    },
    /// Active trigger order rejected while trying to be executed
    RejectTriggerOrder {
        user_address: Address,
        trigger_order_id: TriggerOrderId,
        reason: String,
        execution_timestamp: UnixTimestampMicros,
    },
    /// Trigger order being executed reactivated
    ReactivateTriggerOrder {
        user_address: Address,
        trigger_order_id: TriggerOrderId,
        reason: String,
        execution_timestamp: UnixTimestampMicros,
    },
    /// Twap order created
    CreateTwapOrder {
        user_address: Address,
        twap_id: TwapId,
        market_id: MarketId,
        total_size: PositiveDecimal,
        total_duration_seconds: u64,
        side: Side,
        reduce_only: bool,
    },
    /// Twap order successfully executed
    SuccessfulExecuteTwapOrder {
        user_address: Address,
        twap_id: TwapId,
        market_id: MarketId,
        order_price: PositiveDecimal,
        executed_size: PositiveDecimal,
        side: Side,
        order_type: OrderType,
        execution_timestamp: UnixTimestampMicros,
    },
    /// Active twap order rejected while trying to be executed
    RejectTwapOrder {
        user_address: Address,
        twap_id: TwapId,
        reason: String,
        execution_timestamp: UnixTimestampMicros,
    },
    /// Whole twap order cancelled
    CancelTwap {
        user_address: Address,
        twap_id: TwapId,
        execution_timestamp: UnixTimestampMicros,
    },

    /// Premium Index Updated
    UpdatePremiumIndex {
        market_id: MarketId,
        premium_index: Decimal,
        execution_timestamp: UnixTimestampMicros,
    },
    /// Premium Index Update Failed
    UpdatePremiumIndexFailed {
        market_id: MarketId,
        error: String,
        execution_timestamp: UnixTimestampMicros,
    },
    /// Funding Rate Updated
    UpdateFundingRate {
        market_id: MarketId,
        funding_rate: Decimal,
        execution_timestamp: UnixTimestampMicros,
    },
    /// Funding Rate Update Failed
    UpdateFundingRateFailed {
        market_id: MarketId,
        error: String,
        execution_timestamp: UnixTimestampMicros,
    },
    /// Funding Rate Applied
    ApplyFundingRate {
        user_address: Address,
        market_id: MarketId,
        funding_applied: Decimal,
        execution_timestamp: UnixTimestampMicros,
    },
    /// Funding Rate Application Failed
    ApplyFundingRateFailed {
        user_address: Address,
        error: String,
        execution_timestamp: UnixTimestampMicros,
    },
    /// Oracle Price Updated
    UpdateOraclePrice {
        asset_id: AssetId,
        oracle_price: PositiveDecimal,
        publish_timestamp: UnixTimestampMicros,
        execution_timestamp: UnixTimestampMicros,
    },
    /// Mark Price Updated
    UpdateMarkPrice {
        market_id: MarketId,
        median_cex_price: PositiveDecimal,
        diff_ema: Decimal,
        publish_timestamp: UnixTimestampMicros,
        execution_timestamp: UnixTimestampMicros,
    },
    /// Price Update Failed
    UpdateOraclePriceFailed {
        asset_id: AssetId,
        error: String,
        execution_timestamp: UnixTimestampMicros,
    },
    /// Mark Price Update Failed
    UpdateMarkPriceFailed {
        market_id: MarketId,
        error: String,
        execution_timestamp: UnixTimestampMicros,
    },
    Borrow {
        user_address: Address,
        asset_id: AssetId,
        amount: PositiveDecimal,
        borrow_type: BorrowType,
        execution_timestamp: UnixTimestampMicros,
    },
    RepayBorrow {
        user_address: Address,
        asset_id: AssetId,
        amount: PositiveDecimal,
        repay_type: RepayType,
        execution_timestamp: UnixTimestampMicros,
    },
    UsdcUnrealizedLossBorrowRebalance {
        user_address: Address,
        cached_unrealized_loss: PositiveDecimal,
        internal_repayment: PositiveDecimal,
        execution_timestamp: UnixTimestampMicros,
    },
    Withdraw {
        user_address: Address,
        asset_id: AssetId,
        amount: PositiveDecimal,
        amount_notional: PositiveDecimal,
        fee: PositiveDecimal,
        execution_timestamp: UnixTimestampMicros,
    },
    Trade {
        user_address: Address,
        market_id: MarketId,
        price: PositiveDecimal,
        size: PositiveDecimal,
        side: Side,
        order_id: OrderId,
        is_maker: bool,
        is_full_fill: bool,
        realized_pnl: Decimal,
        fee: Decimal,
        net_fee: PositiveDecimal, /* Taker fee netted against maker rebate. For perps this is
                                   * USDC, for spot it is in the maker's collateral asset */
        trade_id: TradeId,
        is_liquidation: bool,
        client_order_id: Option<ClientOrderId>,
        execution_timestamp: UnixTimestampMicros,
        fee_asset: AssetId,
    },
    ForceSettlePerpPosition {
        user_address: Address,
        market_id: MarketId,
        price: PositiveDecimal,
        size: PositiveDecimal,
        side: Side,
        realized_pnl: Decimal,
        execution_timestamp: UnixTimestampMicros,
    },
    CleanupUserMarketState {
        user_address: Address,
        market_id: MarketId,
        execution_timestamp: UnixTimestampMicros,
    },
    LiquidateBorrowLendLiability {
        liquidatee_address: Address,
        liquidator_address: Address,
        liability_asset_id: AssetId,
        collateral_asset_id: AssetId,
        liability_oracle_price: PositiveDecimal,
        liability_size: PositiveDecimal,
        collateral_size: PositiveDecimal,
        liquidator_reward: PositiveDecimal,
        protocol_reward: PositiveDecimal,
        execution_timestamp: UnixTimestampMicros,
    },
    BackstopLiquidatePerpPosition {
        liquidatee_address: Address,
        liquidator_address: Address,
        market_id: MarketId,
        mark_price: PositiveDecimal,
        size: PositiveDecimal,
        side: Side,
        realized_pnl: Decimal,
        liquidator_reward: PositiveDecimal,
        protocol_reward: PositiveDecimal,
        execution_timestamp: UnixTimestampMicros,
    },
    BackstopLiquidatePerp {
        liquidatee_address: Address,
        liquidator_address: Address,
        liquidator_reward: PositiveDecimal,
        protocol_reward: PositiveDecimal,
        liquidation_loss: Decimal,
        execution_timestamp: UnixTimestampMicros,
    },
    AutoDeleverage {
        user: Address,
        counterparty: Address,
        market_id: MarketId,
        fill_price: PositiveDecimal,
        size: PositiveDecimal,
        user_realized_pnl: Decimal,
        counterparty_realized_pnl: Decimal,
        execution_timestamp: UnixTimestampMicros,
    },
    UpdateReduceOnlyLimitOrder {
        user_address: Address,
        order_id: OrderId,
        market_id: MarketId,
        previous_size: PositiveDecimal,
        new_size: PositiveDecimal,
        execution_timestamp: UnixTimestampMicros,
    },
    BootOrder {
        user_address: Address,
        order_id: OrderId,
        market_id: MarketId,
        execution_timestamp: UnixTimestampMicros,
    },
    AccrueInterestOnBorrowLend {
        asset_id: AssetId,
        utilization_rate: PositiveDecimal,
        time_elapsed: u64,
        borrow_rate: PositiveDecimal,
        prev_cumulative_borrow_rate: PositiveDecimal,
        new_cumulative_borrow_rate: PositiveDecimal,
        prev_cumulative_deposit_rate: PositiveDecimal,
        new_cumulative_deposit_rate: PositiveDecimal,
        new_debt: PositiveDecimal,
        protocol_reward: PositiveDecimal,
        execution_timestamp: UnixTimestampMicros,
    },
    InitializeSpotMarket {
        market_id: MarketId,
        execution_timestamp: UnixTimestampMicros,
    },
    /// Deposit
    DepositVault {
        vault_address: Address,
        user_address: Address,
        asset_id: AssetId,
        amount: PositiveDecimal,
        amount_notional: PositiveDecimal,
        shares: PositiveDecimal,
        execution_timestamp: UnixTimestampMicros,
    },
    /// Withdraw
    QueueWithdrawVault {
        vault_address: Address,
        user_address: Address,
        shares: PositiveDecimal,
        execution_timestamp: UnixTimestampMicros,
    },
    /// Withdraw
    ProcessWithdrawVault {
        vault_address: Address,
        user_address: Address,
        shares: PositiveDecimal,
        amount: PositiveDecimal,
        asset_id: AssetId,
        execution_timestamp: UnixTimestampMicros,
    },
    CollectVaultFees {
        vault_address: Address,
        high_watermark: PositiveDecimal,
        shares_minted_to_leader: PositiveDecimal,
        execution_timestamp: UnixTimestampMicros,
    },
    DepositSpotCollateral {
        user_address: Address,
        asset_id: AssetId,
        amount: PositiveDecimal,
        execution_timestamp: UnixTimestampMicros,
    },
    // deprecated - can't be removed since used in old slots and discriminators have to stay
    // constant
    WithdrawSpotCollateral {
        user_address: Address,
        asset_id: AssetId,
        amount: PositiveDecimal,
        execution_timestamp: UnixTimestampMicros,
    },
    HaltPerpMarket {
        market_id: MarketId,
        settlement_price: PositiveDecimal,
        execution_timestamp: UnixTimestampMicros,
    },
    HaltSpotMarket {
        market_id: MarketId,
        execution_timestamp: UnixTimestampMicros,
    },
    UnhaltPerpMarket {
        market_id: MarketId,
        execution_timestamp: UnixTimestampMicros,
    },
    UnhaltSpotMarket {
        market_id: MarketId,
        execution_timestamp: UnixTimestampMicros,
    },
    HaltBorrowLendPool {
        asset_id: AssetId,
        execution_timestamp: UnixTimestampMicros,
    },
    UnhaltBorrowLendPool {
        asset_id: AssetId,
        execution_timestamp: UnixTimestampMicros,
    },
    AdminAddTradingCredits {
        user_address: Address,
        amount: PositiveDecimal,
        execution_timestamp: UnixTimestampMicros,
    },
    AdminRemoveTradingCredits {
        user_address: Address,
        amount: PositiveDecimal,
        execution_timestamp: UnixTimestampMicros,
    },
    UseTradingCredits {
        user_address: Address,
        amount: PositiveDecimal,
        execution_timestamp: UnixTimestampMicros,
    },
    ClaimReferralRewards {
        address: Address,
        amount_claimed: PositiveDecimal,
        total_rewards: PositiveDecimal,
    },
    UpdateUserFeeTier {
        user_address: Address,
        fee_tier: FeeTier,
        execution_timestamp: UnixTimestampMicros,
    },
    UpdateUserFeeDiscountBps {
        user_address: Address,
        fee_discount_bps: u16,
        execution_timestamp: UnixTimestampMicros,
    },
    WithdrawSpotCollateralV2 {
        user_address: Address,
        asset_id: AssetId,
        amount: PositiveDecimal,
        execution_timestamp: UnixTimestampMicros,
        fee: PositiveDecimal,
    },
    /// Asset is deleted.
    DeleteAsset {
        asset_id: AssetId,
        execution_timestamp: UnixTimestampMicros,
    },
    /// Trigger Orders are pending and should be executed.
    PendingTriggerOrders {
        market_id: MarketId,
        execution_timestamp: UnixTimestampMicros,
    },
    DelegateUser {
        delegator: Address,
        delegate: Address,
        name: String,
        execution_timestamp: UnixTimestampMicros,
    },
    RevokeDelegation {
        delegator: Address,
        delegate: Address,
        execution_timestamp: UnixTimestampMicros,
    },
    AdminRevokeDelegation {
        delegator: Address,
        delegate: Address,
        execution_timestamp: UnixTimestampMicros,
    },
    AdminDeleteDelegateConfig {
        delegator: Address,
        delegate: Address,
        name: String,
        execution_timestamp: UnixTimestampMicros,
    },
    DepositIso {
        user_address: Address,
        market_id: MarketId,
        amount: PositiveDecimal,
        execution_timestamp: UnixTimestampMicros,
    },
    WithdrawIso {
        user_address: Address,
        market_id: MarketId,
        amount: PositiveDecimal,
        fee: PositiveDecimal,
        execution_timestamp: UnixTimestampMicros,
    },
    TakeFromInsuranceFund {
        reason: TakeFromInsuranceFundReason,
        amount: PositiveDecimal,
        execution_timestamp: UnixTimestampMicros,
    },
    DelegateUserV1 {
        delegator: Address,
        delegate: Address,
        name: String,
        expires_at: Option<UnixTimestampMicros>,
        flags: u32,
        execution_timestamp: UnixTimestampMicros,
    },
    // supersedes Trade; adds cumulative order progress (filled_size, filled_cot, remaining_size)
    TradeV1 {
        user_address: Address,
        market_id: MarketId,
        price: PositiveDecimal,
        size: PositiveDecimal,
        side: Side,
        order_id: OrderId,
        is_maker: bool,
        is_full_fill: bool,
        realized_pnl: Decimal,
        fee: Decimal,
        net_fee: PositiveDecimal,
        trade_id: TradeId,
        client_order_id: Option<ClientOrderId>,
        execution_timestamp: UnixTimestampMicros,
        fee_asset: AssetId,
        fill_type: FillType,
        // None for OTC fills (backstop liquidation, ADL): these originate from a position,
        // not an order, so per-order cumulative progress is undefined. Some(_) for all
        // matching-engine fills.
        cumulative_filled_size: Option<PositiveDecimal>,
        cumulative_filled_cot: Option<PositiveDecimal>,
        // None for OTC fills
        remaining_size: Option<PositiveDecimal>,
    },
    InitializeAssetInfo {
        asset_id: AssetId,
        name: String,
        execution_timestamp: UnixTimestampMicros,
    },
    InitializePerpMarketV1 {
        market_id: MarketId,
        name: String,
        base_asset_id: AssetId,
        execution_timestamp: UnixTimestampMicros,
    },
    InitializeSpotMarketV1 {
        market_id: MarketId,
        name: String,
        base_asset_id: AssetId,
        quote_asset_id: AssetId,
        execution_timestamp: UnixTimestampMicros,
    },
    /// supersedes CancelOrder; adds reason
    CancelOrderV1 {
        user_address: Address,
        order_id: OrderId,
        market_id: MarketId,
        execution_timestamp: UnixTimestampMicros,
        client_order_id: Option<ClientOrderId>,
        reason: CancelReason,
    },
    /// supersedes CancelTriggerOrder; adds reason
    CancelTriggerOrderV1 {
        user_address: Address,
        trigger_order_id: TriggerOrderId,
        market_id: MarketId,
        execution_timestamp: UnixTimestampMicros,
        reason: CancelReason,
    },
    /// supersedes CancelTwap; adds reason
    CancelTwapV1 {
        user_address: Address,
        twap_id: TwapId,
        market_id: MarketId,
        execution_timestamp: UnixTimestampMicros,
        reason: CancelReason,
    },
}

impl<Address> Event<Address> {
    pub fn event_key(&self) -> &'static str {
        match self {
            Self::AccrueInterestOnBorrowLend { .. } => "Exchange/AccrueInterestOnBorrowLend",
            Self::ActivateTriggerOrder { .. } => "Exchange/ActivateTriggerOrder",
            Self::AdminAddTradingCredits { .. } => "Exchange/AdminAddTradingCredits",
            Self::AdminRemoveTradingCredits { .. } => "Exchange/AdminRemoveTradingCredits",
            Self::ApplyFundingRate { .. } => "Exchange/ApplyFundingRate",
            Self::ApplyFundingRateFailed { .. } => "Exchange/ApplyFundingRateFailed",
            Self::AutoDeleverage { .. } => "Exchange/AutoDeleverage",
            Self::BackstopLiquidatePerp { .. } => "Exchange/BackstopLiquidatePerp",
            Self::BackstopLiquidatePerpPosition { .. } => "Exchange/BackstopLiquidatePerpPosition",
            Self::BootOrder { .. } => "Exchange/BootOrder",
            Self::Borrow { .. } => "Exchange/Borrow",
            Self::CancelOrder { .. } => "Exchange/CancelOrder",
            Self::CancelTriggerOrder { .. } => "Exchange/CancelTriggerOrder",
            Self::CancelTwap { .. } => "Exchange/CancelTwap",
            Self::ClaimReferralRewards { .. } => "Exchange/ClaimReferralRewards",
            Self::CleanupUserMarketState { .. } => "Exchange/CleanupUserMarketState",
            Self::CollectVaultFees { .. } => "Exchange/CollectVaultFees",
            Self::CreateTriggerOrder { .. } => "Exchange/CreateTriggerOrder",
            Self::CreateTwapOrder { .. } => "Exchange/CreateTwapOrder",
            Self::DeleteMarket { .. } => "Exchange/DeleteMarket",
            Self::DeleteAsset { .. } => "Exchange/DeleteAsset",
            Self::Deposit { .. } => "Exchange/Deposit",
            Self::DepositSpotCollateral { .. } => "Exchange/DepositSpotCollateral",
            Self::DepositVault { .. } => "Exchange/DepositVault",
            Self::EditTriggerOrder { .. } => "Exchange/EditTriggerOrder",
            Self::FailureExecuteTriggerOrder { .. } => "Exchange/FailureExecuteTriggerOrder",
            Self::ForceCancelOrders { .. } => "Exchange/ForceCancelOrders",
            Self::ForceSettlePerpPosition { .. } => "Exchange/ForceSettlePerpPosition",
            Self::HaltBorrowLendPool { .. } => "Exchange/HaltBorrowLendPool",
            Self::HaltPerpMarket { .. } => "Exchange/HaltPerpMarket",
            Self::HaltSpotMarket { .. } => "Exchange/HaltSpotMarket",
            Self::InitializeBorrowLendPool { .. } => "Exchange/InitializeBorrowLendPool",
            Self::InitializePerpMarket { .. } => "Exchange/InitializePerpMarket",
            Self::InitializeSpotMarket { .. } => "Exchange/InitializeSpotMarket",
            Self::LiquidateBorrowLendLiability { .. } => "Exchange/LiquidateBorrowLendLiability",
            Self::PendingTriggerOrders { .. } => "Exchange/PendingTriggerOrders",
            Self::PlaceOrder { .. } => "Exchange/PlaceOrder",
            Self::ProcessWithdrawVault { .. } => "Exchange/ProcessWithdrawVault",
            Self::QueueWithdrawVault { .. } => "Exchange/QueueWithdrawVault",
            Self::ReactivateTriggerOrder { .. } => "Exchange/ReactivateTriggerOrder",
            Self::RejectTriggerOrder { .. } => "Exchange/RejectTriggerOrder",
            Self::RejectTwapOrder { .. } => "Exchange/RejectTwapOrder",
            Self::RepayBorrow { .. } => "Exchange/RepayBorrow",
            Self::SuccessfulExecuteTriggerOrder { .. } => "Exchange/SuccessfulExecuteTriggerOrder",
            Self::SuccessfulExecuteTwapOrder { .. } => "Exchange/SuccessfulExecuteTwapOrder",
            Self::Trade { .. } => "Exchange/Trade",
            Self::TradeV1 { .. } => "Exchange/TradeV1",
            Self::TryExecuteTriggerOrder { .. } => "Exchange/TryExecuteTriggerOrder",
            Self::UnhaltBorrowLendPool { .. } => "Exchange/UnhaltBorrowLendPool",
            Self::UnhaltPerpMarket { .. } => "Exchange/UnhaltPerpMarket",
            Self::UnhaltSpotMarket { .. } => "Exchange/UnhaltSpotMarket",
            Self::UpdateBorrowLendPool { .. } => "Exchange/UpdateBorrowLendPool",
            Self::UpdateFundingRate { .. } => "Exchange/UpdateFundingRate",
            Self::UpdateFundingRateFailed { .. } => "Exchange/UpdateFundingRateFailed",
            Self::UpdateMarkPrice { .. } => "Exchange/UpdateMarkPrice",
            Self::UpdateMarkPriceFailed { .. } => "Exchange/UpdateMarkPriceFailed",
            Self::UpdateMarket { .. } => "Exchange/UpdateMarket",
            Self::UpdateOraclePrice { .. } => "Exchange/UpdateOraclePrice",
            Self::UpdateOraclePriceFailed { .. } => "Exchange/UpdateOraclePriceFailed",
            Self::UpdatePremiumIndex { .. } => "Exchange/UpdatePremiumIndex",
            Self::UpdatePremiumIndexFailed { .. } => "Exchange/UpdatePremiumIndexFailed",
            Self::UpdateReduceOnlyLimitOrder { .. } => "Exchange/UpdateReduceOnlyLimitOrder",
            Self::UpdateUserFeeDiscountBps { .. } => "Exchange/UpdateUserFeeDiscountBps",
            Self::UpdateUserFeeTier { .. } => "Exchange/UpdateUserFeeTier",
            Self::UsdcUnrealizedLossBorrowRebalance { .. } => {
                "Exchange/UsdcUnrealizedLossBorrowRebalance"
            }
            Self::UseTradingCredits { .. } => "Exchange/UseTradingCredits",
            Self::Withdraw { .. } => "Exchange/Withdraw",
            Self::WithdrawSpotCollateral { .. } => "Exchange/WithdrawSpotCollateral",
            Self::WithdrawSpotCollateralV2 { .. } => "Exchange/WithdrawSpotCollateralV2",
            Self::DelegateUser { .. } => "Exchange/DelegateUser",
            Self::RevokeDelegation { .. } => "Exchange/RevokeDelegation",
            Self::AdminRevokeDelegation { .. } => "Exchange/AdminRevokeDelegation",
            Self::AdminDeleteDelegateConfig { .. } => "Exchange/AdminDeleteDelegateConfig",
            Self::DepositIso { .. } => "Exchange/DepositIso",
            Self::WithdrawIso { .. } => "Exchange/WithdrawIso",
            Self::TakeFromInsuranceFund { .. } => "Exchange/TakeFromInsuranceFund",
            Self::DelegateUserV1 { .. } => "Exchange/DelegateUserV1",
            Self::InitializeAssetInfo { .. } => "Exchange/InitializeAssetInfo",
            Self::InitializePerpMarketV1 { .. } => "Exchange/InitializePerpMarketV1",
            Self::InitializeSpotMarketV1 { .. } => "Exchange/InitializeSpotMarketV1",
            Self::CancelOrderV1 { .. } => "Exchange/CancelOrderV1",
            Self::CancelTriggerOrderV1 { .. } => "Exchange/CancelTriggerOrderV1",
            Self::CancelTwapV1 { .. } => "Exchange/CancelTwapV1",
        }
    }
}

crate::define_simple_enum!(OrderSource { Admin, Liquidate, User, Trigger, Twap });
