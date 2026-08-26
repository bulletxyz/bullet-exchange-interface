//! User operations.

// Several variants below are `#[deprecated]` in favour of `Transfer`. The
// derived `borsh`/`serde`/`strum` impls read those variants' fields, which would
// otherwise raise in-crate deprecation warnings at the definition site. Lint
// levels are per-crate, so this only silences the noise here — downstream crates
// that construct the deprecated variants still get the deprecation warning.
#![allow(deprecated)]

use crate::decimals::PositiveDecimal;
use crate::define_enum;
use crate::string::CustomString;
use crate::time::UnixTimestampMicros;
use crate::types::{
    AssetId, MarketId, OrderIdKind, SpotCollateralTransferDirection, TradingMode, TriggerOrderId,
    TwapId,
};
mod args;
pub use args::*;

define_enum! {
    /// User operations requiring account ownership or delegation.
    ///
    /// These operations are authorized via `context.sender()` with optional delegate resolution.
    /// Operations include account management, trading, vault deposits, and user-initiated liquidations.
    #[non_exhaustive]
    #[strum_discriminants(non_exhaustive)]
    enum UserAction<Address> {
        // =========================================================================
        // Account Operations (0-19)
        // =========================================================================
        /// Deposit assets to perp margin account.
        #[deprecated(note = "use `UserAction::Transfer` instead")]
        Deposit {
            asset_id: AssetId,
            amount: PositiveDecimal,
        } = 0,

        /// Withdraw assets from perp margin account.
        #[deprecated(note = "use `UserAction::Transfer` instead")]
        Withdraw {
            asset_id: AssetId,
            amount: PositiveDecimal,
        } = 1,

        /// Deposit assets to spot collateral.
        #[deprecated(note = "use `UserAction::Transfer` instead")]
        DepositSpotCollateral {
            asset_id: AssetId,
            amount: PositiveDecimal,
        } = 2,

        /// Withdraw assets from spot collateral.
        #[deprecated(note = "use `UserAction::Transfer` instead")]
        WithdrawSpotCollateral {
            asset_id: AssetId,
            amount: PositiveDecimal,
        } = 3,

        /// Transfer assets between perp margin and spot collateral.
        #[deprecated(note = "use `UserAction::Transfer` instead")]
        TransferSpotCollateral {
            asset_id: AssetId,
            amount: PositiveDecimal,
            direction: SpotCollateralTransferDirection,
            sub_account_index: Option<u8>,
        } = 4,

        /// Borrow assets from spot pool.
        #[deprecated(note = "use `UserAction::Transfer` instead")]
        BorrowSpot {
            asset_id: AssetId,
            amount: PositiveDecimal,
            sub_account_index: Option<u8>,
        } = 5,

        /// Create a new sub-account.
        #[deprecated(note = "use `UserAction::Transfer` instead")]
        CreateSubAccount { index: u8 } = 6,

        /// Transfer assets between main account and sub-account.
        #[deprecated(note = "use `UserAction::Transfer` instead")]
        TransferToSubAccount {
            asset_id: AssetId,
            amount: PositiveDecimal,
            sub_account_index: u8,
            to_sub_account: bool,
        } = 7,

        /// Delegate trading permissions to another address.
        DelegateUser { delegate: Address, name: CustomString } = 8,

        /// Revoke delegation from an address.
        RevokeDelegation { delegate: Address } = 9,

        /// Update maximum leverage for a market.
        UpdateMaxLeverage {
            market_id: MarketId,
            max_leverage: u16,
            sub_account_index: Option<u8>,
        } = 10,

        /// Claim your own referral rewards.
        ClaimReferralRewards { asset_id: AssetId } = 11,

        /// Deposit USDC into an isolated market / isolated margin position.
        #[deprecated(note = "use `UserAction::Transfer` instead")]
        DepositIso {
            market_id: MarketId,
            amount: PositiveDecimal,
        } = 12,

        /// Withdraw USDC from an isolated market / isolated margin position.
        #[deprecated(note = "use `UserAction::Transfer` instead")]
        WithdrawIso {
            market_id: MarketId,
            amount: PositiveDecimal,
        } = 13,

        /// Set the trading mode of a perp ledger to cross or iso.
        ///
        /// A delegate may set it on the delegator's ledger.
        SetPerpLedgerTradingMode {
            market_id: MarketId,
            trading_mode: TradingMode,
            sub_account_index: Option<u8>,
        } = 14,

        /// Delegate trading permissions to another address allowing for sub accounts to be delegated.
        DelegateUserV1 {
            delegate: Address,
            name: CustomString,
            sub_account_index: Option<u8>,
        } = 15,

        /// Revoke delegation from an address allowing for sub accounts to revoke delegation.
        RevokeDelegationV1 {
            delegate: Address,
            sub_account_index: Option<u8>,
        } = 16,

        /// Delegate trading permissions with optional expiry and flags
        DelegateUserV2 {
            delegate: Address,
            name: CustomString,
            sub_account_index: Option<u8>,
            expires_at: Option<UnixTimestampMicros>,
            flags: u32,
        } = 17,

        /// Transfer an asset between two account/balance locations.
        ///
        /// Supersedes the deposit/withdraw/transfer variants above: `from` is
        /// always on the sender's account, while `to` combined with `to_address`
        /// identifies the destination (`to_address: None` = the sender's own
        /// account, `Some(addr)` = another account). `memo` carries a free-form
        /// end-to-end reference.
        Transfer {
            from: TransferEndpoint,
            to: TransferEndpoint,
            to_address: Option<Address>,
            asset_id: AssetId,
            amount: PositiveDecimal,
            memo: CustomString,
        } = 18,

        /// Set an account's self-trade group.
        ///
        /// Accounts sharing a `group` are treated as a single entity by the
        /// self-trade check, so they cannot trade against each other. A master
        /// and its subs, and a vault and its leader, are bound to each other
        /// independently of `group` and cannot be separated by it.
        ///
        /// `address` names the account to configure:
        /// - `None` — the sender's own account, with `sub_account_index` applied
        ///   as elsewhere.
        /// - `Some(addr)` — that account, and `sub_account_index` is **ignored**.
        ///   Allowed for the sender's own account, a sub-account whose recorded
        ///   master is the sender, a vault the sender leads, or any account if the
        ///   sender is the Protocol admin.
        ///
        /// A delegate may set a group, acting with its delegator's authority: on
        /// the delegator's own account and on the delegator's sub-accounts. A
        /// delegate of a vault may set that vault's group; a delegate of the
        /// *leader* may not, since vault configuration is leader-only.
        SetAccountGroup {
            address: Option<Address>,
            group: Address,
            sub_account_index: Option<u8>,
        } = 19,

        // =========================================================================
        // Order Operations (20-39)
        // =========================================================================
        /// Place new orders on a market.
        PlaceOrders {
            market_id: MarketId,
            orders: Vec<NewOrderArgs>,
            replace: bool,
            sub_account_index: Option<u8>,
        } = 20,

        /// Amend existing orders (cancel + place).
        AmendOrders {
            market_id: MarketId,
            orders: Vec<AmendOrderArgs>,
            sub_account_index: Option<u8>,
        } = 21,

        /// Cancel specific orders.
        CancelOrders {
            market_id: MarketId,
            orders: Vec<CancelOrderArgs>,
            sub_account_index: Option<u8>,
        } = 22,

        /// Cancel all orders on a market.
        CancelMarketOrders {
            market_id: MarketId,
            sub_account_index: Option<u8>,
        } = 23,

        /// Create trigger orders for spot markets.
        CreateTriggerOrders {
            market_id: MarketId,
            trigger_orders: Vec<NewTriggerOrderArgs>,
            sub_account_index: Option<u8>,
        } = 24,

        /// Create take-profit/stop-loss for a perp position.
        CreatePositionTpsl {
            market_id: MarketId,
            tpsl_pair: TpslPair,
            size: Option<PositiveDecimal>,
            sub_account_index: Option<u8>,
        } = 25,

        /// Cancel trigger orders.
        CancelTriggerOrders {
            market_id: MarketId,
            trigger_order_ids: Vec<TriggerOrderId>,
            sub_account_index: Option<u8>,
        } = 26,

        /// Create TWAP orders
        CreateTwapOrder {
            market_id: MarketId,
            twap_order_args: NewTwapOrderArgs,
            sub_account_index: Option<u8>,
        } = 27,

        /// Cancel a TWAP order
        CancelTwapOrder {
            market_id: MarketId,
            twap_id: TwapId,
            sub_account_index: Option<u8>,
        } = 28,

        /// Cancel all orders for perp or spot
        CancelAllOrders { sub_account_index: Option<u8> } = 29,

        /// Place (expiring) orders after canceling a list of orders.
        CancelAndPlaceOrders {
            market_id: MarketId,
            cancels: Vec<OrderIdKind>,
            places: Vec<NewOrderArgs>,
            expiry_timestamp: Option<UnixTimestampMicros>,
            sub_account_index: Option<u8>,
        } = 30,

        /// High-risk "degen" trading: an exclusive isolated position at up to
        /// 1000x leverage, funded with at most 10 USDC, executed top-of-book.
        ///
        /// See [`DegenAction`] for the open/close semantics. The user's perp
        /// ledger for the market must be empty on open; it is switched to
        /// [`TradingMode::Degen`] automatically.
        DegenTrade {
            market_id: MarketId,
            action: DegenAction,
            sub_account_index: Option<u8>,
        } = 31,

        // Reserved: 32-39


        // =========================================================================
        // Pool Operations (40-49)
        // =========================================================================
        /// Deposit USDC to the PnL pool.
        DepositToPnlPool { usdc_amount: PositiveDecimal } = 40,

        /// Settle user's PnL from the pool.
        SettleFromPnlPool { sub_account_index: Option<u8> } = 41,

        /// Deposit to the insurance fund.
        DepositToInsuranceFund { usdc_amount: PositiveDecimal } = 42,

        /// Deposit to protocol treasury.
        DepositToTreasury {
            asset_id: AssetId,
            amount: PositiveDecimal,
        } = 43,

        /// Claim accumulated borrow/lend protocol fees.
        ClaimBorrowLendFees {} = 44,

        /// Deposit to the iso insurance fund.
        DepositToIsoInsuranceFund { market_id: MarketId, amount: PositiveDecimal } = 45,
        // Reserved: 46-49

        // =========================================================================
        // Vault User Operations (50-59)
        // =========================================================================
        /// Create a new vault (caller becomes the vault leader).
        CreateVault { args: CreateVaultArgs<Address> } = 50,

        /// Deposit assets to a vault.
        DepositToVault {
            vault_address: Address,
            asset_id: AssetId,
            amount: PositiveDecimal,
        } = 51,

        /// Queue a withdrawal from a vault.
        QueueWithdrawal {
            vault_address: Address,
            shares: PositiveDecimal,
        } = 52,

        /// Cancel a queued withdrawal.
        CancelQueuedWithdrawal { vault_address: Address } = 53,

        /// Force withdraw from a vault (bypasses queue).
        ForceWithdrawVault {
            vault_address: Address,
            shares: PositiveDecimal,
        } = 54,

        // Reserved: 55-59

        // =========================================================================
        // User-Initiated Liquidation Operations (60-69)
        // =========================================================================
        /// Backstop liquidation for perp positions (user provides capital).
        BackstopLiquidatePerpPositions {
            address: Address,
            positions: Option<Vec<BackstopLiquidatePerpPositionArgs>>,
            sub_account_index: Option<u8>,
        } = 60,

        /// Liquidate borrow/lend liability (user provides capital).
        LiquidateBorrowLendLiability {
            liquidatee_address: Address,
            liability_asset_id: AssetId,
            collateral_asset_id: AssetId,
            liability_amount: PositiveDecimal,
            sub_account_index: Option<u8>,
        } = 61,

        BackstopLiquidateIsoPerpPosition {
            address: Address,
            position: BackstopLiquidatePerpPositionArgs,
            sub_account_index: Option<u8>,
        } = 62,
        // Reserved: 63-255
    }
}
