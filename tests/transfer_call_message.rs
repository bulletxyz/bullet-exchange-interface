//! Coverage for the `UserAction::Transfer` call message: discriminant
//! stability, borsh round-trip, and snake_case JSON structure/round-trip.

use borsh::{BorshDeserialize, to_vec};
use bullet_exchange_interface::address::Address;
use bullet_exchange_interface::decimals::PositiveDecimal;
use bullet_exchange_interface::message::{CallMessage, TransferEndpoint, UserAction};
use bullet_exchange_interface::string::CustomString;
use bullet_exchange_interface::transaction::RuntimeCall;
use bullet_exchange_interface::types::{AssetId, BalanceBucket, MarketId};

fn own_account_transfer() -> RuntimeCall {
    RuntimeCall::Exchange(CallMessage::User(UserAction::Transfer {
        from: TransferEndpoint {
            sub_account_index: None,
            balance: BalanceBucket::Cross,
        },
        to: TransferEndpoint {
            sub_account_index: Some(2),
            balance: BalanceBucket::Iso(MarketId(7)),
        },
        to_address: None,
        asset_id: AssetId(3),
        amount: PositiveDecimal::from(100u32),
        memo: CustomString::from("ref-42"),
    }))
}

fn cross_account_transfer() -> RuntimeCall {
    RuntimeCall::Exchange(CallMessage::User(UserAction::Transfer {
        from: TransferEndpoint {
            sub_account_index: None,
            balance: BalanceBucket::Bank,
        },
        to: TransferEndpoint {
            sub_account_index: None,
            balance: BalanceBucket::Spot,
        },
        to_address: Some(Address([0x01; 32])),
        asset_id: AssetId(9),
        amount: PositiveDecimal::from(5u32),
        memo: CustomString::from(""),
    }))
}

#[test]
fn transfer_borsh_discriminant_prefix_is_stable() {
    // RuntimeCall::Exchange = 7, CallMessage::User = 0, UserAction::Transfer = 18 (0x12).
    let bytes = to_vec(&own_account_transfer()).expect("serialize transfer");
    assert_eq!(&bytes[0..3], &[0x07, 0x00, 0x12]);
}

#[test]
fn transfer_borsh_round_trips() {
    for call in [own_account_transfer(), cross_account_transfer()] {
        let bytes = to_vec(&call).expect("serialize transfer");
        assert_eq!(
            RuntimeCall::try_from_slice(&bytes).expect("deserialize transfer"),
            call
        );
    }
}

#[test]
fn transfer_json_round_trips() {
    for call in [own_account_transfer(), cross_account_transfer()] {
        let value = serde_json::to_value(&call).expect("serialize transfer");
        assert_eq!(
            serde_json::from_value::<RuntimeCall>(value).expect("deserialize transfer"),
            call
        );
    }
}

#[test]
fn transfer_json_uses_snake_case_structure() {
    let value = serde_json::to_value(own_account_transfer()).expect("serialize transfer");
    let transfer = &value["exchange"]["user"]["transfer"];

    // Endpoints: unit balance variant renders as a snake_case string; the
    // data-carrying `Iso` variant renders as `{ "iso": <market_id> }`.
    assert_eq!(transfer["from"]["balance"], serde_json::json!("cross"));
    assert!(transfer["from"]["sub_account_index"].is_null());
    assert_eq!(transfer["to"]["balance"]["iso"], serde_json::json!(7));
    assert_eq!(transfer["to"]["sub_account_index"], serde_json::json!(2));

    // Own-account destination => no address.
    assert!(transfer["to_address"].is_null());
    assert_eq!(transfer["asset_id"], serde_json::json!(3));
    assert_eq!(transfer["memo"], serde_json::json!("ref-42"));
}

#[test]
fn transfer_json_encodes_destination_address() {
    let value = serde_json::to_value(cross_account_transfer()).expect("serialize transfer");
    let to_address = &value["exchange"]["user"]["transfer"]["to_address"];
    // Address serializes as a base58 string in human-readable form.
    assert_eq!(
        to_address.as_str().expect("to_address is a string"),
        Address([0x01; 32]).to_string()
    );
}
