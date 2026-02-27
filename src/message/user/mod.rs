//! User operations.

use crate::decimals::PositiveDecimal;
use crate::define_enum;
use crate::string::CustomString;
use crate::types::{AssetId, MarketId, SpotCollateralTransferDirection, TriggerOrderId, TwapId};
mod args;
pub use args::*;

define_enum! {
    /// User operations requiring account ownership or delegation.
    ///
    /// These operations are authorized via `context.sender()` with optional delegate resolution.
    /// Operations include account management, trading, vault deposits, and user-initiated liquidations.
    enum UserAction<Address> {
        // =========================================================================
        // Account Operations (0-19)
        // =========================================================================
        /// Deposit assets to perp margin account.
        Deposit {
            asset_id: AssetId,
            amount: PositiveDecimal,
        } = 0,

        /// Withdraw assets from perp margin account.
        Withdraw {
            asset_id: AssetId,
            amount: PositiveDecimal,
        } = 1,

        /// Deposit assets to spot collateral.
        DepositSpotCollateral {
            asset_id: AssetId,
            amount: PositiveDecimal,
        } = 2,

        /// Withdraw assets from spot collateral.
        WithdrawSpotCollateral {
            asset_id: AssetId,
            amount: PositiveDecimal,
        } = 3,

        /// Transfer assets between perp margin and spot collateral.
        TransferSpotCollateral {
            asset_id: AssetId,
            amount: PositiveDecimal,
            direction: SpotCollateralTransferDirection,
            sub_account_index: Option<u8>,
        } = 4,

        /// Borrow assets from spot pool.
        BorrowSpot {
            asset_id: AssetId,
            amount: PositiveDecimal,
            sub_account_index: Option<u8>,
        } = 5,

        /// Create a new sub-account.
        CreateSubAccount { index: u8 } = 6,

        /// Transfer assets between main account and sub-account.
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

        // Reserved: 12-19

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
        // Reserved: 30-39

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
        // Reserved: 45-49

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
        // Reserved: 62-255
    }
}
