//! Simple types and enums.

pub use rust_decimal::Decimal;

use crate::string::CustomString;
use crate::{define_enum, define_simple_enum, define_simple_type};

pub const RESERVED_ORDER_ID: OrderId = OrderId(0); // 0 is reserved for OTC (liquidation) orders
pub const RESERVED_TRADE_ID: TradeId = TradeId(0); // 0 is reserved for force settlement of positions

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
define_enum! {
    enum OrderIdKind {
        Server(OrderId)  = 0,
        Client(ClientOrderId) = 1,
    }
}
define_simple_type!(AssetId(u16));
define_simple_type!(MarketId(u16));
impl MarketId {
    // Already created edge cases previously, so hard code certain values.
    // Arms stay in ascending range order so the table can be read against the
    // market ID allocations; merging equal-bodied arms would break that.
    #[allow(clippy::match_same_arms)]
    pub fn kind(&self) -> MarketKind {
        use MarketKind::*;
        match self.0 {
            0..10_000 => CryptoPerp,
            10_000..20_000 => Spot,
            20_000..20_003 => RwaPerpCommodities,
            20_003..20_007 => RwaPerpUsEquity, // SPCX, MU, SNDK, TSLA,
            20_007 => RwaPerpKrEquity,         // SKHYNIX
            20_008..21_000 => RwaPerpCommodities,
            21_000..22_000 => RwaPerpUsEquity,
            22_000..23_000 => RwaPerpKrEquity,
            23_000..24_000 => RwaPerpJpEquity,
            24_000..25_000 => RwaPerpHkEquity,
            25_000..26_000 => RwaPerpUsEquityIndices,
            26_000..27_000 => RwaPerpJpEquityIndices,
            27_000..28_000 => RwaPerpHkEquityIndices,
            28_000..29_000 => RwaPerpPreIpo,
            29_000.. => RwaPerpCnEquity,
        }
    }
}

define_simple_enum!(MarketKind{ CryptoPerp = 0, Spot = 1, RwaPerpCommodities = 2, RwaPerpUsEquity = 3, RwaPerpKrEquity = 4, RwaPerpJpEquity = 5, RwaPerpHkEquity = 6, RwaPerpUsEquityIndices = 7, RwaPerpJpEquityIndices = 8, RwaPerpHkEquityIndices = 9, RwaPerpPreIpo = 10, RwaPerpCnEquity = 11 });

impl MarketKind {
    /// True for every real-world-asset perp kind (commodities, equities across
    /// regions, equity indices, pre-IPO).
    ///
    /// The match is exhaustive on purpose: adding a new `MarketKind` variant
    /// fails to compile here until it is explicitly classified as RWA or not,
    /// so the "is this an RWA perp?" question stays correct in one place.
    pub fn is_rwa_perp(&self) -> bool {
        match self {
            MarketKind::CryptoPerp | MarketKind::Spot => false,
            MarketKind::RwaPerpCommodities
            | MarketKind::RwaPerpUsEquity
            | MarketKind::RwaPerpKrEquity
            | MarketKind::RwaPerpJpEquity
            | MarketKind::RwaPerpHkEquity
            | MarketKind::RwaPerpUsEquityIndices
            | MarketKind::RwaPerpJpEquityIndices
            | MarketKind::RwaPerpHkEquityIndices
            | MarketKind::RwaPerpPreIpo
            | MarketKind::RwaPerpCnEquity => true,
        }
    }

    pub fn is_any_perp(&self) -> bool {
        match self {
            MarketKind::CryptoPerp
            | MarketKind::RwaPerpCommodities
            | MarketKind::RwaPerpUsEquity
            | MarketKind::RwaPerpKrEquity
            | MarketKind::RwaPerpJpEquity
            | MarketKind::RwaPerpHkEquity
            | MarketKind::RwaPerpUsEquityIndices
            | MarketKind::RwaPerpJpEquityIndices
            | MarketKind::RwaPerpHkEquityIndices
            | MarketKind::RwaPerpPreIpo
            | MarketKind::RwaPerpCnEquity => true,
            MarketKind::Spot => false,
        }
    }

    pub fn is_crypto_perp(&self) -> bool {
        matches!(self, MarketKind::CryptoPerp)
    }

    pub fn is_spot(&self) -> bool {
        matches!(self, MarketKind::Spot)
    }
}

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
        Tier4,
        Tier5,
        Tier6,
        Tier7,
        Tier8,
        Tier9
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
    PostOnlyFront = 5, // TODO: Delete this
    // System-only marker for the auto-managed ISO liquidation trigger. Users cannot place an order
    // or trigger with this type (rejected by order-placement and `validate_trigger_order_type`); it
    // is set exclusively by the reconcile path and never fed to the matching engine (the liquidation
    // executes via `process_perp_liquidation_order` using an internal IOC order).
    Liquidation = 6
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
    Referrals,
    MarketStatus
});

define_simple_type!(
    #[cfg_attr(feature = "schema", derive(sov_universal_wallet::UniversalWallet))]
    TokenId(CustomString)
        + Debug
);
impl std::str::FromStr for TokenId {
    type Err = ();
    fn from_str(v: &str) -> Result<Self, Self::Err> {
        Ok(Self(CustomString::from(v)))
    }
}

define_simple_enum!(TradingMode{ Iso = 0, Cross = 1 });

define_simple_enum!(TradingCreditsUpdateType {
    /// Credit the user, funded from the USDC PNL pool.
    Add = 0,
    /// Debit the user, capped at their current credit balance.
    Remove = 1
});

define_enum! {
    /// A balance bucket within an account.
    ///
    /// Extensible: new buckets may be added with new discriminants without
    /// breaking existing on-chain encodings.
    #[derive(Copy)]
    #[non_exhaustive]
    #[strum_discriminants(non_exhaustive)]
    enum BalanceBucket {
        /// On-chain bank/wallet balance (outside the exchange).
        Bank = 0,
        /// Cross-margin balance.
        Cross = 1,
        /// Spot collateral balance.
        Spot = 2,
        /// Isolated-margin balance for a specific market.
        Iso(MarketId) = 3,
    }
}

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
    /// Orders can only be posted into the orderbook and not executed, orders can still be cancelled.
    /// Can go from Active/Halted/Cleaned/CancelOnly to PostOnly. Can go from PostOnly to Halted/Active/CancelOnly.
    PostOnly = 4,
    /// Orders can only be cancelled.
    /// Can go from Active/Halted/Cleaned/PostOnly to CancelOnly. Can go from CancelOnly to Halted/Active/PostOnly.
    CancelOnly = 5,
});

define_enum!(
    enum TakeFromInsuranceFundReason {
        LiquidateBorrowLendLiability,
        LiquidateIsoPerpPosition(MarketId),
    }
);

define_enum!(
    enum PythLazerFeeds {
        None,
        Base(u32),
        Feeds { base: u32, quote: u32 },
    }
);

define_simple_enum! {
    #[derive(strum::EnumIter)]
    MarginDiscount {
        None,
        LP,
    }
}
#[allow(clippy::derivable_impls)]
impl Default for MarginDiscount {
    fn default() -> Self {
        Self::None
    }
}
