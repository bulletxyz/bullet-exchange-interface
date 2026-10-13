//! Coverage for the `UserAction::CancelQueuedWithdrawalV1` call message: discriminant
//! stability, borsh round-trip, and snake_case JSON structure/round-trip.

use borsh::{BorshDeserialize, to_vec};
use bullet_exchange_interface::address::Address;
use bullet_exchange_interface::message::{CallMessage, UserAction};
use bullet_exchange_interface::time::UnixTimestampMicros;
use bullet_exchange_interface::transaction::RuntimeCall;

fn cancel(withdrawal_timestamp: Option<UnixTimestampMicros>) -> RuntimeCall {
    RuntimeCall::Exchange(CallMessage::User(UserAction::CancelQueuedWithdrawalV1 {
        vault_address: Address([0x01; 32]),
        withdrawal_timestamp,
    }))
}

fn cancel_one() -> RuntimeCall {
    cancel(Some(UnixTimestampMicros::from_micros(0x0102)))
}

fn cancel_all() -> RuntimeCall {
    cancel(None)
}

#[test]
fn cancel_queued_withdrawal_v1_borsh_layout_is_stable() {
    // RuntimeCall::Exchange = 7, CallMessage::User = 0, UserAction::CancelQueuedWithdrawalV1 = 55
    // (0x37). The 32-byte vault address follows, then the withdrawal timestamp as an Option<i64 LE>.
    let bytes = to_vec(&cancel_one()).expect("serialize cancel one");
    assert_eq!(&bytes[0..3], &[0x07, 0x00, 0x37]);
    assert_eq!(&bytes[3..35], &[0x01; 32]);
    assert_eq!(&bytes[35..], &[0x01, 0x02, 0x01, 0, 0, 0, 0, 0, 0]);

    let bytes = to_vec(&cancel_all()).expect("serialize cancel all");
    assert_eq!(&bytes[0..3], &[0x07, 0x00, 0x37]);
    assert_eq!(&bytes[35..], &[0x00]);
}

#[test]
fn cancel_queued_withdrawal_v1_borsh_round_trips() {
    for call in [cancel_one(), cancel_all()] {
        let bytes = to_vec(&call).expect("serialize cancel");
        assert_eq!(
            RuntimeCall::try_from_slice(&bytes).expect("deserialize cancel"),
            call
        );
    }
}

#[test]
fn cancel_queued_withdrawal_v1_json_round_trips() {
    for call in [cancel_one(), cancel_all()] {
        let value = serde_json::to_value(&call).expect("serialize cancel");
        assert_eq!(
            serde_json::from_value::<RuntimeCall>(value).expect("deserialize cancel"),
            call
        );
    }
}

#[test]
fn cancel_queued_withdrawal_v1_json_uses_snake_case_structure() {
    let value = serde_json::to_value(cancel_one()).expect("serialize");
    let cancel = &value["exchange"]["user"]["cancel_queued_withdrawal_v1"];

    assert_eq!(
        cancel["vault_address"]
            .as_str()
            .expect("vault_address is a string"),
        Address([0x01; 32]).to_string()
    );
    assert_eq!(cancel["withdrawal_timestamp"], serde_json::json!(0x0102));

    let value = serde_json::to_value(cancel_all()).expect("serialize");
    let cancel = &value["exchange"]["user"]["cancel_queued_withdrawal_v1"];
    // No timestamp => cancel all.
    assert!(cancel["withdrawal_timestamp"].is_null());
}
