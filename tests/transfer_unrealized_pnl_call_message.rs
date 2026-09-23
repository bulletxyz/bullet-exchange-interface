//! Coverage for the `UserAction::TransferUnrealizedPnl` call message: discriminant
//! stability, borsh round-trip, and snake_case JSON structure/round-trip.

use borsh::{BorshDeserialize, to_vec};
use bullet_exchange_interface::address::Address;
use bullet_exchange_interface::decimals::PositiveDecimal;
use bullet_exchange_interface::message::{CallMessage, PerpPositionEndpoint, UserAction};
use bullet_exchange_interface::transaction::RuntimeCall;
use bullet_exchange_interface::types::MarketId;

fn own_account_transfer() -> RuntimeCall {
    RuntimeCall::Exchange(CallMessage::User(UserAction::TransferUnrealizedPnl {
        from: PerpPositionEndpoint {
            sub_account_index: None,
            market_id: MarketId(7),
        },
        to: PerpPositionEndpoint {
            sub_account_index: Some(2),
            market_id: MarketId(7),
        },
        to_address: None,
        amount: PositiveDecimal::from(25u32),
    }))
}

fn cross_account_transfer() -> RuntimeCall {
    RuntimeCall::Exchange(CallMessage::User(UserAction::TransferUnrealizedPnl {
        from: PerpPositionEndpoint {
            sub_account_index: Some(1),
            market_id: MarketId(7),
        },
        to: PerpPositionEndpoint {
            sub_account_index: None,
            market_id: MarketId(9),
        },
        to_address: Some(Address([0x01; 32])),
        amount: PositiveDecimal::try_from(rust_decimal::Decimal::new(25, 1)).unwrap(),
    }))
}

#[test]
fn transfer_unrealized_pnl_borsh_discriminant_prefix_is_stable() {
    // RuntimeCall::Exchange = 7, CallMessage::User = 0, UserAction::TransferUnrealizedPnl = 70
    // (0x46). The `from` endpoint follows: an Option<u8> tag, then the market id as u16 LE.
    let bytes = to_vec(&own_account_transfer()).expect("serialize own-account transfer");
    assert_eq!(&bytes[0..3], &[0x07, 0x00, 0x46]);
    assert_eq!(bytes[3], 0x00); // from.sub_account_index == None
    assert_eq!(&bytes[4..6], &[0x07, 0x00]); // from.market_id == MarketId(7)
    assert_eq!(&bytes[6..8], &[0x01, 0x02]); // to.sub_account_index == Some(2)

    let bytes = to_vec(&cross_account_transfer()).expect("serialize cross-account transfer");
    assert_eq!(&bytes[0..3], &[0x07, 0x00, 0x46]);
    assert_eq!(&bytes[3..5], &[0x01, 0x01]); // from.sub_account_index == Some(1)
}

#[test]
fn transfer_unrealized_pnl_borsh_round_trips() {
    for call in [own_account_transfer(), cross_account_transfer()] {
        let bytes = to_vec(&call).expect("serialize transfer");
        assert_eq!(
            RuntimeCall::try_from_slice(&bytes).expect("deserialize transfer"),
            call
        );
    }
}

#[test]
fn transfer_unrealized_pnl_json_round_trips() {
    for call in [own_account_transfer(), cross_account_transfer()] {
        let value = serde_json::to_value(&call).expect("serialize transfer");
        assert_eq!(
            serde_json::from_value::<RuntimeCall>(value).expect("deserialize transfer"),
            call
        );
    }
}

#[test]
fn transfer_unrealized_pnl_json_uses_snake_case_structure() {
    let value = serde_json::to_value(own_account_transfer()).expect("serialize");
    let transfer = &value["exchange"]["user"]["transfer_unrealized_pnl"];

    assert_eq!(transfer["from"]["market_id"], serde_json::json!(7));
    assert!(transfer["from"]["sub_account_index"].is_null());
    assert_eq!(transfer["to"]["sub_account_index"], serde_json::json!(2));
    assert_eq!(transfer["amount"], serde_json::json!("25"));
    // Own-account destination => no address.
    assert!(transfer["to_address"].is_null());

    let value = serde_json::to_value(cross_account_transfer()).expect("serialize");
    let transfer = &value["exchange"]["user"]["transfer_unrealized_pnl"];
    assert_eq!(transfer["to"]["market_id"], serde_json::json!(9));
    assert_eq!(transfer["amount"], serde_json::json!("2.5"));
    // Address serializes as a base58 string in human-readable form.
    assert_eq!(
        transfer["to_address"]
            .as_str()
            .expect("to_address is a string"),
        Address([0x01; 32]).to_string()
    );
}
