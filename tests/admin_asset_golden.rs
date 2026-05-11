#![cfg(feature = "fixtures")]

use borsh::{BorshDeserialize, to_vec};
use bullet_exchange_interface::address::Address;
use bullet_exchange_interface::fixtures::admin_asset_serialization::{
    update_asset_info_v1, update_asset_info_v1_action,
};
use bullet_exchange_interface::message::{AdminAction, UpdateAssetInfoArgsV1};

fn decode_hex(input: &str) -> Vec<u8> {
    hex::decode(input).expect("valid hex fixture")
}

#[test]
fn update_asset_info_v1_borsh_bytes_are_stable() {
    let fixture = update_asset_info_v1();
    let expected = decode_hex("2a00010000000000000000020000000000000001112700000112270000");

    assert_eq!(to_vec(&fixture).expect("serialize fixture"), expected);
    assert_eq!(
        UpdateAssetInfoArgsV1::try_from_slice(&expected).expect("deserialize fixture"),
        fixture
    );
}

#[test]
fn asset_info_v1_admin_action_borsh_bytes_are_stable() {
    let update_fixture = update_asset_info_v1_action();
    let update_expected =
        decode_hex("162a00010000000000000000020000000000000001112700000112270000");

    assert_eq!(to_vec(&update_fixture).expect("serialize fixture"), update_expected);
    assert_eq!(
        AdminAction::<Address>::try_from_slice(&update_expected).expect("deserialize fixture"),
        update_fixture
    );
}
