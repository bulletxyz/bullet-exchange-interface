//! Vault operations.

mod args;
pub use args::*;

/// Vault management operations requiring vault leadership.
///
/// These operations can only be called by the vault leader (the address that created the vault).
#[derive(
    Clone,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
    borsh::BorshDeserialize,
    borsh::BorshSerialize,
    schemars::JsonSchema,
    serde::Deserialize,
    serde::Serialize,
    sov_universal_wallet::UniversalWallet,
    strum::AsRefStr,
)]
#[serde(rename_all = "snake_case")]
#[borsh(use_discriminant = true)]
#[repr(u8)]
pub enum VaultAction<Address> {
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
    } = 4,

    /// Revoke vault trading delegation.
    RevokeVaultDelegation {
        vault_address: Address,
        delegate: Address,
    } = 5,
    // Reserved: 6-255
}
