//! Permissionless operations.

use crate::bounds::MAX_USER_BATCH;
use crate::types::MarketId;
use crate::{SafeVec, define_enum};

define_enum! {
    /// Permissionless operations anyone can call.
    ///
    /// These operations have no authorization checks - anyone can trigger them.
    /// Typically used for MEV opportunities (liquidations) or protocol maintenance
    /// (interest accrual, funding application).
    enum PublicAction<Address> {
        /// Liquidate perp positions for an underwater account (permissionless).
        LiquidatePerpPositions { address: Address } = 0,

        /// Force cancel orders for a liquidatable user (permissionless).
        ForceCancelOrders { user_address: Address } = 1,

        /// Execute active trigger orders (permissionless).
        ExecuteTriggerOrders { market_id: MarketId } = 2,

        /// Apply funding to user accounts (permissionless).
        ApplyFunding { addresses: SafeVec<Address, MAX_USER_BATCH> } = 3,

        /// Accrue borrow/lend interest (permissionless).
        AccrueBorrowLendInterest {} = 4,

        /// Execute TWAP orders  (permissionless)
        ExecuteTwapOrders { market_id: MarketId } = 5,

        /// Activate TWAP orders  (permissionless)
        ActivateTwapOrders { market_ids: SafeVec<MarketId, MAX_USER_BATCH> } = 6,
        // Reserved: 7-255
    }
}
