//! Argument types for keeper operations.

use crate::decimals::{PositiveDecimal, SurrogateDecimal};
use crate::define_struct;
use crate::types::{AssetId, MarketId};

define_struct! {
    struct OraclePriceUpdateArgs {
        asset_id: AssetId,
        oracle_price: PositiveDecimal,
    }
}

define_struct! {
    struct MarkPriceUpdateArgs {
        market_id: MarketId,
        median_cex_price: PositiveDecimal,
        diff_ema: SurrogateDecimal,
    }
}
