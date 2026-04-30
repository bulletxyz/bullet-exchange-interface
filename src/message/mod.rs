//! Exchange call message types - nested enum with role-based categorization.
//!
//! # Adding New Operations
//!
//! 1. Determine the role/category: Who is authorized to call this operation?
//!    - User? Vault leader? Specific admin? Anyone?
//! 2. Add the variant to the appropriate *Action enum
//! 3. Choose a NEVER used explicit discriminant: `NewOperation { ... } = N,`
//! 4. Add handler function and update dispatch
//! 5. NEVER change existing discriminants, reorder variants/fields, change types

use crate::define_enum;

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

define_enum! {
    /// Top-level call message enum organized by authorization role.
    enum CallMessage<Address> {
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
}

impl<Address> CallMessage<Address> {
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

impl<Address> From<UserAction<Address>> for CallMessage<Address> {
    fn from(v: UserAction<Address>) -> Self {
        Self::User(v)
    }
}
impl<Address> From<VaultAction<Address>> for CallMessage<Address> {
    fn from(v: VaultAction<Address>) -> Self {
        Self::Vault(v)
    }
}
impl<Address> From<KeeperAction<Address>> for CallMessage<Address> {
    fn from(v: KeeperAction<Address>) -> Self {
        Self::Keeper(v)
    }
}
impl<Address> From<PublicAction<Address>> for CallMessage<Address> {
    fn from(v: PublicAction<Address>) -> Self {
        Self::Public(v)
    }
}
impl<Address> From<AdminAction<Address>> for CallMessage<Address> {
    fn from(v: AdminAction<Address>) -> Self {
        Self::Admin(v)
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
