//! Compile-time bounds for `SafeVec` fields in CallMessage types.
//!
//! These constants define the maximum number of elements allowed in batch
//! operations. They are enforced during Borsh deserialization, rejecting
//! oversized transactions before any handler logic runs. This significantly
//! reduces the DoS attack surface.
//!
//! # Design Rationale
//!
//! - **User/Public actions** use conservative bounds (30) since they are
//!   accessible to unauthenticated users and represent the primary DoS vector.
//! - **Keeper actions** use moderate bounds (128) since they require specific
//!   admin privileges but still process batches of market updates.
//! - **Admin actions** use larger bounds (256) since they require protocol
//!   admin authorization and are not externally exploitable.

/// Maximum batch size for user-facing and permissionless operations.
///
/// Covers: PlaceOrders, AmendOrders, CancelOrders, CreateTriggerOrders,
/// CancelTriggerOrders, BackstopLiquidatePerpPositions, ApplyFunding,
/// ActivateTwapOrders, CreateVaultArgs deposit_asset_ids.
pub const MAX_USER_BATCH: usize = 30;

/// Maximum batch size for keeper (privileged bot) operations.
///
/// Covers: UpdateOraclePrices, UpdateMarkPrices, UpdatePremiumIndexes,
/// UpdateFunding.
pub const MAX_KEEPER_BATCH: usize = 128;

/// Maximum batch size for protocol admin operations.
///
/// Covers: CleanupUserMarketState, admin CancelOrders,
/// admin CancelTriggerOrders, ForceSettlePerpPosition.
pub const MAX_ADMIN_BATCH: usize = 256;

/// Maximum number of fee tiers in market configuration.
pub const MAX_FEE_TIERS: usize = 32;

/// Maximum number of deposit limit entries in global config.
pub const MAX_DEPOSIT_LIMITS: usize = 128;

/// Maximum number of whitelisted deposit addresses in global config.
pub const MAX_WHITELISTED_DEPOSITORS: usize = 256;
