//! Protocol admin operations.

use crate::decimals::PositiveDecimal;
use crate::define_enum;
use crate::message::CreateVaultArgs;
use crate::types::{AdminType, AssetId, MarketId, OrderId, TriggerOrderId};

mod args;
pub use args::*;

define_enum! {
    /// Protocol admin operations.
    ///
    /// These operations require Protocol admin authorization and are used for
    /// protocol-level configuration, market management, and emergency actions.
    #[non_exhaustive]
    enum AdminAction<Address> {
        // =========================================================================
        // Market Operations (0-19)
        // =========================================================================
        /// Initialize a new perp market.
        InitPerpMarket { args: InitPerpMarketArgs } = 0,

        /// Update perp market configuration.
        UpdatePerpMarket { args: UpdatePerpMarketArgs } = 1,

        /// Initialize a new spot market.
        InitSpotMarket { args: InitSpotMarketArgs } = 2,

        /// Update spot market configuration.
        UpdateSpotMarket { args: UpdateSpotMarketArgs } = 3,

        /// Halt a perp market with settlement price.
        HaltPerpMarket {
            market_id: MarketId,
            settlement_price: PositiveDecimal,
        } = 4,

        /// Unhalt a perp market.
        UnhaltPerpMarket { market_id: MarketId } = 5,

        /// Halt a spot market.
        HaltSpotMarket { market_id: MarketId } = 6,

        /// Unhalt a spot market.
        UnhaltSpotMarket { market_id: MarketId } = 7,

        /// Prune market data.
        PruneMarket { market_id: MarketId } = 8,

        /// Delete a market.
        DeleteMarket { market_id: MarketId } = 9,

        /// Cleanup user market state.
        CleanupUserMarketState {
            market_id: MarketId,
            users: Vec<Address>,
        } = 10,

        /// Update perp market leverage table.
        UpdatePerpLeverageTable {
            market_id: MarketId,
            args: SurrogateLeverageTableArgs,
        } = 11,

        /// Delete an asset.
        DeleteAsset { asset_id: AssetId } = 12,

        // Reserved: 13-19

        // =========================================================================
        // Asset Operations (20-29)
        // =========================================================================
        /// Initialize asset info.
        InitAssetInfo { args: InitAssetInfoArgs } = 20,

        /// Update asset info.
        UpdateAssetInfo { args: UpdateAssetInfoArgs } = 21,

        /// Initialize asset info with Pyth Lazer feed configuration.
        InitAssetInfoV1 { args: InitAssetInfoArgsV1 } = 22,

        /// Update asset info with Pyth Lazer feed configuration.
        UpdateAssetInfoV1 { args: UpdateAssetInfoArgsV1 } = 23,

        // Reserved: 24-29

        // =========================================================================
        // Borrow/Lend Operations (30-39)
        // =========================================================================
        /// Initialize a borrow/lend pool.
        InitBorrowLendPool { args: InitBorrowLendPoolArgs } = 30,

        /// Update borrow/lend pool configuration.
        UpdateBorrowLendPool { args: UpdateBorrowLendPoolArgs } = 31,

        /// Halt a borrow/lend pool.
        HaltBorrowLendPool { asset_id: AssetId } = 32,

        /// Unhalt a borrow/lend pool.
        UnhaltBorrowLendPool { asset_id: AssetId } = 33,
        // Reserved: 34-39

        // =========================================================================
        // Configuration Operations (40-49)
        // =========================================================================
        /// Update global configuration.
        UpdateGlobalConfig {
            args: UpdateGlobalConfigArgs<Address>,
        } = 40,

        /// Update perp liquidation configuration.
        UpdatePerpLiquidationConfig {
            args: UpdatePerpLiquidationConfigArgs,
        } = 41,

        /// Update global vault configuration.
        UpdateGlobalVaultConfig { args: UpdateGlobalVaultConfigArgs } = 42,

        /// Update admin addresses.
        UpdateAdmin {
            admin_type: AdminType,
            new_admin: Address,
        } = 43,

        /// Update global configuration with Pyth Lazer trusted signers.
        UpdateGlobalConfigV1 {
            args: UpdateGlobalConfigArgsV1<Address>,
        } = 44,

        // Reserved: 45-49

        // =========================================================================
        // Treasury Operations (50-59)
        // =========================================================================
        /// Withdraw from protocol treasury.
        WithdrawFromTreasury {
            asset_id: AssetId,
            amount: PositiveDecimal,
            to: Address,
        } = 50,
        // Reserved: 51-59

        // =========================================================================
        // Force/Emergency Operations (60-69)
        // =========================================================================
        /// Force cancel user orders.
        CancelOrders {
            cancels: Vec<(MarketId, OrderId, Address)>,
        } = 60,

        /// Force cancel user trigger orders.
        CancelTriggerOrders {
            cancels: Vec<(MarketId, TriggerOrderId, Address)>,
        } = 61,

        /// Force settle perp positions.
        ForceSettlePerpPosition {
            market_id: MarketId,
            users: Vec<Address>,
        } = 62,

        /// Auto-deleverage positions.
        AutoDeleverage {
            counterparty_a: Address,
            counterparty_a_sub_account_index: Option<u8>,
            counterparty_b: Address,
            counterparty_b_sub_account_index: Option<u8>,
            market_id: MarketId,
            size: Option<PositiveDecimal>,
            settlement_price: PositiveDecimal,
        } = 63,

        /// Admin deposit to any user account (creates account if needed).
        /// Funds come from the admin's wallet.
        Deposit {
            user_address: Address,
            asset_id: AssetId,
            amount: PositiveDecimal,
        } = 64,

        ForceRemoveDelegate { delegator: Address, delegate: Address } = 65,

        /// Initialize a new protocol-owned vault.
        ///
        /// Behaves identically to a user-created vault but is flagged with
        /// `AccountVariant::ProtocolVault` so future protocol-specific behavior
        /// (e.g. governance-only withdrawals, fee exemptions) can branch on it.
        InitProtocolVault { args: CreateVaultArgs<Address> } = 66,
        // Reserved: 67-69
    }
}
