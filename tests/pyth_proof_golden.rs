#![cfg(feature = "fixtures")]

use borsh::{BorshDeserialize, to_vec};
use bullet_exchange_interface::address::Address;
use bullet_exchange_interface::fixtures::pyth_proof_serialization::{
    oracle_price_update_with_quote, oracle_price_update_without_quote,
    update_oracle_prices_with_pyth_proofs_call,
};
use bullet_exchange_interface::message::{CallMessage, OraclePriceUpdateWithPythProofArgs};

fn decode_hex(input: &str) -> Vec<u8> {
    hex::decode(input).expect("valid hex fixture")
}

#[test]
fn oracle_price_update_with_pyth_proof_without_quote_borsh_bytes_are_stable() {
    let fixture = oracle_price_update_without_quote();
    let expected = decode_hex("070018000000707974682d7072696d6172792d6e6f2d71756f74652d763100");

    assert_eq!(to_vec(&fixture).expect("serialize fixture"), expected);
    assert_eq!(
        OraclePriceUpdateWithPythProofArgs::try_from_slice(&expected).expect("deserialize fixture"),
        fixture
    );
}

#[test]
fn oracle_price_update_with_pyth_proof_with_quote_borsh_bytes_are_stable() {
    let fixture = oracle_price_update_with_quote();
    let expected = decode_hex(
        "08001a000000707974682d7072696d6172792d776974682d71756f74652d7631010d000000707974682d71756f74652d7631",
    );

    assert_eq!(to_vec(&fixture).expect("serialize fixture"), expected);
    assert_eq!(
        OraclePriceUpdateWithPythProofArgs::try_from_slice(&expected).expect("deserialize fixture"),
        fixture
    );
}

#[test]
fn update_oracle_prices_with_pyth_proofs_call_borsh_bytes_are_stable() {
    let fixture = update_oracle_prices_with_pyth_proofs_call();
    let expected = decode_hex(
        "020302000000070018000000707974682d7072696d6172792d6e6f2d71756f74652d76310008001a000000707974682d7072696d6172792d776974682d71756f74652d7631010d000000707974682d71756f74652d76314077d115064b0600",
    );

    assert_eq!(to_vec(&fixture).expect("serialize fixture"), expected);
    assert_eq!(
        CallMessage::<Address>::try_from_slice(&expected).expect("deserialize fixture"),
        fixture
    );
}
