//! Argument types for vault operations.

use crate::decimals::PositiveDecimal;
use crate::define_struct;

define_struct! {
    struct UpdateVaultConfigArgs {
        deposit_limit: Option<PositiveDecimal>,
        withdraw_lockup_period_hours: Option<u8>,
        profit_share_percentage: Option<u8>,
    }
}

define_struct! {
    /// [`UpdateVaultConfigArgs`] plus the withdrawal fee, which may only be lowered.
    struct UpdateVaultConfigArgsV1 {
        deposit_limit: Option<PositiveDecimal>,
        withdraw_lockup_period_hours: Option<u8>,
        profit_share_percentage: Option<u8>,
        /// New withdrawal fee in basis points. Must not exceed the vault's current fee.
        withdrawal_fee_bps: Option<u8>,
    }
}

define_struct! {
    /// A per-address override of a vault's default deposit cap.
    struct StrategicDepositorCap<Address> {
        user_address: Address,
        /// Cumulative deposit cap (notional) for this address, replacing the vault default.
        cap: PositiveDecimal,
    }
}

define_struct! {
    /// A vault's per-user deposit policy, supplied whole.
    ///
    /// `SetDepositPolicy` replaces any previous policy with this one, so `strategic_caps` is the
    /// complete list of overrides after the call: omitting an address that had one removes it,
    /// and an empty list leaves every depositor on the default cap.
    ///
    /// Caps are cumulative deposited notional, not a current balance. Withdrawing does not free
    /// allowance, and these totals live on the depositor's own record, so they outlive any change
    /// to the policy.
    struct VaultDepositPolicyArgs<Address> {
        /// Cumulative deposit cap (notional) applied to any depositor without an override.
        default_user_deposit_cap: PositiveDecimal,
        /// Per-address overrides, replacing the whole previous set.
        strategic_caps: Vec<StrategicDepositorCap<Address>>,
    }
}
