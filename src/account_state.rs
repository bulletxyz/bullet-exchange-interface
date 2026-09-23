//! Authoritative account images, one per account a transaction writes.

/// Version 1 of a position-bearing account's persisted state.
///
/// Emitted once per account a transaction writes, after the transaction's
/// other events, and reflects the account's state at the end of that
/// transaction. Ordering and source identity come from the transaction
/// envelope, not from this payload.
#[derive(
    borsh::BorshDeserialize,
    borsh::BorshSerialize,
    serde::Deserialize,
    serde::Serialize,
    schemars::JsonSchema,
    Debug,
    Clone,
    PartialEq,
    Eq,
)]
pub struct AccountStateV1<Address> {
    /// Resolved account address, including a subaccount or vault's own address.
    pub account_address: Address,
    pub operation: AccountStateOperationV1,
}

/// A full replacement or a deletion marker. Append variants only.
#[derive(
    borsh::BorshDeserialize,
    borsh::BorshSerialize,
    serde::Deserialize,
    serde::Serialize,
    schemars::JsonSchema,
    Debug,
    Clone,
    PartialEq,
    Eq,
)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum AccountStateOperationV1 {
    Replace {
        /// Borsh bytes of the exchange module's `UserContainer` envelope
        /// (version discriminant included) reduced to its risk projection:
        /// balances, per-market positions, leverage settings and resting-order
        /// / TWAP notional aggregates are kept; order, trigger-order and TWAP
        /// id sets, spot ledgers, client order ids and rewards are emptied.
        /// `UserContainer::into_account` of these bytes yields the same margin
        /// figures as the stored container. JSON uses unprefixed lowercase hex.
        /// Incompatible container changes require a new event version.
        #[serde(with = "hex::serde")]
        #[schemars(with = "String")]
        user_container: Vec<u8>,
    },
    /// The address no longer holds a position-bearing account.
    Delete,
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::event::Event;
    use serde_json::json;

    #[test]
    fn account_state_v1_json_and_event_key_are_stable() {
        let snapshot = AccountStateV1 {
            account_address: "account".to_owned(),
            operation: AccountStateOperationV1::Replace {
                user_container: vec![4, 0, 171, 255],
            },
        };
        let event = Event::AccountStateV1(snapshot.clone());
        let json = json!({
            "account_state_v1": {
                "account_address": "account",
                "operation": {"type": "replace", "user_container": "0400abff"}
            }
        });
        assert_eq!(event.event_key(), "Exchange/AccountStateV1");
        assert_eq!(serde_json::to_value(&event).unwrap(), json);
        assert_eq!(
            serde_json::from_value::<Event<String>>(json).unwrap(),
            event
        );
        assert_eq!(
            borsh::from_slice::<AccountStateV1<String>>(&borsh::to_vec(&snapshot).unwrap())
                .unwrap(),
            snapshot
        );
    }

    #[test]
    fn account_state_v1_delete_json_is_stable() {
        let snapshot = AccountStateV1 {
            account_address: "account".to_owned(),
            operation: AccountStateOperationV1::Delete,
        };
        assert_eq!(
            serde_json::to_value(&snapshot).unwrap(),
            json!({"account_address": "account", "operation": {"type": "delete"}})
        );
    }

    #[test]
    fn account_state_v1_rejects_invalid_hex() {
        for invalid in ["xyz", "0", "0x04"] {
            assert!(
                serde_json::from_value::<AccountStateOperationV1>(json!({
                    "type": "replace", "user_container": invalid
                }))
                .is_err()
            );
        }
    }
}
