#![cfg(feature = "fixtures")]

use borsh::{BorshDeserialize, to_vec};
use bullet_exchange_interface::address::Address;
use bullet_exchange_interface::fixtures::admin_asset_serialization::{
    init_asset_info_v1, init_asset_info_v1_action, update_asset_info_v1,
    update_asset_info_v1_action,
};
use bullet_exchange_interface::message::{AdminAction, InitAssetInfoArgsV1, UpdateAssetInfoArgsV1};

fn decode_hex(input: &str) -> Vec<u8> {
    hex::decode(input).expect("valid hex fixture")
}

#[test]
fn init_asset_info_v1_borsh_bytes_are_stable() {
    let fixture = init_asset_info_v1();
    let expected = decode_hex(
        "2a0003000000534f4c0109000000746f6b656e5f534f4c090000000000000000010000000000000001112700000112270000",
    );

    assert_eq!(to_vec(&fixture).expect("serialize fixture"), expected);
    assert_eq!(
        InitAssetInfoArgsV1::try_from_slice(&expected).expect("deserialize fixture"),
        fixture
    );
}

#[test]
fn update_asset_info_v1_borsh_bytes_are_stable() {
    let fixture = update_asset_info_v1();
    let expected = decode_hex("2a000000000000000000020000000000000001112700000112270000");

    assert_eq!(to_vec(&fixture).expect("serialize fixture"), expected);
    assert_eq!(
        UpdateAssetInfoArgsV1::try_from_slice(&expected).expect("deserialize fixture"),
        fixture
    );
}

#[test]
fn asset_info_v1_admin_action_borsh_bytes_are_stable() {
    let init_fixture = init_asset_info_v1_action();
    let init_expected = decode_hex(
        "162a0003000000534f4c0109000000746f6b656e5f534f4c090000000000000000010000000000000001112700000112270000",
    );

    assert_eq!(to_vec(&init_fixture).expect("serialize fixture"), init_expected);
    assert_eq!(
        AdminAction::<Address>::try_from_slice(&init_expected).expect("deserialize fixture"),
        init_fixture
    );

    let update_fixture = update_asset_info_v1_action();
    let update_expected = decode_hex("172a000000000000000000020000000000000001112700000112270000");

    assert_eq!(to_vec(&update_fixture).expect("serialize fixture"), update_expected);
    assert_eq!(
        AdminAction::<Address>::try_from_slice(&update_expected).expect("deserialize fixture"),
        update_fixture
    );
}
