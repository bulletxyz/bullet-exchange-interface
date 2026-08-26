//! Coverage for the `UserAction::DegenTrade` call message: discriminant
//! stability, borsh round-trip, and snake_case JSON structure/round-trip.

use borsh::{BorshDeserialize, to_vec};
use bullet_exchange_interface::decimals::PositiveDecimal;
use bullet_exchange_interface::message::{CallMessage, DegenAction, UserAction};
use bullet_exchange_interface::transaction::RuntimeCall;
use bullet_exchange_interface::types::{MarketId, Side};

fn degen_open(size: PositiveDecimal, side: Side) -> RuntimeCall {
    RuntimeCall::Exchange(CallMessage::User(UserAction::DegenTrade {
        market_id: MarketId(7),
        action: DegenAction::Open {
            size,
            side,
            amount_to_transfer: PositiveDecimal::from(10u32),
        },
        sub_account_index: None,
    }))
}

fn degen_close() -> RuntimeCall {
    RuntimeCall::Exchange(CallMessage::User(UserAction::DegenTrade {
        market_id: MarketId(7),
        action: DegenAction::Close,
        sub_account_index: Some(2),
    }))
}

#[test]
fn degen_trade_borsh_discriminant_prefix_is_stable() {
    // RuntimeCall::Exchange = 7, CallMessage::User = 0, UserAction::DegenTrade = 31 (0x1F).
    // The market id (u16 LE) follows, then the DegenAction discriminant.
    let bytes =
        to_vec(&degen_open(PositiveDecimal::from(5u32), Side::Bid)).expect("serialize degen open");
    assert_eq!(&bytes[0..3], &[0x07, 0x00, 0x1F]);
    assert_eq!(&bytes[3..5], &[0x07, 0x00]); // MarketId(7)
    assert_eq!(bytes[5], 0x00); // DegenAction::Open

    let bytes = to_vec(&degen_close()).expect("serialize degen close");
    assert_eq!(&bytes[0..3], &[0x07, 0x00, 0x1F]);
    assert_eq!(bytes[5], 0x01); // DegenAction::Close
}

#[test]
fn degen_trade_borsh_round_trips() {
    for call in [
        degen_open(PositiveDecimal::from(5u32), Side::Bid),
        degen_open(
            PositiveDecimal::try_from(rust_decimal::Decimal::new(25, 1)).unwrap(),
            Side::Ask,
        ),
        degen_close(),
    ] {
        let bytes = to_vec(&call).expect("serialize degen trade");
        assert_eq!(
            RuntimeCall::try_from_slice(&bytes).expect("deserialize degen trade"),
            call
        );
    }
}

#[test]
fn degen_trade_json_round_trips() {
    for call in [
        degen_open(PositiveDecimal::from(5u32), Side::Bid),
        degen_open(
            PositiveDecimal::try_from(rust_decimal::Decimal::new(25, 1)).unwrap(),
            Side::Ask,
        ),
        degen_close(),
    ] {
        let value = serde_json::to_value(&call).expect("serialize degen trade");
        assert_eq!(
            serde_json::from_value::<RuntimeCall>(value).expect("deserialize degen trade"),
            call
        );
    }
}

#[test]
fn degen_trade_json_uses_snake_case_structure() {
    let value = serde_json::to_value(degen_open(
        PositiveDecimal::try_from(rust_decimal::Decimal::new(25, 1)).unwrap(),
        Side::Ask,
    ))
    .expect("serialize");
    let degen = &value["exchange"]["user"]["degen_trade"];

    assert_eq!(degen["market_id"], serde_json::json!(7));
    assert!(degen["sub_account_index"].is_null());
    assert_eq!(degen["action"]["open"]["size"], serde_json::json!("2.5"));
    assert_eq!(degen["action"]["open"]["side"], serde_json::json!("ask"));
    assert_eq!(
        degen["action"]["open"]["amount_to_transfer"],
        serde_json::json!("10")
    );

    // The unit `Close` variant renders as a snake_case string.
    let value = serde_json::to_value(degen_close()).expect("serialize");
    let degen = &value["exchange"]["user"]["degen_trade"];
    assert_eq!(degen["action"], serde_json::json!("close"));
    assert_eq!(degen["sub_account_index"], serde_json::json!(2));
}
