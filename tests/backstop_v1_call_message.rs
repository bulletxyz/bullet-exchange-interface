use borsh::{BorshDeserialize, to_vec};
use bullet_exchange_interface::address::Address;
use bullet_exchange_interface::decimals::PositiveDecimal;
use bullet_exchange_interface::message::{
    BackstopLiquidatePerpPositionArgs, BackstopLiquidatePerpPositionArgsV1, UserAction,
};
use bullet_exchange_interface::types::{MarketId, Side};

fn priced_leg(side: Side) -> BackstopLiquidatePerpPositionArgsV1 {
    BackstopLiquidatePerpPositionArgsV1 {
        market_id: MarketId(7),
        size: PositiveDecimal::from(2u8),
        liquidator_side: side,
        takeover_price: PositiveDecimal::from(99u8),
    }
}

#[test]
#[allow(deprecated)]
fn v0_backstop_bytes_remain_stable() {
    let address = Address([1; 32]);
    let cross = UserAction::BackstopLiquidatePerpPositions {
        address,
        positions: Some(vec![BackstopLiquidatePerpPositionArgs {
            market_id: MarketId(7),
            size: PositiveDecimal::from(2u8),
        }]),
        sub_account_index: Some(3),
    };
    let iso = UserAction::BackstopLiquidateIsoPerpPosition {
        address,
        position: BackstopLiquidatePerpPositionArgs {
            market_id: MarketId(7),
            size: PositiveDecimal::from(2u8),
        },
        sub_account_index: None,
    };

    let mut cross_expected = vec![60];
    cross_expected.extend([1; 32]);
    cross_expected.extend([1, 1, 0, 0, 0, 7, 0]);
    cross_expected.extend([0, 0, 0, 0, 0, 0, 0, 0, 2, 0, 0, 0, 0, 0, 0, 0]);
    cross_expected.extend([1, 3]);
    assert_eq!(to_vec(&cross).unwrap(), cross_expected);

    let mut iso_expected = vec![62];
    iso_expected.extend([1; 32]);
    iso_expected.extend([7, 0]);
    iso_expected.extend([0, 0, 0, 0, 0, 0, 0, 0, 2, 0, 0, 0, 0, 0, 0, 0]);
    iso_expected.push(0);
    assert_eq!(to_vec(&iso).unwrap(), iso_expected);

    for (positions, mut expected_tail) in [(None, vec![0]), (Some(vec![]), vec![1, 0, 0, 0, 0])] {
        let action = UserAction::BackstopLiquidatePerpPositions {
            address,
            positions,
            sub_account_index: None,
        };
        let mut expected = vec![60];
        expected.extend([1; 32]);
        expected.append(&mut expected_tail);
        expected.push(0);
        assert_eq!(to_vec(&action).unwrap(), expected);
        assert_eq!(UserAction::try_from_slice(&expected).unwrap(), action);
    }
}

#[test]
fn v1_action_has_pinned_bytes_and_round_trips() {
    let address = Address([2; 32]);
    let cross = UserAction::BackstopLiquidatePerpPositionsV1 {
        address,
        positions: vec![priced_leg(Side::Bid)],
        sub_account_index: Some(4),
    };

    let mut cross_expected = vec![63];
    cross_expected.extend([2; 32]);
    cross_expected.extend([1, 0, 0, 0, 7, 0]);
    cross_expected.extend([0, 0, 0, 0, 0, 0, 0, 0, 2, 0, 0, 0, 0, 0, 0, 0]);
    cross_expected.push(0);
    cross_expected.extend([0, 0, 0, 0, 0, 0, 0, 0, 99, 0, 0, 0, 0, 0, 0, 0]);
    cross_expected.extend([1, 4]);
    assert_eq!(to_vec(&cross).unwrap(), cross_expected);
    assert_eq!(UserAction::try_from_slice(&cross_expected).unwrap(), cross);
}

#[test]
fn v1_json_and_schema_expose_the_additive_contract() {
    let address = Address([3; 32]);
    let action = UserAction::BackstopLiquidatePerpPositionsV1 {
        address,
        positions: vec![priced_leg(Side::Bid)],
        sub_account_index: None,
    };
    let value = serde_json::to_value(&action).unwrap();
    let payload = &value["backstop_liquidate_perp_positions_v1"];
    assert_eq!(payload["positions"][0]["market_id"], 7);
    assert_eq!(payload["positions"][0]["liquidator_side"], "bid");
    assert_eq!(payload["positions"][0]["takeover_price"], "99");
    assert_eq!(
        serde_json::from_value::<UserAction<Address>>(value).unwrap(),
        action
    );

    let schema = serde_json::to_string(&schemars::schema_for!(UserAction<Address>)).unwrap();
    for expected in [
        "backstop_liquidate_perp_positions_v1",
        "liquidator_side",
        "takeover_price",
    ] {
        assert!(schema.contains(expected), "schema omitted {expected}");
    }
    assert!(!schema.contains("backstop_liquidate_iso_perp_position_v1"));
}
