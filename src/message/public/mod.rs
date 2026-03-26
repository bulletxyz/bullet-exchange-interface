//! Permissionless operations.

use crate::define_enum;
use crate::types::MarketId;

define_enum! {
    /// Permissionless operations anyone can call.
    ///
    /// These operations have no authorization checks - anyone can trigger them.
    /// Typically used for MEV opportunities (liquidations) or protocol maintenance
    /// (interest accrual, funding application).
    enum PublicAction<Address> {
        /// Try to force close cross margin perp positions for a user account under maintenance margin but above backstop liquidation margin (permissionless).
        LiquidatePerpPositions { address: Address } = 0,

        /// Force cancel orders for a user under initial margin on cross margin positions (permissionless).
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

        /// Force cancel orders for a user under initial margin for an iso margin position (permissionless)
        ForceCancelIsoOrders { user_address: Address, market_id: MarketId } = 7,

        /// Try to force close an iso margin position for a user under maintenance margin but above backstop liquidation margin (permissionless)
        LiquidateIsoPerpPosition { user_address: Address, market_id: MarketId } = 8,

        // Reserved: 9-255
    }
}
