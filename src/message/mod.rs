//! Exchange call message types - nested enum with role-based categorization.
//!
//! # Migration from v1 (Flat Enum)
//!
//! # Adding New Operations
//!
//! 1. Determine the role/category: Who is authorized to call this operation?
//!    - User? Vault leader? Specific admin? Anyone?
//! 2. Add the variant to the appropriate *Action enum
//! 3. Choose a NEVER used explicit discriminant: `NewOperation { ... } = N,`
//! 4. Add handler function and update dispatch
//! 5. NEVER change existing discriminants
//!
//! Each category can have up to 256 operations, providing ample room for growth.
//!
//! # Borsh ABI Rules
//!
//! - `#[borsh(use_discriminant = true)]` + `#[repr(u8)]` = explicit discriminant values
//! - Add new variants with explicit discriminant (`= N`), position doesn't matter
//! - Add new fields at END as `Option<T>` (old payloads won't have trailing bytes)
//! - NEVER: reorder variants/fields, change/reuse discriminants, change types

use sov_rollup_interface::BasicAddress;

mod admin;
mod keeper;
mod public;
mod user;
mod vault;

pub use self::admin::*;
pub use self::keeper::*;
pub use self::public::*;
pub use self::user::*;
pub use self::vault::*;

/// Top-level call message enum organized by authorization role.
///
/// Serializes as: `[category:u8][action:u8][fields...]`
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
)]
#[serde(rename_all = "snake_case", bound = "Address: BasicAddress")]
#[schemars(bound = "Address: BasicAddress")]
#[borsh(use_discriminant = true)]
#[repr(u8)]
pub enum CallMessage<Address: BasicAddress> {
    /// User-facing operations requiring account ownership.
    ///
    /// Auth: `context.sender()` or resolved delegate
    User(UserAction<Address>) = 0,

    /// Vault management operations requiring vault leadership.
    ///
    /// Auth: `vault.leader() == context.sender()`
    Vault(VaultAction<Address>) = 1,

    /// Keeper operations requiring specific admin privileges.
    ///
    /// Auth: Specific admin type (Pricing, Funding, Credits, FeeTier, Referrals)
    Keeper(KeeperAction<Address>) = 2,

    /// Permissionless operations anyone can call.
    ///
    /// Auth: None (permissionless)
    Public(PublicAction<Address>) = 3,

    /// Protocol admin operations.
    ///
    /// Auth: Protocol admin
    Admin(AdminAction<Address>) = 4,
}

impl<Address: BasicAddress> CallMessage<Address> {
    pub fn msg_type(&self) -> String {
        match self {
            Self::User(x) => format!("User/{}", x.as_ref()),
            Self::Vault(x) => format!("Vault/{}", x.as_ref()),
            Self::Keeper(x) => format!("Keeper/{}", x.as_ref()),
            Self::Public(x) => format!("Public/{}", x.as_ref()),
            Self::Admin(x) => format!("Admin/{}", x.as_ref()),
        }
    }
}

#[test]
fn test_msg_type() {
    assert_eq!(
        "User/CreateSubAccount",
        CallMessage::<crate::address::Address>::User(UserAction::CreateSubAccount { index: 0 })
            .msg_type()
    );
}
