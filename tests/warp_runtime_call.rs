use borsh::{BorshDeserialize, to_vec};
use bullet_exchange_interface::address::Address;
use bullet_exchange_interface::message::{CallMessage, UserAction};
use bullet_exchange_interface::string::CustomString;
use bullet_exchange_interface::transaction::bank::{Coins, TokenId};
use bullet_exchange_interface::transaction::{
    Amount, BankCall, RuntimeCall, WarpBytes32, WarpCall,
};
use serde_json::json;

fn bytes(value: u8) -> [u8; 32] {
    [value; 32]
}

fn warp_bytes(value: u8) -> WarpBytes32 {
    WarpBytes32(bytes(value))
}

fn decode_hex(input: &str) -> Vec<u8> {
    hex::decode(input).expect("valid hex fixture")
}

fn hex_32_json(byte: u8) -> String {
    format!("0x{}", format!("{byte:02x}").repeat(32))
}

fn transfer_remote_call() -> RuntimeCall {
    RuntimeCall::Warp(WarpCall::TransferRemote {
        warp_route: warp_bytes(0x11),
        destination_domain: 1234,
        recipient: warp_bytes(0x22),
        amount: Amount(123_456),
        relayer: None,
        gas_payment_limit: Amount(400_000),
    })
}

#[test]
fn warp_transfer_remote_serializes_to_expected_json() {
    let call = transfer_remote_call();
    let expected = json!({
        "warp": {
            "transfer_remote": {
                "warp_route": hex_32_json(0x11),
                "destination_domain": 1234,
                "recipient": hex_32_json(0x22),
                "amount": "123456",
                "relayer": null,
                "gas_payment_limit": "400000",
            }
        }
    });

    assert_eq!(serde_json::to_value(&call).expect("serialize runtime call"), expected);
    assert_eq!(
        serde_json::from_value::<RuntimeCall>(expected).expect("deserialize runtime call"),
        call
    );
}

#[test]
fn warp_transfer_remote_rejects_numeric_json_amounts() {
    let invalid = json!({
        "warp": {
            "transfer_remote": {
                "warp_route": hex_32_json(0x11),
                "destination_domain": 1234,
                "recipient": hex_32_json(0x22),
                "amount": 123456,
                "relayer": null,
                "gas_payment_limit": "400000",
            }
        }
    });

    assert!(serde_json::from_value::<RuntimeCall>(invalid).is_err());
}

#[test]
fn warp_transfer_remote_borsh_bytes_match_runtime_encoding() {
    let call = transfer_remote_call();
    let expected = decode_hex(&format!(
        "0f04{}d2040000{}40e2010000000000000000000000000000801a0600000000000000000000000000",
        "11".repeat(32),
        "22".repeat(32),
    ));

    assert_eq!(to_vec(&call).expect("serialize runtime call"), expected);
    assert_eq!(RuntimeCall::try_from_slice(&expected).expect("deserialize runtime call"), call);
}

#[test]
fn adding_warp_does_not_change_exchange_or_bank_runtime_call_encoding() {
    let exchange =
        RuntimeCall::Exchange(CallMessage::User(UserAction::CreateSubAccount { index: 42 }));
    assert_eq!(to_vec(&exchange).expect("serialize exchange call"), decode_hex("0700062a"));

    let bank = RuntimeCall::Bank(BankCall::TransferWithMemo {
        to: Address(bytes(0x01)),
        coins: Coins { amount: Amount(99), token_id: TokenId(bytes(0x02)) },
        memo: CustomString::from("memo"),
    });
    let bank_expected = decode_hex(&format!(
        "0206{}63000000000000000000000000000000{}040000006d656d6f",
        "01".repeat(32),
        "02".repeat(32),
    ));
    assert_eq!(to_vec(&bank).expect("serialize bank call"), bank_expected);
}

#[cfg(feature = "schema")]
#[test]
fn warp_transfer_remote_is_present_in_generated_universal_wallet_schema() {
    use bullet_exchange_interface::schema::Schema;
    use bullet_exchange_interface::transaction::Transaction;

    let schema = Schema::of_single_type::<Transaction>().expect("generate transaction schema");
    let schema_json = serde_json::to_string(&schema).expect("serialize schema");

    assert!(schema_json.contains("\"Warp\""));
    assert!(schema_json.contains("\"TransferRemote\""));
    assert!(schema_json.contains("\"warp_route\""));
    assert!(schema_json.contains("\"destination_domain\""));
    assert!(schema_json.contains("\"recipient\""));
    assert!(schema_json.contains("\"amount\""));
    assert!(schema_json.contains("\"relayer\""));
    assert!(schema_json.contains("\"gas_payment_limit\""));
    assert!(schema_json.contains("\"Hex\""));
}
