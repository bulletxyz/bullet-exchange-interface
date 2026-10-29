//! Exchange call message types - nested enum with role-based categorization.
//!
//! Adding an operation: pick the `*Action` enum by who may call it, give the variant a never-used
//! explicit discriminant, and add its handler and dispatch. NEVER change existing discriminants,
//! reorder variants/fields or change types.

use crate::define_enum;
use crate::types::OrderType;

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
    #[non_exhaustive]
    #[strum_discriminants(non_exhaustive)]
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

    /// How the sequencer queues this call if it places or cancels orders, `None` otherwise.
    pub fn order_flow(&self) -> Option<OrderFlow> {
        fn placing(mut order_types: impl Iterator<Item = OrderType>) -> OrderFlow {
            if order_types.all(|t| {
                matches!(
                    t,
                    OrderType::PostOnly | OrderType::PostOnlySlide | OrderType::PostOnlyFront
                )
            }) {
                OrderFlow::Maker
            } else {
                OrderFlow::Taker
            }
        }

        let Self::User(action) = self else {
            return None;
        };
        match action {
            UserAction::CancelOrders { .. }
            | UserAction::CancelAllOrders { .. }
            | UserAction::CancelMarketOrders { .. } => Some(OrderFlow::Cancel),
            UserAction::CancelAndPlaceOrders { places, .. } if places.is_empty() => {
                Some(OrderFlow::Cancel)
            }
            UserAction::CancelAndPlaceOrders { places, .. } => {
                Some(placing(places.iter().map(|o| o.order_type)))
            }
            UserAction::PlaceOrders { orders, .. } => {
                Some(placing(orders.iter().map(|o| o.order_type)))
            }
            UserAction::AmendOrders { orders, .. } => {
                Some(placing(orders.iter().map(|a| a.place.order_type)))
            }
            // Degen open/close always execute aggressively at top-of-book.
            UserAction::DegenTrade { .. } => Some(OrderFlow::Taker),
            _ => None,
        }
    }
}

/// Sequencer queueing class of an order call: takers wait out a speed bump, makers and cancels
/// queue immediately.
#[derive(Debug, Clone, Copy, PartialEq, Eq, strum::IntoStaticStr)]
#[strum(serialize_all = "snake_case")]
pub enum OrderFlow {
    /// Places only post-only orders.
    Maker,
    /// Places at least one order that may cross the book.
    Taker,
    /// Only cancels orders.
    Cancel,
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
#[allow(deprecated)] // pins the wire `msg_type` of the deprecated variant
fn test_msg_type() {
    assert_eq!(
        "User/CreateSubAccount",
        CallMessage::<crate::address::Address>::User(UserAction::CreateSubAccount { index: 0 })
            .msg_type()
    );
}
