use crate::define_enum;
use crate::transaction::bank::TokenId;
use crate::transaction::{Amount, WarpBytes20, WarpBytes32};

// The nested `Admin`/`TokenKind`/`Ism` types use serde's DEFAULT (PascalCase)
// variant naming and positional borsh discriminants — matching the runtime's
// `hyperlane::warp` module. They deliberately do NOT go through `define_enum!`,
// which forces `#[serde(rename_all = "snake_case")]` (the runtime only applies
// snake_case to the top-level `CallMessage`, not to these).

/// The authority that can modify a warp route. Mirrors the runtime `Admin<S>`.
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
    serde::Deserialize,
    serde::Serialize,
    schemars::JsonSchema,
)]
#[cfg_attr(feature = "schema", derive(sov_universal_wallet::UniversalWallet))]
pub enum Admin<Address> {
    /// No admin — the route is immutable.
    None,
    /// Allow the specified address to modify the route.
    InsecureOwner(Address),
}

/// The source of the token backing a warp route. Mirrors the runtime `TokenKind`.
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
    serde::Deserialize,
    serde::Serialize,
    schemars::JsonSchema,
)]
#[cfg_attr(feature = "schema", derive(sov_universal_wallet::UniversalWallet))]
pub enum TokenKind {
    /// Natively issued on a remote chain; represented locally as a synthetic token.
    Synthetic {
        /// The ID of the remote token.
        remote_token_id: WarpBytes32,
        /// The number of decimal places of the remote token.
        remote_decimals: u8,
        /// The number of decimal places for the local (synthetic) token.
        local_decimals: Option<u8>,
    },
    /// Natively issued on the local chain.
    Collateral {
        /// The ID of the token on the local chain.
        token: TokenId,
    },
    /// The native token of the local chain.
    Native,
}

/// The interchain security module for a warp route. Mirrors the runtime `Ism`.
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
    serde::Deserialize,
    serde::Serialize,
    schemars::JsonSchema,
)]
#[cfg_attr(feature = "schema", derive(sov_universal_wallet::UniversalWallet))]
pub enum Ism {
    /// Performs no validation — accepts any message.
    AlwaysTrust,
    /// Accepts all messages from a trusted relayer.
    TrustedRelayer {
        /// The address of the trusted relayer.
        relayer: WarpBytes32,
    },
    /// Accepts messages signed by `threshold` or more of the given `validators`.
    MessageIdMultisig {
        /// The validator addresses (20-byte Ethereum addresses).
        validators: Vec<WarpBytes20>,
        /// The number of signatures required to accept a message.
        threshold: u32,
    },
}

define_enum! {
    /// CallMessage for the Warp module.
    #[non_exhaustive]
    #[strum_discriminants(non_exhaustive)]
    enum CallMessage<Address> {
        /// Register a route with the given token source and ISM.
        Register {
            /// The authority that can modify the route, if any.
            admin: Admin<Address>,
            /// The token source for the route.
            token_source: TokenKind,
            /// The ISM for this route.
            ism: Ism,
            /// Remote routers to enroll on route registration: `(domain, router)`.
            remote_routers: Vec<(u32, WarpBytes32)>,
            /// Inbound rate-limit bucket capacity.
            #[serde(with = "crate::transaction::serde_amount_decimal_string")]
            #[schemars(with = "String")]
            inbound_transferrable_tokens_limit: Amount,
            /// Inbound rate-limit refill per visible slot.
            #[serde(with = "crate::transaction::serde_amount_decimal_string")]
            #[schemars(with = "String")]
            inbound_limit_replenishment_per_slot: Amount,
            /// Outbound rate-limit bucket capacity.
            #[serde(with = "crate::transaction::serde_amount_decimal_string")]
            #[schemars(with = "String")]
            outbound_transferrable_tokens_limit: Amount,
            /// Outbound rate-limit refill per visible slot.
            #[serde(with = "crate::transaction::serde_amount_decimal_string")]
            #[schemars(with = "String")]
            outbound_limit_replenishment_per_slot: Amount,
        } = 0,
        /// Update an existing route's admin, ISM, or rate limits.
        Update {
            /// The ID of the warp route to update.
            warp_route: WarpBytes32,
            /// New authority that can modify the route.
            admin: Option<Admin<Address>>,
            /// New ISM for this route.
            ism: Option<Ism>,
            /// New inbound rate-limit bucket capacity.
            #[serde(default, with = "crate::transaction::serde_amount_decimal_string_opt")]
            #[schemars(with = "Option<String>")]
            inbound_transferrable_tokens_limit: Option<Amount>,
            /// New inbound rate-limit refill per visible slot.
            #[serde(default, with = "crate::transaction::serde_amount_decimal_string_opt")]
            #[schemars(with = "Option<String>")]
            inbound_limit_replenishment_per_slot: Option<Amount>,
            /// New outbound rate-limit bucket capacity.
            #[serde(default, with = "crate::transaction::serde_amount_decimal_string_opt")]
            #[schemars(with = "Option<String>")]
            outbound_transferrable_tokens_limit: Option<Amount>,
            /// New outbound rate-limit refill per visible slot.
            #[serde(default, with = "crate::transaction::serde_amount_decimal_string_opt")]
            #[schemars(with = "Option<String>")]
            outbound_limit_replenishment_per_slot: Option<Amount>,
        } = 1,
        /// Transfer a token from the local chain to a remote chain.
        TransferRemote {
            warp_route: WarpBytes32,
            destination_domain: u32,
            recipient: WarpBytes32,
            #[serde(with = "crate::transaction::serde_amount_decimal_string")]
            #[schemars(with = "String")]
            amount: Amount,
            relayer: Option<Address>,
            #[serde(with = "crate::transaction::serde_amount_decimal_string")]
            #[schemars(with = "String")]
            gas_payment_limit: Amount,
        } = 4,
    }
}
