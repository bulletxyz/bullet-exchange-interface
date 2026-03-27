//! Simple types and enums.

pub use rust_decimal::Decimal;

use crate::string::CustomString;
use crate::{define_enum, define_simple_enum, define_simple_type};

pub const RESERVED_ORDER_ID: OrderId = OrderId(0); // 0 is reserved for OTC (liquidation) orders
pub const RESERVED_TRADE_ID: TradeId = TradeId(0); // 0 is reserved for force settlement of positions
pub const SPOT_MARKET_ID_OFFSET: u16 = 10_000;

define_simple_type!(OrderId(u64));
impl OrderId {
    pub fn reserved() -> Self {
        RESERVED_ORDER_ID
    }

    pub fn first_non_reserved() -> Self {
        Self(RESERVED_ORDER_ID.0 + 1)
    }

    pub fn is_reserved(&self) -> bool {
        self.0 == RESERVED_ORDER_ID.0
    }
}

define_simple_type!(TradeId(u64));
impl TradeId {
    pub fn reserved() -> Self {
        RESERVED_TRADE_ID
    }

    pub fn first_non_reserved() -> Self {
        Self(RESERVED_TRADE_ID.0 + 1)
    }

    pub fn is_reserved(&self) -> bool {
        self.0 == RESERVED_TRADE_ID.0
    }
}

define_simple_type!(TriggerOrderId(u64));
define_simple_type!(TwapId(u64));
define_simple_type!(ClientOrderId(u64));
define_simple_type!(AssetId(u16));
define_simple_type!(MarketId(u16));
impl MarketId {
    pub fn kind(&self) -> MarketKind {
        match self.0 {
            id if id < SPOT_MARKET_ID_OFFSET => MarketKind::Perp,
            _ => MarketKind::Spot,
        }
    }
}

define_simple_enum!(MarketKind{ Perp = 0, Spot = 1 });

define_simple_enum!(Side{ Bid = 0, Ask = 1});
impl Side {
    pub fn reverse(&self) -> Self {
        match self {
            Side::Bid => Side::Ask,
            Side::Ask => Side::Bid,
        }
    }
}

define_simple_enum! {
    #[derive(strum::EnumIter)]
    FeeTier {
        Tier0,
        Tier1,
        Tier2,
        Tier3,
        Tier4
    }
}
#[allow(clippy::derivable_impls)]
impl Default for FeeTier {
    fn default() -> Self {
        Self::Tier0
    }
}
define_simple_enum!(TriggerPriceCondition{Mark = 0, Oracle = 1, LastTrade = 2});
define_simple_enum!(
    #[derive(strum::EnumIter)]
    TriggerDirection{ GreaterThanOrEqual = 0, LessThanOrEqual = 1}
);
define_simple_enum!(OrderType {
    Limit = 0,
    PostOnly = 1,
    FillOrKill = 2,
    ImmediateOrCancel = 3,
    PostOnlySlide = 4, // TODO: Delete this
    PostOnlyFront = 5  // TODO: Delete this
});
define_simple_enum!(SpotCollateralTransferDirection {
    MarginToSpot = 0,
    SpotToMargin = 1
});
define_simple_enum!(AdminType {
    Protocol,
    Funding,
    Pricing,
    FeeTier,
    Credits,
    Referrals
});

define_simple_type!(TokenId(CustomString) + Debug + sov_universal_wallet::UniversalWallet);
impl std::str::FromStr for TokenId {
    type Err = ();
    fn from_str(v: &str) -> Result<Self, Self::Err> {
        Ok(Self(CustomString::from(v)))
    }
}

define_simple_enum!(TradingMode{ Iso = 0, Cross = 1 });

define_simple_enum!(BorrowType {
    /// Internal borrows from trading operations (PnL, margin, etc.)
    Internal = 0,
    /// Unrealized loss borrow (synthetic)
    UnrealizedLoss = 1,
    /// User-initiated borrows with actual fund withdrawal
    External = 2,
});

define_simple_enum!(RepayType {
    /// Borrow repayment from PnL processing
    PnlProcessing,
    /// Borrow repayment from unrealized loss borrow rebalancing
    Rebalance,
    /// Borrow repayment from balance updates in the form of a deposit
    BalanceUpdate,
});

define_simple_enum!(MarketTradingStatus {
    /// It can be actively traded.
    Active = 0,
    /// It cannot be traded, but can be pruned or force settled.
    /// Can go from Halted to Active, if the only action taken is pruning.
    Halted = 1,
    /// If force settling has started, the market enters Cleaning state.
    /// Nothing else can be done here.
    Cleaning = 2,
    /// Once the market is cleaned, it can become active again or it can be safely deleted.
    Cleaned = 3,
});

define_enum!(
    enum TakeFromInsuranceFundReason {
        LiquidateBorrowLendLiability,
        LiquidateIsoPerpPosition(MarketId),
    }
);
