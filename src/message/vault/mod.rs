//! Vault operations.
use crate::define_enum;
use crate::string::CustomString;
use crate::time::UnixTimestampMicros;
mod args;
pub use args::*;

define_enum! {
    /// Vault management operations requiring vault leadership.
    ///
    /// These operations can only be called by the vault leader (the address that created the vault).
    #[non_exhaustive]
    #[strum_discriminants(non_exhaustive)]
    enum VaultAction<Address> {
        /// Deprecated - use UpdateVaultConfigV1 instead.
        UpdateVaultConfig {
            vault_address: Address,
            args: UpdateVaultConfigArgs,
        } = 0,

        /// Process pending vault withdrawals.
        ProcessWithdrawalQueue { vault_address: Address } = 1,

        /// Whitelist a depositor for the vault.
        WhitelistDepositor {
            vault_address: Address,
            user_address: Address,
        } = 2,

        /// Remove a depositor from the vault whitelist.
        UnwhitelistDepositor {
            vault_address: Address,
            user_address: Address,
        } = 3,

        /// Delegate vault trading to another address.
        DelegateVaultUser {
            vault_address: Address,
            delegate: Address,
            name: CustomString,
        } = 4,

        /// Revoke vault trading delegation.
        RevokeVaultDelegation {
            vault_address: Address,
            delegate: Address,
        } = 5,

        /// Delegate vault trading with optional expiry and flags
        DelegateVaultUserV1 {
            vault_address: Address,
            delegate: Address,
            name: CustomString,
            expires_at: Option<UnixTimestampMicros>,
            flags: u32,
        } = 6,

        /// Update vault configuration including the withdrawal fee (leader only).
        UpdateVaultConfigV1 {
            vault_address: Address,
            args: UpdateVaultConfigArgsV1,
        } = 7,

        /// Set, replace or clear the vault's per-user deposit policy (leader only).
        ///
        /// `policy: None` clears it, restoring unrestricted deposits. A supplied policy replaces
        /// the previous one outright, strategic caps included, so dropping an address from
        /// `strategic_caps` is how its override is removed. Cumulative per-user deposit totals
        /// live on the depositor's own record and survive every one of these, so clearing and
        /// re-setting a policy never hands anyone a fresh allowance.
        ///
        /// The vault-wide `deposit_limit` is unaffected and stays managed by `UpdateVaultConfigV1`.
        SetDepositPolicy {
            vault_address: Address,
            policy: Option<VaultDepositPolicyArgs<Address>>,
        } = 8,
        // Reserved: 9-255
    }
}
