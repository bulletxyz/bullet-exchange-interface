//! Canonical test fixtures owned by this interface crate.

pub mod pyth_proof_serialization {
    //! Deterministic Pyth proof serialization fixtures.
    //!
    //! These byte arrays are not signed Pyth payloads. They are stable interface
    //! serialization inputs for Borsh layout tests; downstream protocol tests own
    //! cryptographic signature acceptance.

    use crate::address::Address;
    use crate::message::{CallMessage, KeeperAction, OraclePriceUpdateWithPythProofArgs};
    use crate::time::UnixTimestampMicros;
    use crate::types::AssetId;

    pub const QUOTE_NONE_ASSET_ID: AssetId = AssetId(7);
    pub const QUOTE_SOME_ASSET_ID: AssetId = AssetId(8);
    pub const PUBLISH_TIMESTAMP: UnixTimestampMicros = UnixTimestampMicros(1_771_339_368_200_000);

    pub const PRIMARY_MESSAGE_WITHOUT_QUOTE: &[u8] = b"pyth-primary-no-quote-v1";
    pub const PRIMARY_MESSAGE_WITH_QUOTE: &[u8] = b"pyth-primary-with-quote-v1";
    pub const QUOTE_MESSAGE: &[u8] = b"pyth-quote-v1";

    pub fn oracle_price_update_without_quote() -> OraclePriceUpdateWithPythProofArgs {
        OraclePriceUpdateWithPythProofArgs {
            asset_id: QUOTE_NONE_ASSET_ID,
            primary_message: PRIMARY_MESSAGE_WITHOUT_QUOTE.to_vec(),
            quote_message: None,
        }
    }

    pub fn oracle_price_update_with_quote() -> OraclePriceUpdateWithPythProofArgs {
        OraclePriceUpdateWithPythProofArgs {
            asset_id: QUOTE_SOME_ASSET_ID,
            primary_message: PRIMARY_MESSAGE_WITH_QUOTE.to_vec(),
            quote_message: Some(QUOTE_MESSAGE.to_vec()),
        }
    }

    pub fn update_oracle_prices_with_pyth_proofs_call() -> CallMessage<Address> {
        CallMessage::Keeper(KeeperAction::UpdateOraclePricesWithPythProofs {
            prices: vec![oracle_price_update_without_quote(), oracle_price_update_with_quote()],
            publish_timestamp: PUBLISH_TIMESTAMP,
        })
    }
}

pub mod admin_asset_serialization {
    //! Deterministic asset admin serialization fixtures.

    use crate::address::Address;
    use crate::decimals::PositiveDecimal;
    use crate::message::{AdminAction, InitAssetInfoArgsV1, UpdateAssetInfoArgsV1};
    use crate::string::CustomString;
    use crate::types::{AssetId, TokenId};

    pub const ASSET_ID: AssetId = AssetId(42);
    pub const PRIMARY_PYTH_LAZER_FEED_ID: u32 = 10_001;
    pub const QUOTE_PYTH_LAZER_FEED_ID: u32 = 10_002;

    pub fn init_asset_info_v1() -> InitAssetInfoArgsV1 {
        InitAssetInfoArgsV1 {
            asset_id: ASSET_ID,
            asset_name: CustomString::from("SOL"),
            token_id: Some(TokenId(CustomString::from("token_SOL"))),
            decimals: 9,
            withdraw_fee: PositiveDecimal::from(1_u8),
            pyth_lazer_feed_id: Some(PRIMARY_PYTH_LAZER_FEED_ID),
            pyth_lazer_quote_feed_id: Some(QUOTE_PYTH_LAZER_FEED_ID),
        }
    }

    pub fn update_asset_info_v1() -> UpdateAssetInfoArgsV1 {
        UpdateAssetInfoArgsV1 {
            asset_id: ASSET_ID,
            withdraw_fee: PositiveDecimal::from(2_u8),
            pyth_lazer_feed_id: Some(PRIMARY_PYTH_LAZER_FEED_ID),
            pyth_lazer_quote_feed_id: Some(QUOTE_PYTH_LAZER_FEED_ID),
        }
    }

    pub fn init_asset_info_v1_action() -> AdminAction<Address> {
        AdminAction::InitAssetInfoV1 { args: init_asset_info_v1() }
    }

    pub fn update_asset_info_v1_action() -> AdminAction<Address> {
        AdminAction::UpdateAssetInfoV1 { args: update_asset_info_v1() }
    }
}
