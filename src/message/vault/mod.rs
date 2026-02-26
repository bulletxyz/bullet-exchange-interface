//! Vault operations.
use crate::define_enum;
use crate::string::CustomString;
mod args;
pub use args::*;

define_enum! {
    /// Vault management operations requiring vault leadership.
    ///
    /// These operations can only be called by the vault leader (the address that created the vault).

    enum VaultAction<Address> {
        /// Update vault configuration (leader only).
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
            name: Option<CustomString>,
        } = 4,

        /// Revoke vault trading delegation.
        RevokeVaultDelegation {
            vault_address: Address,
            delegate: Address,
        } = 5,
        // Reserved: 6-255
    }
}
