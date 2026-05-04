use super::{PositiveDecimal, RoundingMode};

pub const FIXED_DECIMALS: u32 = 12;

#[derive(
    Default,
    Debug,
    Clone,
    Copy,
    PartialEq,
    Eq,
    PartialOrd,
    Ord,
    serde::Serialize,
    serde::Deserialize,
    borsh::BorshSerialize,
    borsh::BorshDeserialize,
)]
#[cfg_attr(feature = "schema", derive(sov_universal_wallet::UniversalWallet))]
#[serde(transparent)]
pub struct FixedPositiveDecimal(PositiveDecimal);

impl std::fmt::Display for FixedPositiveDecimal {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl From<PositiveDecimal> for FixedPositiveDecimal {
    fn from(value: PositiveDecimal) -> Self {
        Self::new(value, RoundingMode::Down)
    }
}

impl From<FixedPositiveDecimal> for PositiveDecimal {
    fn from(value: FixedPositiveDecimal) -> PositiveDecimal {
        value.0
    }
}

impl FixedPositiveDecimal {
    pub const ZERO: FixedPositiveDecimal = FixedPositiveDecimal(PositiveDecimal::ZERO);

    #[inline]
    pub fn new(value: PositiveDecimal, mode: RoundingMode) -> Self {
        let rounded = match mode {
            RoundingMode::Up => value.as_dec().round_dp_with_strategy(
                FIXED_DECIMALS,
                rust_decimal::RoundingStrategy::AwayFromZero,
            ),
            RoundingMode::Down => value
                .as_dec()
                .round_dp_with_strategy(FIXED_DECIMALS, rust_decimal::RoundingStrategy::ToZero),
        };

        // SAFETY: Rounding down a PositiveDecimal (non-negative) with RoundingMode::Down
        // always produces a valid FixedPositiveDecimal. The only way FixedPositiveDecimal::new
        // can fail is if PositiveDecimal::new returns None, which only happens for negative
        // values. Since we're rounding down a non-negative value, the result is always
        // non-negative.
        #[allow(
            clippy::expect_used,
            reason = "Rounding down a PositiveDecimal is infallible - result is always non-negative"
        )]
        Self(
            PositiveDecimal::new(rounded).expect(
                "rounding down a PositiveDecimal always produces valid FixedPositiveDecimal",
            ),
        )
    }

    #[inline(always)]
    pub fn is_zero(&self) -> bool {
        self.0.is_zero()
    }

    #[inline(always)]
    pub fn as_pos_dec(&self) -> PositiveDecimal {
        self.0
    }
}
